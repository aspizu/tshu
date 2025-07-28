pub enum Enc {
    RETURNCODE,
    OUTPUT,
    TEXT,
    BYTES,
    JSON,
    YAML,
    TOML,
}

impl From<usize> for Enc {
    fn from(value: usize) -> Self {
        match value {
            1 => Enc::RETURNCODE,
            2 => Enc::OUTPUT,
            3 => Enc::TEXT,
            4 => Enc::BYTES,
            5 => Enc::JSON,
            6 => Enc::YAML,
            7 => Enc::TOML,
            _ => Enc::RETURNCODE,
        }
    }
}
