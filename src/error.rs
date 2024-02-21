#[derive(Debug, Clone, PartialEq)]
pub struct Error {
    pub line: i64,
    location: String,
    message: String,
}

impl Error {
    pub fn new(line: i64, location: &str, message: &str) -> Self {
        Self {
            line,
            location: location.to_string(),
            message: message.to_string(),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[line {}] Error{}: {}", self.line, self.location, self.message)
    }
}
