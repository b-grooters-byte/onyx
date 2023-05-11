use std::collections::HashMap;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum TokenType {
    And,
    AddAssign,
    AndAssign,
    Assign,
    Asterisk,
    BitwiseAndAssign,
    BitwiseOrAssign,
    Tilde,
    Comma,
    DivideAssign,
    Else,
    Eof,
    Equal,
    False,
    Float,
    Function,
    GreaterThan,
    GreaterThanOrEqual,
    Identifier,
    If,
    Illegal,
    Integer,
    Let,
    LessThan,
    LessThanOrEqual,
    LBrace,
    LParen,
    Minus,
    MultiplyAssign,
    Not,
    NotEqual,
    Or,
    OrAssign,
    Plus,
    RBrace,
    Return,
    RParen,
    Semicolon,
    ShortCircuitAnd,
    ShortCircuitOr,
    SubtractAssign,
    Slash,
    True,
}

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, TokenType> = {
        let mut map = HashMap::new();
        map.insert("else", TokenType::Else);
        map.insert("false", TokenType::False);
        map.insert("if", TokenType::If);
        map.insert("fn", TokenType::Function);
        map.insert("let", TokenType::Let);
        map.insert("return", TokenType::Return);
        map.insert("true", TokenType::True);
        
        map
    };
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

pub fn map_identifier(ident: &str) -> TokenType {
    match KEYWORDS.get(ident) {
        Some(token_type) => *token_type,
        None => TokenType::Identifier,
    }
}
