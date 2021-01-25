use crate::lox::{expr::Expr, tokens::Token};
use std::fmt;

pub enum Stmt {
    Expression(Box<Expr>),
    Print(Box<Expr>),
    Var(Token),
    InitialisedVar(Token, Box<Expr>),
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Stmt::Expression(expr) => write!(f, "Expr {}", expr),
            Stmt::Print(expr) => write!(f, "Print {}", expr),
            Stmt::Var(token) => write!(f, "Var {}", token),
            Stmt::InitialisedVar(token, expr) => write!(f, "Init Var {} {}", token, expr),
            _ => panic!("Unsupported Statement"),
        }
    }
}

impl Stmt {
    pub fn interpret(&self) {
        match self {
            Stmt::Expression(expr) => {
                expr.interpret();
            }
            Stmt::Print(expr) => {
                println!("{}", expr.interpret());
            }
            _ => panic!("Unsupported Expression"),
        };
    }
}
