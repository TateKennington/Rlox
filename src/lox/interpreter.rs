use crate::lox::environment::Environment;
use crate::lox::expr::Expr;
use crate::lox::tokens::{Token, TokenType};
use std::fmt;
use std::rc::Rc;

#[derive(Clone)]
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

impl fmt::Debug for Value {
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
    pub fn interpret(&self, environment: &mut Box<Environment>) -> Value {
        match self {
            Expr::Binary { left, right, op } => {
                Expr::interpret_binary(left, right, op, environment)
            }
            Expr::Unary { right, op } => Expr::interpret_unary(right, op, environment),
            Expr::Grouping(expr) => expr.interpret(environment),
            Expr::Literal(token) => Expr::interpret_literal(token, environment),
            Expr::Var(token) => {
                println!("Var Expression");
                if let TokenType::Identifier(identifier) = &token.token_type {
                    return environment.get_variable(identifier.to_string()).clone();
                }
                panic!("Expected Identifier")
            }
            Expr::Assignment(token, right) => {
                if let TokenType::Identifier(id) = token.token_type.clone() {
                    let result = right.interpret(environment);
                    environment.assign_variable(id, result.clone());
                    return result;
                }
                panic!("Expected Identifier")
            }
            _ => panic!("Unsupported Expr"),
        }
    }

    fn interpret_unary(right: &Expr, op: &Token, environment: &mut Box<Environment>) -> Value {
        let right_val = right.interpret(environment);

        match op.token_type {
            TokenType::Minus => Value::Number(-1.0 * Expr::get_number(right_val)),
            TokenType::Bang => Value::Boolean(!Expr::is_truthy(right_val)),
            _ => panic!("Error: Expected Unary Operation"),
        }
    }

    fn interpret_literal(token: &Token, environment: &mut Box<Environment>) -> Value {
        match &token.token_type {
            TokenType::Number(value) => Value::Number(*value),
            TokenType::String(value) => Value::String(value.clone()),
            TokenType::True => Value::Boolean(true),
            TokenType::False => Value::Boolean(false),
            TokenType::Nil => Value::Nil,
            _ => panic!("Error: Expected Literal"),
        }
    }

    fn interpret_binary(
        left: &Expr,
        right: &Expr,
        op: &Token,
        environment: &mut Box<Environment>,
    ) -> Value {
        let left_val = left.interpret(environment);
        let right_val = right.interpret(environment);

        match op.token_type {
            //Arithmetic operations
            TokenType::Plus => Expr::interpret_plus(left_val, right_val),
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
            TokenType::Less => {
                Value::Boolean(Expr::get_number(left_val) < Expr::get_number(right_val))
            }
            TokenType::LessEqual => {
                Value::Boolean(Expr::get_number(left_val) <= Expr::get_number(right_val))
            }
            TokenType::Greater => {
                Value::Boolean(Expr::get_number(left_val) > Expr::get_number(right_val))
            }
            TokenType::GreaterEqual => {
                Value::Boolean(Expr::get_number(left_val) >= Expr::get_number(right_val))
            }
            //Equality
            TokenType::EqualEqual => Value::Boolean(Expr::is_equal(left_val, right_val)),
            TokenType::BangEqual => Value::Boolean(!Expr::is_equal(left_val, right_val)),
            _ => panic!("Error: Expected Binary Operation"),
        }
    }

    fn interpret_plus(left_val: Value, right_val: Value) -> Value {
        match (left_val, right_val) {
            (Value::Number(left), Value::Number(right)) => Value::Number(left + right),
            (Value::String(left), Value::String(right)) => {
                Value::String(format!("{}{}", left, right))
            }
            _ => panic!("Mismatched arguments"),
        }
    }

    pub fn is_truthy(value: Value) -> bool {
        match value {
            Value::Boolean(false) | Value::Nil => false,
            _ => true,
        }
    }

    fn is_equal(left_val: Value, right_val: Value) -> bool {
        match (left_val, right_val) {
            (Value::Number(left), Value::Number(right)) => left == right,
            (Value::String(left), Value::String(right)) => left == right,
            (Value::Nil, Value::Nil) => true,
            _ => false,
        }
    }

    fn get_number(value: Value) -> f32 {
        if let Value::Number(result) = value {
            return result;
        }
        panic!("Error: Expected Number")
    }
}
