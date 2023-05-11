use crate::token::{map_identifier, Token, TokenType};

#[derive(Debug, PartialEq, Clone)]
pub struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut lexer = Lexer {
            input: input.chars().collect(),
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        lexer.read_char();
        lexer
    }

    pub fn next_token(&mut self) -> Token {
        while self.ch.is_whitespace() {
            self.read_char();
        }
        println!("next_token char: {}", self.ch);
         let token = match self.ch {
            '=' => 
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    Token::new(TokenType::Equal, format!("{}{}", ch, self.ch))
                } else {
                    Token::new(TokenType::Assign, self.ch.to_string())
                },
            ';' => Token::new(TokenType::Semicolon, self.ch.to_string()),
            '(' => Token::new(TokenType::LParen, self.ch.to_string()),
            ')' => Token::new(TokenType::RParen, self.ch.to_string()),
            '{' => Token::new(TokenType::LBrace, self.ch.to_string()),
            '}' => Token::new(TokenType::RBrace, self.ch.to_string()),
            ',' => Token::new(TokenType::Comma, self.ch.to_string()),
            '+' => 
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    Token::new(TokenType::AddAssign, format!("{}{}", ch, self.ch))
                } else {
                    Token::new(TokenType::Plus, self.ch.to_string())
                },
            '-' =>  
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    Token::new(TokenType::SubtractAssign, format!("{}{}", ch, self.ch))
                } else {
                    Token::new(TokenType::Minus, self.ch.to_string())
                },
            '<' => 
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    Token::new(TokenType::LessThanOrEqual, format!("{}{}", ch, self.ch))
                } else {
                    Token::new(TokenType::LessThan, self.ch.to_string())
                },
            '>' => 
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    Token::new(TokenType::GreaterThanOrEqual, format!("{}{}", ch, self.ch))
                } else {
                    Token::new(TokenType::GreaterThan, self.ch.to_string())
                },
            '&' => 
                match self.peek_char() {
                    '&' => {
                        let ch = self.ch;
                        self.read_char();
                        Token::new(TokenType::ShortCircuitAnd, format!("{}{}", ch, self.ch))
                    },
                    '=' => {
                        let ch = self.ch;
                        self.read_char();
                        Token::new(TokenType::AndAssign, format!("{}{}", ch, self.ch))
                    },
                    _ => Token::new(TokenType::And, self.ch.to_string())
                },
            '|' => 
                match self.peek_char() {
                    '|' => {
                        let ch = self.ch;
                        self.read_char();
                        Token::new(TokenType::ShortCircuitOr, format!("{}{}", ch, self.ch))
                    },
                    '=' => {
                        let ch = self.ch;
                        self.read_char();
                        Token::new(TokenType::OrAssign, format!("{}{}", ch, self.ch))
                    },
                    _ => Token::new(TokenType::Or, self.ch.to_string()),
                },
            '/' => 
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    Token::new(TokenType::DivideAssign, format!("{}{}", ch, self.ch))
                } else {
                    Token::new(TokenType::Slash, self.ch.to_string())
                },                
            '*' => 
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    Token::new(TokenType::MultiplyAssign, format!("{}{}", ch, self.ch))
                } else {
                    Token::new(TokenType::Asterisk, self.ch.to_string())
                },
            '!' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    Token::new(TokenType::NotEqual, format!("{}{}", ch, self.ch))
                } else {
                    Token::new(TokenType::Not, self.ch.to_string())
                }
            },
            '%' => 
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    Token::new(TokenType::RemainderAssign, format!("{}{}", ch, self.ch))
                } else {
                    Token::new(TokenType::Remainder, self.ch.to_string())
                },
            '\0' => Token::new(TokenType::Eof, self.ch.to_string()),
            _ => {
                if self.ch.is_alphabetic() {
                    let literal = self.read_identifier();
                    let token_type = map_identifier(&literal);
                    return Token::new(token_type, literal);
                } else if self.ch.is_digit(10) {
                    let literal = self.read_number();
                    return Token::new(TokenType::Integer, literal);
                } else {
                    Token::new(TokenType::Illegal, self.ch.to_string())
                }
            }
        };
        self.read_char();
        token
    }

    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input[self.read_position]
        }
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {            
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_identifier(&mut self) -> String {
        let pos = self.position;
        while self.ch.is_alphabetic() || self.ch == '_' {
            self.read_char();
        }
        self.input[pos..self.position].iter().collect()
    }

    fn read_number(&mut self) -> String {
        let pos = self.position;
        while self.ch.is_digit(10) {
            self.read_char();
        }
        self.input[pos..self.position].iter().collect()
    }

}

