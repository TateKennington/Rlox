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
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Stmt::Expression(expr) => write!(f, "Expr {}", expr),
            Stmt::Print(expr) => write!(f, "Print {}", expr),
            Stmt::Var(token) => write!(f, "Var {}", token),
            Stmt::InitialisedVar(token, expr) => write!(f, "Init Var {} {}", token, expr),
            Stmt::Block(_) => write!(f, "Block Statement"),
            _ => panic!("Unsupported Statement"),
        }
    }
}

impl<'a> Stmt {
    pub fn interpret(&self, environment: &mut Rc<Environment>, output: &mut String) {
        match self {
            Stmt::Expression(expr) => {
                expr.interpret(environment);
            }
            Stmt::Print(expr) => {
                writeln!(output, "{}", expr.interpret(environment));
            }
            Stmt::Var(token) => {
                if let TokenType::Identifier(identifier) = &token.token_type {
                    Rc::get_mut(environment)
                        .unwrap()
                        .set_variable(identifier.to_string(), Value::Nil);
                } else {
                    panic!("Expected Identifier");
                }
            }
            Stmt::InitialisedVar(token, initialiser) => {
                if let TokenType::Identifier(identifier) = &token.token_type {
                    let value = initialiser.interpret(environment);
                    Rc::get_mut(environment)
                        .unwrap()
                        .set_variable(identifier.to_string(), value);
                    environment.print();
                } else {
                    panic!("Expected Identifier");
                }
            }
            Stmt::Block(stmts) => {
                let mut local = Environment::new();
                local.parent = Some(environment.clone());
                let mut local = Rc::new(local);
                for stmt in stmts.iter() {
                    stmt.interpret(&mut local, output);
                }
            }
            _ => panic!("Unsupported Expression"),
        }
    }
}
