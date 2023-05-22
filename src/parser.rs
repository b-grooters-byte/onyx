use std::{error::Error, fmt::{Display, Formatter}};

use crate::{lexer, token, ast::{self, Program, Statement}};

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
    pub fn new(lexer: lexer::Lexer) -> Self {
        let mut p = Parser {
            lexer,
            cur_token: token::Token::new(token::TokenType::Illegal, String::new()),
            peek_token: token::Token::new(token::TokenType::Illegal, String::new()),
        };
        p.next_token();
        p.next_token();
        p
    }

    fn expect_peek(&mut self, token_type: token::TokenType) -> bool {
        if self.peek_token.token_type() == token_type {
            self.next_token();
            true
        } else {
            false
        }
    }

    fn peek_token_is(&self, token_type: token::TokenType) -> bool {
        self.peek_token.token_type() == token_type
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Result<Program, ParseError> {
        let mut program = ast::Program::new();
        while self.cur_token.token_type() != token::TokenType::Eof {
            let stmt = self.parse_statement()?;
            program.statements_mut().push(stmt);
            self.next_token();
        }
        Ok(program)
    }

    fn parse_identifier(&mut self) -> Result<ast::Identifier, ParseError> {
        Ok(ast::Identifier::new(self.cur_token.clone(), self.cur_token.literal()))
    }

    fn parse_statement(&mut self) -> Result<Box<dyn Statement>, ParseError> {
        match self.cur_token.token_type() {
            token::TokenType::Let => self.parse_let_statement(),
            _ => Err(ParseError::new(format!("unknown token type: {}", self.cur_token.token_type()))),
        }
    }

    fn parse_let_statement(&mut self) -> Result<Box<dyn Statement>, ParseError> {
        let token = self.cur_token.clone();
        if !self.expect_peek(token::TokenType::Identifier) {
            return Err(ParseError::new(format!("expected Ident, got {}", self.peek_token.token_type())));
        }
        let name = self.parse_identifier()?;
        if !self.expect_peek(token::TokenType::Assign) {
            return Err(ParseError::new(format!("expected Assign, got {}", self.peek_token.token_type())));
        }
        while self.cur_token.token_type() != token::TokenType::Semicolon {
            self.next_token();
        }
        Ok(Box::new(ast::LetStatement::new(token, name, None)))
    }

}