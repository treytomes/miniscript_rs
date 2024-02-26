use std::fmt::{Debug, Display};

use crate::Expr;

#[derive(Clone, PartialEq)]
pub enum Stmt {
    Expression(Expr),
    Print(Expr),
    // Assignment(String, Expr),
}

// impl Stmt {
//     pub fn line(&self) -> i64 {
//         match self {
//             Stmt::Expression(expr) => expr.line(),
//             Stmt::Print(expr) => expr.line(),
//         }
//     }
// }

impl Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stmt::Expression(expr) => write!(f, "{}", expr),
            Stmt::Print(expr) => write!(f, "print {}", expr),
            // Stmt::Assignment(name, expr) => write!(f, "{} = {}", name, expr),
        }
    }
}

impl Debug for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Stmt::Expression(expr) => write!(f, "Expression({})", expr),
            Stmt::Print(expr) => write!(f, "Print({})", expr),
            // Stmt::Assignment(name, expr) => write!(f, "Assignment({}, {})", name, expr),
        }
    }
}
