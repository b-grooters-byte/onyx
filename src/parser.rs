use std::{error::Error, fmt::{Display, Formatter}};

use crate::{lexer, token, ast};

#[derive(Debug, Clone)]
pub struct ParseError {
    msg: String,
}   

impl ParseError {
    fn new(msg: String) -> Self {
        ParseError {
            msg,
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParseError: {}", self.msg)
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        &self.msg
    }
}

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

    fn parse_identifier(&mut self) -> Result<ast::Identifier, ParseError> {
        Ok(ast::Identifier::new(self.cur_token.clone(), self.cur_token.literal()))
    }
}