use crate::lox::{
    expr::Expr,
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

    pub fn parse(&mut self) -> Expr {
        return self.expression();
    }

    fn expression(&mut self) -> Expr {
        return self.equality();
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
            TokenType::LeftParen => {
                let expr = self.expression();
                if !matches!(self.peek().token_type, TokenType::RightParen) {
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
