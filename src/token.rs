#[derive(PartialEq, Debug, Clone, Copy)]
pub enum TokenType {
    Add,
    Assign,
    Comma,
    Eof,
    Float,
    Function,
    Identifier,
    Illegal,
    Integer,
    Let,
    LBrace,
    LParen,
    RBrace,
    RParen,
    Substract,
    Semicolon,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, literal: String) -> Self {
        Token {
            token_type,
            literal,
        }
    }

    pub fn token_type(&self) -> TokenType {
        self.token_type
    }

    pub fn literal(&self) -> String {
        self.literal.clone()
    }
}
