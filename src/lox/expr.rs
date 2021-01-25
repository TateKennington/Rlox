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
    Var(Token),
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
            Expr::Var(token) => write!(f, "{}", token),
        }
    }
}

impl Expr {
    pub fn print_rpn(&self) -> String {
        match self {
            Expr::Grouping(expr) => format!("{}", expr.print_rpn()),
            Expr::Binary { left, right, op } => {
                format!("{} {} {}", left.print_rpn(), right.print_rpn(), op.lexeme)
            }
            Expr::Literal(literal) => format!("{}", literal.lexeme),
            Expr::Unary { right, op } => format!("{} {}", right.print_rpn(), op.lexeme),
            _ => panic!("Unsupported expression"),
        }
    }
}
