use crate::{lexer, token, ast};



pub struct Parser {
    lexer: lexer::Lexer,
    cur_token: token::Token,
    peek_token: token::Token,
}

impl Parser {
    fn new(lexer: lexer::Lexer) -> Self {
        let mut p = Parser {
            lexer,
            cur_token: token::Token::new(token::TokenType::Illegal, String::new()),
            peek_token: token::Token::new(token::TokenType::Illegal, String::new()),
        };
        p.next_token();
        p.next_token();
        p
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }
}