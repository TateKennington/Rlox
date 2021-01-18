use crate::lox::expr::Expr;
use crate::lox::tokens::{Token, TokenType};
use std::fmt;

pub enum Value {
    Number(f32),
    String(String),
    Boolean(bool),
    Nil,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Number(value) => write!(f, "{}", value),
            Value::String(value) => write!(f, "{}", value),
            Value::Boolean(value) => write!(f, "{}", value),
            Value::Nil => write!(f, "Nil"),
        }
    }
}

impl Expr {
    pub fn interpret(&self) -> Value {
        match self {
            Expr::Binary { left, right, op } => Expr::interpret_binary(left, right, op),
            Expr::Unary { right, op } => Expr::interpret_unary(right, op),
            Expr::Grouping(expr) => expr.interpret(),
            Expr::Literal(token) => Expr::interpret_literal(token),
        }
    }

    fn interpret_unary(right: &Expr, op: &Token) -> Value {
        let right_val = right.interpret();

        match op.token_type {
            TokenType::Minus => Value::Number(-1.0 * Expr::get_number(right_val)),
            TokenType::Bang => panic!("Not Implement: Unary Bang"),
            _ => panic!("Error: Expected Unary Operation"),
        }
    }

    fn interpret_literal(token: &Token) -> Value {
        match &token.token_type {
            TokenType::Number(value) => Value::Number(*value),
            TokenType::String(value) => Value::String(value.clone()),
            TokenType::True => Value::Boolean(true),
            TokenType::False => Value::Boolean(false),
            TokenType::Nil => Value::Nil,
            _ => panic!("Error: Expected Literal"),
        }
    }

    fn interpret_binary(left: &Expr, right: &Expr, op: &Token) -> Value {
        let left_val = left.interpret();
        let right_val = right.interpret();

        match op.token_type {
            //Arithmetic operations
            TokenType::Plus => {
                Value::Number(Expr::get_number(left_val) + Expr::get_number(right_val))
            }
            TokenType::Star => {
                Value::Number(Expr::get_number(left_val) * Expr::get_number(right_val))
            }
            TokenType::Minus => {
                Value::Number(Expr::get_number(left_val) - Expr::get_number(right_val))
            }
            TokenType::Slash => {
                Value::Number(Expr::get_number(left_val) / Expr::get_number(right_val))
            }
            //Comparisons
            TokenType::Less => panic!("Not Implemented: Binary Less"),
            TokenType::LessEqual => panic!("Not Implemented: Binary LessEqual"),
            TokenType::Greater => panic!("Not Implemented: Binary Greater"),
            TokenType::GreaterEqual => panic!("Not Implemented: Binary GreaterEqual"),
            //Equality
            TokenType::EqualEqual => panic!("Not Implemented: Binary EqualEqual"),
            TokenType::BangEqual => panic!("Not Implemented: Binary BangEqual"),
            _ => panic!("Error: Expected Binary Operation"),
        }
    }

    fn get_number(value: Value) -> f32 {
        if let Value::Number(result) = value {
            return result;
        }
        panic!("Error: Expected Number")
    }
}
