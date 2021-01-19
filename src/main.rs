mod lox;
use lox::expr::Expr;
use lox::tokens::{Token, TokenType};

fn main() {
    let args: Vec<_> = std::env::args().collect();
    println!("{:?}", args);
    if args.len() > 2 {
        println!("Usage: rlox [script]")
    } else if args.len() == 2 {
        println!("Running {}", args[1]);
        lox::run_file(&args[1]);
    } else {
        println!("Running Prompt");
        lox::run_prompt();
    }
}
