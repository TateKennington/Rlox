pub mod environment;
pub mod expr;
mod interpreter;
mod parser;
mod scanner;
mod stmt;
pub mod tokens;

static mut HAD_ERROR: bool = false;

use std::rc::Rc;

pub fn run_prompt() {
    let stdin = std::io::stdin();
    let mut buffer = String::default();
    let mut environment = Rc::new(environment::Environment::new());
    let mut output = String::new();
    while stdin.read_line(&mut buffer).unwrap() != 0 {
        run(buffer, &mut environment, &mut output);
        println!("{:?}", output);
        output = String::new();
        buffer = String::default();
    }
}

pub fn run_file(path: &String, output: &mut String) {
    let mut environment = environment::Environment::new();
    run(
        std::fs::read_to_string(path).unwrap(),
        &mut Rc::new(environment),
        output,
    );
}

pub fn run(source: String, environment: &mut Rc<environment::Environment>, output: &mut String) {
    let scn = scanner::Scanner::new(source);
    let tokens = scn.scan_tokens();

    for token in tokens.iter() {
        println!("{}", token);
    }

    let mut parser = parser::Parser::new(tokens);
    let program = parser.parse();

    for stmt in program.iter() {
        println!("Stmt: {}", stmt);
        stmt.interpret(environment, output);
    }
}

pub fn evaluate_run(source: String) {
    let scn = scanner::Scanner::new(source);
    let tokens = scn.scan_tokens();

    for token in tokens.iter() {
        println!("{}", token);
    }

    let mut parser = parser::Parser::new(tokens);
    let program = parser.parse();
}

pub fn error(line: usize, message: &str) {
    report(line, "", message);
}

pub fn report(line: usize, location: &str, message: &str) {
    println!("[line {}] Error{}: {}", line, location, message);
    unsafe {
        HAD_ERROR = true;
    }
}
