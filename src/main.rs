mod lox;
use lox::expr::Expr;
use lox::tokens::{Token, TokenType};

fn main() {
    /* println!(
        "{}",
        Expr::Binary {
            left: Box::new(Expr::Unary {
                op: Token {
                    token_type: TokenType::Minus,
                    lexeme: String::from("-"),
                    line: 0
                },
                right: Box::new(Expr::Literal(Token {
                    token_type: TokenType::Number(123.0),
                    lexeme: String::from("123"),
                    line: 0
                })),
            }),
            right: Box::new(Expr::Grouping(Box::new(Expr::Literal(Token {
                token_type: TokenType::Number(123.0),
                lexeme: String::from("123"),
                line: 0
            })))),
            op: Token {
                token_type: TokenType::Star,
                lexeme: String::from("*"),
                line: 0
            }
        }
        .print_rpn()
    ); */
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
