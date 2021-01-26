use crate::lox::{
    environment::Environment,
    expr::Expr,
    interpreter::Value,
    tokens::{Token, TokenType},
};
use std::fmt;
use std::fmt::Write;

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
    pub fn interpret(&self, environment: &mut Environment, output: &mut String) {
        match self {
            Stmt::Expression(expr) => {
                expr.interpret(environment);
            }
            Stmt::Print(expr) => {
                write!(output, "{}", expr.interpret(environment));
            }
            Stmt::Var(token) => {
                if let TokenType::Identifier(identifier) = &token.token_type {
                    environment.set_variable(identifier.to_string(), Value::Nil);
                } else {
                    panic!("Expected Identifier");
                }
            }
            Stmt::InitialisedVar(token, initialiser) => {
                if let TokenType::Identifier(identifier) = &token.token_type {
                    let value = initialiser.interpret(environment);
                    environment.set_variable(identifier.to_string(), value);
                    environment.print();
                } else {
                    panic!("Expected Identifier");
                }
            }
            _ => panic!("Unsupported Expression"),
        };
    }
}
