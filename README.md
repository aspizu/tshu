# shl

```py
from shl import sh
await sh("uv add shl")
```

Run safe and cross-platform bash commands using Python 3.14's t-strings

Uses [brush](https://github.com/reubeno/brush) for a cross-platform bash implementation.

## Installation

```shell
uv add shl
```

## Usage

```py
from shl import sh
```

### Run a bash command.

Always use a t-string, regular strings are not allowed to prevent accidental usage of
f-strings.

```py
await sh(t"cat /usr/share/dict/words | wc -l")
```

### Run a bash command with user input

You still need to quote substitutions to prevent word-splitting. Word-splitting is a
feature, not a vuln.

Substitutions use variables to prevent shell injection. These temporary variables
are not accessible by child programs as they are not exported and have random names.

```py
await sh(t'cat "{__file__}" | wc -l')
```

### Exit code and check

By default, awaiting a command returns the exit code. [shl.CommandError][] is raised
when a command returns a non-zero exit code. To disable this behaviour, either
pass `check=False` to `sh` or set `sh.check = False` to disable checking globally.

### Suppress command output

Either pass `quiet=True` or set `sh.quiet = True` to suppress output globally.

### Change working directory

Either pass `cwd=...` or set `sh.cwd = "..."`. You can use `pathlib.Path`.

### Set environment variables

Either modify `os.environ` or pass `env={}`. These are exported, so accessible to
child programs, use substitutions to pass user input.

### Pass standard input

input can be string or bytes.

```py
assert await sh("wc -l", input="1\n2\n3\n").json() == 3
```

### Capture the stdout, stderr of a program

```py
result = await sh("help").output()
result.returncode
result.stdout
result.stderr
```

### Get standard output directly

Use `.text()` to directly get the output as string (utf-8 encoded).

```py
contents = await sh("cat file.txt").text()
```

### Get standard output directly as bytes

```py
contents = await sh("cat file.bin").bytes()
```

### Get standard output parsed as JSON

```py
data = await sh("cat file.json").json()
```
