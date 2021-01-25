mod environment;
pub mod expr;
mod interpreter;
mod parser;
mod scanner;
mod stmt;
pub mod tokens;

static mut HAD_ERROR: bool = false;

pub fn run_prompt() {
    let stdin = std::io::stdin();
    let mut buffer = String::default();
    while stdin.read_line(&mut buffer).unwrap() != 0 {
        run(buffer);
        buffer = String::default();
    }
}

pub fn run_file(path: &String) {
    run(std::fs::read_to_string(path).unwrap());
}

pub fn run(source: String) {
    let scn = scanner::Scanner::new(source);
    let tokens = scn.scan_tokens();

    for token in tokens.iter() {
        println!("{}", token);
    }

    let mut parser = parser::Parser::new(tokens);
    let program = parser.parse();

    program.iter().for_each(|stmt| {
        println!("Stmt: {}", stmt);
        stmt.interpret();
    });
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
