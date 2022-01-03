use std::error::Error;

#[derive(Debug)]
pub struct LogError(String);

impl Error for LogError {
    fn description(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for LogError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        f.write_str(&self.0)
    }
}

impl From<String> for LogError {
    fn from(err_msg: String) -> Self {
        LogError(err_msg)
    }
}

impl From<serde_json::Error> for LogError {
    fn from(err_msg: serde_json::Error) -> Self {
        LogError(format!("{}", err_msg))
    }
}
