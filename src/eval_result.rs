use std::fmt::{Display, Formatter};

use crate::Error;

#[derive(Debug, PartialEq, Clone)]
pub enum EvalResult {
    Null,
    Number(f64),
    String(String),
    Error(Error),
}

impl Display for EvalResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EvalResult::Null => write!(f, "null"),
            EvalResult::Number(n) => write!(f, "{}", n),
            EvalResult::String(s) => write!(f, "{}", s),
            EvalResult::Error(e) => write!(f, "{}", e),
        }
    }
}
