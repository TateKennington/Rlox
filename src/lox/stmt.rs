use crate::lox::{
    environment::Environment,
    expr::Expr,
    interpreter::Value,
    tokens::{Token, TokenType},
};
use std::fmt;
use std::fmt::Write;
use std::rc::Rc;

pub enum Stmt {
    Expression(Box<Expr>),
    Print(Box<Expr>),
    Var(Token),
    InitialisedVar(Token, Box<Expr>),
    Block(Box<Vec<Stmt>>),
    If(Box<Expr>, Box<Stmt>, Option<Box<Stmt>>),
    While(Box<Expr>, Box<Stmt>),
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Stmt::Expression(expr) => write!(f, "Expr {}", expr),
            Stmt::Print(expr) => write!(f, "Print {}", expr),
            Stmt::Var(token) => write!(f, "Var {}", token),
            Stmt::InitialisedVar(token, expr) => write!(f, "Init Var {} {}", token, expr),
            Stmt::Block(_) => write!(f, "Block Statement"),
            Stmt::If(_, _, _) => write!(f, "If Statement"),
            Stmt::While(_, _) => write!(f, "While Statment"),
            _ => panic!("Unsupported Statement"),
        }
    }
}

impl<'a> Stmt {
    pub fn interpret(&self, environment: &mut Box<Environment>, output: &mut String) {
        match self {
            Stmt::Expression(expr) => {
                expr.interpret(environment);
            }
            Stmt::Print(expr) => {
                writeln!(output, "{}", expr.interpret(environment));
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
            Stmt::Block(stmts) => {
                let mut local = Environment::new();
                local.parent = Some(environment.clone());
                let mut local = Box::new(local);
                for stmt in stmts.iter() {
                    stmt.interpret(&mut local, output);
                }
                std::mem::replace(environment, local);
            }
            Stmt::If(condition, consequent, alternate) => {
                let condition = Expr::is_truthy(condition.interpret(environment));
                if condition {
                    consequent.interpret(environment, output);
                } else if let Some(alternate) = alternate {
                    alternate.interpret(environment, output);
                }
            }
            Stmt::While(condition, stmt) => {
                while Expr::is_truthy(condition.interpret(environment)) {
                    stmt.interpret(environment, output);
                }
            }
            _ => panic!("Unsupported Expression"),
        }
    }
}
