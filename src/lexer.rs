use crate::token::{Token, TokenType};

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
        let token = match self.ch {
            '=' => Token::new(TokenType::Assign, self.ch.to_string()),
            ';' => Token::new(TokenType::Semicolon, self.ch.to_string()),
            '(' => Token::new(TokenType::LParen, self.ch.to_string()),
            ')' => Token::new(TokenType::RParen, self.ch.to_string()),
            '{' => Token::new(TokenType::LBrace, self.ch.to_string()),
            '}' => Token::new(TokenType::RBrace, self.ch.to_string()),
            ',' => Token::new(TokenType::Comma, self.ch.to_string()),
            '+' => Token::new(TokenType::Add, self.ch.to_string()),
            '-' => Token::new(TokenType::Substract, self.ch.to_string()),
            '\0' => Token::new(TokenType::Eof, self.ch.to_string()),
            _ => Token::new(TokenType::Illegal, self.ch.to_string()),
        };
        self.read_char();
        token
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
}

#[cfg(test)]
mod test {
    use crate::token::TokenType;

    use super::Lexer;

    #[test]
    fn test_next_token() {
        let input = "=+(){},;";
        let tests = vec![
            TokenType::Assign,
            TokenType::Add,
            TokenType::LParen,
            TokenType::RParen,
            TokenType::LBrace,
            TokenType::RBrace,
            TokenType::Comma,
            TokenType::Semicolon,
        ];

        let mut l = Lexer::new(input);

        for tt in tests {
            let tok = l.next_token();

            assert_eq!(tok.token_type(), tt);
        }
    }
}
