use crate::error_stage::ErrorStage;

#[derive(Debug, Clone, PartialEq)]
pub struct Error {
    pub line: i64,
    location: String,
    message: String,
    pub stage: ErrorStage,
}

impl Error {
    pub fn new(line: i64, location: &str, message: &str, stage: ErrorStage) -> Self {
        Self {
            line,
            location: location.to_string(),
            message: message.to_string(),
            stage,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[line {}] {} Error{}: {}", self.line, self.stage, self.location, self.message)
    }
}
