use crate::lox::{
    expr::Expr,
    stmt::Stmt,
    tokens::{Token, TokenType},
};

pub struct Parser {
    tokens: Vec<Token>,
    curr: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, curr: 0 }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut program = vec![];
        while !matches!(self.peek().token_type, TokenType::Eof) {
            program.push(self.declaration());
        }
        return program;
    }

    fn declaration(&mut self) -> Stmt {
        if matches!(self.peek().token_type, TokenType::Var) {
            self.advance();
            return self.variable_declaration();
        }
        return self.statement();
    }

    fn variable_declaration(&mut self) -> Stmt {
        let identifier = self.advance();
        if !matches!(identifier.token_type, TokenType::Identifier(_)) {
            panic!("Expected Identifier")
        }
        if matches!(self.peek().token_type, TokenType::Equal) {
            self.advance();
            let initialiser = self.expression();
            let res = Stmt::InitialisedVar(identifier, Box::new(initialiser));
            if !matches!(self.advance().token_type, TokenType::Semicolon) {
                panic!("Expected Semicolon")
            }
            return res;
        }

        let res = Stmt::Var(identifier);
        if !matches!(self.advance().token_type, TokenType::Semicolon) {
            panic!("Expected Semicolon")
        }
        return res;
    }

    fn statement(&mut self) -> Stmt {
        let stmt = match self.peek().token_type {
            TokenType::Print => {
                self.advance();
                Stmt::Print(Box::new(self.expression()))
            }
            TokenType::LeftBrace => {
                self.advance();
                return Stmt::Block(Box::new(self.block()));
            }
            _ => Stmt::Expression(Box::new(self.expression())),
        };
        if !matches!(self.advance().token_type, TokenType::Semicolon) {
            panic!("Expected Semicolon")
        }
        return stmt;
    }

    fn block(&mut self) -> Vec<Stmt> {
        let mut res = vec![];
        while !matches!(self.peek().token_type, TokenType::RightBrace) {
            res.push(self.declaration());
        }
        if !matches!(self.advance().token_type, TokenType::RightBrace) {
            panic!("Expected } at end of block")
        }
        return res;
    }

    fn expression(&mut self) -> Expr {
        return self.assignment();
    }

    fn assignment(&mut self) -> Expr {
        let left = self.equality();
        if matches!(self.peek().token_type, TokenType::Equal) {
            self.advance();
            let right = self.assignment();
            if let Expr::Var(id) = left {
                return Expr::Assignment(id, Box::new(right));
            }
            panic!("Invalid Assignment")
        }
        return left;
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();
        while matches!(
            self.peek().token_type,
            TokenType::BangEqual | TokenType::EqualEqual
        ) {
            let operator = self.advance();
            let right = self.comparison();
            expr = Expr::Binary {
                left: Box::new(expr),
                op: operator,
                right: Box::new(right),
            }
        }
        return expr;
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();
        while matches!(
            self.peek().token_type,
            TokenType::Greater | TokenType::GreaterEqual | TokenType::Less | TokenType::LessEqual
        ) {
            let operator = self.advance();
            let right = self.term();
            expr = Expr::Binary {
                left: Box::new(expr),
                op: operator,
                right: Box::new(right),
            }
        }
        return expr;
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();
        while matches!(self.peek().token_type, TokenType::Plus | TokenType::Minus) {
            let operator = self.advance();
            let right = self.factor();
            expr = Expr::Binary {
                left: Box::new(expr),
                op: operator,
                right: Box::new(right),
            }
        }
        return expr;
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();
        while matches!(self.peek().token_type, TokenType::Slash | TokenType::Star) {
            let operator = self.advance();
            let right = self.unary();
            expr = Expr::Binary {
                left: Box::new(expr),
                op: operator,
                right: Box::new(right),
            }
        }
        return expr;
    }

    fn unary(&mut self) -> Expr {
        if matches!(self.peek().token_type, TokenType::Bang | TokenType::Minus) {
            let operator = self.advance();
            let right = self.unary();
            return Expr::Unary {
                op: operator,
                right: Box::new(right),
            };
        }
        return self.primary();
    }

    fn primary(&mut self) -> Expr {
        match self.peek().token_type {
            TokenType::True
            | TokenType::False
            | TokenType::Nil
            | TokenType::Number(_)
            | TokenType::String(_) => Expr::Literal(self.advance()),
            TokenType::Identifier(_) => Expr::Var(self.advance()),
            TokenType::LeftParen => {
                self.advance();
                let expr = self.expression();
                if !matches!(self.advance().token_type, TokenType::RightParen) {
                    println!("Err: mismatched paren");
                }
                return Expr::Grouping(Box::new(expr));
            }
            _ => {
                println!("Err: Expected Literal");
                Expr::Literal(Token::new(
                    TokenType::String(String::from("Error")),
                    String::from("Error"),
                    0,
                ))
            }
        }
    }

    fn advance(&mut self) -> Token {
        if self.is_at_end() {
            return self.peek();
        }
        self.curr += 1;
        return self.previous();
    }

    fn previous(&mut self) -> Token {
        return self.tokens[self.curr - 1].clone();
    }

    fn peek(&mut self) -> Token {
        return self.tokens[self.curr].clone();
    }

    fn is_at_end(&mut self) -> bool {
        return matches!(self.peek().token_type, TokenType::Eof);
    }
}
