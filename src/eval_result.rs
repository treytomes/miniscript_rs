use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum EvalResult {
    Number(f64),
    String(String),
}

impl Display for EvalResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EvalResult::Number(n) => write!(f, "{}", n),
            EvalResult::String(s) => write!(f, "{}", s),
        }
    }
}
