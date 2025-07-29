use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Seek, Write},
};

use pyo3::prelude::*;
use tempfile::tempfile;
use tokio::io;

use crate::{
    enc::Enc,
    templates::{ParsedTemplate, parse_template},
};

pub fn anyio<T>(result: Result<T, impl Into<anyhow::Error>>) -> io::Result<T> {
    result.map_err(|err| io::Error::new(io::ErrorKind::Other, err.into()))
}

// TODO: Shell is clone()-able, only initialize shell once, then clone on each
// invocation.
async fn load_shell() -> PyResult<brush_core::Shell> {
    let mut options = brush_core::CreateOptions::default();
    options.posix = false;
    options.sh_mode = false;
    options.no_profile = true;
    options.login = false;
    options.no_rc = true;
    options.interactive = false;
    Ok(anyio(brush_core::Shell::new(&options).await)?)
}

fn set_input(command: &Bound<'_, PyAny>, stdin: &mut File) -> PyResult<()> {
    let input = command.getattr("_input")?;
    if let Ok(str) = input.extract::<&str>() {
        stdin.write_all(str.as_bytes())?;
        stdin.seek(io::SeekFrom::Start(0))?;
    } else if let Ok(bytes) = input.extract::<&[u8]>() {
        stdin.write_all(bytes)?;
        stdin.seek(io::SeekFrom::Start(0))?;
    } else {
        todo!("Raise python exception here.")
    }
    Ok(())
}

fn set_cwd(command: &Bound<'_, PyAny>, shell: &mut brush_core::Shell) -> PyResult<()> {
    let cwd = command.getattr("_cwd")?;
    let Ok(cwd) = cwd.extract::<&str>() else { return Ok(()) };
    anyio(shell.set_working_dir(cwd))?;
    Ok(())
}

fn set_env(command: &Bound<'_, PyAny>, shell: &mut brush_core::Shell) -> PyResult<()> {
    let env = command.getattr("_env")?;
    let Ok(env) = env.extract::<HashMap<String, String>>() else { return Ok(()) };
    for (key, value) in env {
        let mut variable = brush_core::ShellVariable::new(value);
        variable.export();
        anyio(shell.set_env_global(&key, variable))?;
    }
    Ok(())
}

fn set_template_vars(shell: &mut brush_core::Shell, template: &ParsedTemplate) -> PyResult<()> {
    for (key, value) in &template.interpolations {
        let variable = brush_core::ShellVariable::new(value);
        anyio(shell.set_env_global(&key, variable))?;
    }
    Ok(())
}

fn encode_json(py: Python<'_>, mut stdout: File) -> PyResult<Py<PyAny>> {
    let json = py.import("json")?;
    let loads = json.getattr("loads")?;
    stdout.seek(io::SeekFrom::Start(0))?;
    let mut bstdout = String::new();
    stdout.read_to_string(&mut bstdout)?;
    Ok(loads.call1((bstdout.into_pyobject(py)?,))?.into_any().unbind())
}

fn encode_bytes(py: Python<'_>, mut stdout: File) -> PyResult<Py<PyAny>> {
    stdout.seek(io::SeekFrom::Start(0))?;
    let mut bstdout = vec![];
    stdout.read_to_end(&mut bstdout)?;
    Ok(bstdout.into_pyobject(py)?.into_any().unbind())
}

fn encode_text(py: Python<'_>, mut stdout: File) -> PyResult<Py<PyAny>> {
    stdout.seek(io::SeekFrom::Start(0))?;
    let mut bstdout = String::new();
    stdout.read_to_string(&mut bstdout)?;
    Ok(bstdout.into_pyobject(py)?.into_any().unbind())
}

fn encode_output(
    py: Python<'_>,
    returncode: u8,
    mut stdout: File,
    mut stderr: File,
) -> Result<Py<PyAny>, PyErr> {
    let tshu = py.import("tshu")?;
    let completed_command_t = tshu.getattr("CompletedCommand")?;
    stdout.seek(io::SeekFrom::Start(0))?;
    stderr.seek(io::SeekFrom::Start(0))?;
    let mut bstdout = vec![];
    let mut bstderr = vec![];
    stdout.read_to_end(&mut bstdout)?;
    stderr.read_to_end(&mut bstderr)?;
    let completed_command = completed_command_t.call1((
        returncode.into_pyobject(py)?,
        bstdout.into_pyobject(py)?,
        bstderr.into_pyobject(py)?,
    ))?;
    Ok(completed_command.into_any().unbind())
}

async fn execute_command_future(
    quiet: bool,
    check: bool,
    enc: Enc,
    input: bool,
    template: ParsedTemplate,
    command: Py<PyAny>,
) -> PyResult<Py<PyAny>> {
    let mut shell = load_shell().await?;
    let mut stdin = tempfile()?;
    let stdout = tempfile()?;
    let stderr = tempfile()?;
    Python::with_gil(|py| -> PyResult<()> {
        let command = command.bind(py);
        if input {
            set_input(command, &mut stdin)?;
        }
        set_cwd(command, &mut shell)?;
        set_env(command, &mut shell)?;
        Ok(())
    })?;
    set_template_vars(&mut shell, &template)?;
    if input {
        shell.open_files.set(0, stdin.into());
    }
    if quiet {
        shell.open_files.set(1, brush_core::OpenFile::Null);
        shell.open_files.set(2, brush_core::OpenFile::Null);
    }
    if matches!(enc, Enc::OUTPUT | Enc::TEXT | Enc::BYTES | Enc::JSON) {
        shell.open_files.set(1, stdout.try_clone()?.into());
    }
    if matches!(enc, Enc::OUTPUT) {
        shell.open_files.set(2, stderr.try_clone()?.into());
    }
    let result = anyio(shell.run_string(template.text, &shell.default_exec_params()).await)?;
    if check && result.exit_code != 0 {
        todo!("Raise python exception here.");
    }
    Python::with_gil(|py| match enc {
        Enc::RETURNCODE => Ok(result.exit_code.into_pyobject(py)?.into_any().unbind()),
        Enc::OUTPUT => encode_output(py, result.exit_code, stdout, stderr),
        Enc::TEXT => encode_text(py, stdout),
        Enc::BYTES => encode_bytes(py, stdout),
        Enc::JSON => encode_json(py, stdout),
        _ => todo!(),
    })
}

#[pyfunction]
pub fn _execute_command<'py>(
    py: Python<'py>,
    command: Bound<'py, PyAny>,
) -> PyResult<Bound<'py, PyAny>> {
    let quiet = command.getattr("_quiet")?.extract::<bool>()?;
    let check = command.getattr("_check")?.extract::<bool>()?;
    let enc = Enc::from(command.getattr("_enc")?.extract::<usize>()?);
    let input = !command.getattr("_input")?.is_none();
    let template = parse_template(command.getattr("_template")?)?;
    let command = command.unbind();
    let fut = execute_command_future(quiet, check, enc, input, template, command);
    pyo3_async_runtimes::tokio::future_into_py(py, fut)
}
