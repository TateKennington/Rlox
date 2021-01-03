use crate::lox::tokens::Token;
use std::fmt;

pub enum Expr {
    Binary {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>,
    },
    Grouping(Box<Expr>),
    Literal(Token),
    Unary {
        right: Box<Expr>,
        op: Token,
    },
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Grouping(expr) => write!(f, "(group {})", expr),
            Expr::Binary { left, right, op } => {
                write!(f, "({} {} {})", op.lexeme, left, right)
            }
            Expr::Literal(literal) => write!(f, "{}", literal.lexeme),
            Expr::Unary { right, op } => write!(f, "({} {})", op.lexeme, right),
        }
    }
}