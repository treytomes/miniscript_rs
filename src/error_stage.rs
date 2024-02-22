use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ErrorStage {
    Compile,
    Runtime,
}

impl Display for ErrorStage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ErrorStage::Compile => write!(f, "Compile"),
            ErrorStage::Runtime => write!(f, "Runtime"),
        }
    }
}
