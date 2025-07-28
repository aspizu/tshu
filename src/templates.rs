use std::collections::HashMap;

use pyo3::prelude::*;
use rand::prelude::*;

pub struct ParsedTemplate {
    pub text: String,
    pub interpolations: HashMap<String, String>,
}

pub fn parse_template(template: Bound<'_, PyAny>) -> PyResult<ParsedTemplate> {
    let mut rng = rand::rng();
    let mut text = String::new();
    let mut interpolations = HashMap::new();
    for part in template.try_iter()? {
        let part = part?;
        if let Ok(partstr) = part.extract::<&str>() {
            text.push_str(partstr);
        } else {
            let partstr =
                part.getattr("value")?.call_method1("__str__", ())?.extract::<String>()?;
            let varname = format!("PYOBJECT{}", rng.random::<u64>());
            text.push_str("$");
            text.push_str(&varname);
            interpolations.insert(varname, partstr);
        }
    }
    Ok(ParsedTemplate { text, interpolations })
}
