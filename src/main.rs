mod lox;
use lox::expr::Expr;
use lox::tokens::{Token, TokenType};

fn main() {
    let args: Vec<_> = std::env::args().collect();
    println!("{:?}", args);
    if args.len() > 2 {
        println!("Usage: rlox [script]")
    } else if args.len() == 2 {
        let mut result = String::new();
        lox::run_file(&args[1], &mut result);
        println!("{}", result);
    } else {
        lox::run_prompt();
    }
}
