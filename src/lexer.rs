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
            '=' => Token::new(TokenType::Assign, self.ch.to_string()),
            ';' => Token::new(TokenType::Semicolon, self.ch.to_string()),
            '(' => Token::new(TokenType::LParen, self.ch.to_string()),
            ')' => Token::new(TokenType::RParen, self.ch.to_string()),
            '{' => Token::new(TokenType::LBrace, self.ch.to_string()),
            '}' => Token::new(TokenType::RBrace, self.ch.to_string()),
            ',' => Token::new(TokenType::Comma, self.ch.to_string()),
            '+' => Token::new(TokenType::Plus, self.ch.to_string()),
            '-' => Token::new(TokenType::Minus, self.ch.to_string()),
            '<' => Token::new(TokenType::LessThan, self.ch.to_string()),
            '>' => Token::new(TokenType::GreaterThan, self.ch.to_string()),
            '&' => 
                if self.peek_char() == '&' {
                    let ch = self.ch;
                    self.read_char();
                    Token::new(TokenType::ShortCircuitAnd, format!("{}{}", ch, self.ch))
                } else {
                    Token::new(TokenType::Illegal, self.ch.to_string())
                },
            '|' => 
                if self.peek_char() == '|' {
                    let ch = self.ch;
                    self.read_char();
                    Token::new(TokenType::ShortCircuitOr, format!("{}{}", ch, self.ch))
                } else {
                    Token::new(TokenType::Or, self.ch.to_string())
                },
            '/' => Token::new(TokenType::Slash, self.ch.to_string()),
            '*' => Token::new(TokenType::Asterisk, self.ch.to_string()),
            '!' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    Token::new(TokenType::NotEqual, format!("{}{}", ch, self.ch))
                } else {
                    Token::new(TokenType::Not, self.ch.to_string())
                }
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
        while self.ch.is_alphabetic() {
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

#[cfg(test)]
mod test {
    use crate::token::{Token, TokenType};

    use super::Lexer;

    #[test]
    fn test_next_token() {
        let input = "=+(){},;";
        let tests = vec![
            TokenType::Assign,
            TokenType::Plus,
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

    #[test]
    fn test_token_stream_basic() {
        let input = "let five = 5;
            let ten = 10;
            let add = fn(x, y) {
                x + y;
            };
            let result = add(five, ten);";
        let mut expected: Vec<Token> = vec![
            Token::new(TokenType::Let, "let".to_string()),
            Token::new(TokenType::Identifier, "five".to_string()),
            Token::new(TokenType::Assign, "=".to_string()),
            Token::new(TokenType::Integer, "5".to_string()),
            Token::new(TokenType::Semicolon, ";".to_string()),
            Token::new(TokenType::Let, "let".to_string()),
            Token::new(TokenType::Identifier, "ten".to_string()),
            Token::new(TokenType::Assign, "=".to_string()),
            Token::new(TokenType::Integer, "10".to_string()),
            Token::new(TokenType::Semicolon, ";".to_string()),
            Token::new(TokenType::Let, "let".to_string()),
            Token::new(TokenType::Identifier, "add".to_string()),
            Token::new(TokenType::Assign, "=".to_string()),
            Token::new(TokenType::Function, "fn".to_string()),
            Token::new(TokenType::LParen, "(".to_string()),
            Token::new(TokenType::Identifier, "x".to_string()),
            Token::new(TokenType::Comma, ",".to_string()),
            Token::new(TokenType::Identifier, "y".to_string()),
            Token::new(TokenType::RParen, ")".to_string()),
            Token::new(TokenType::LBrace, "{".to_string()),
            Token::new(TokenType::Identifier, "x".to_string()),
            Token::new(TokenType::Plus, "+".to_string()),
            Token::new(TokenType::Identifier, "y".to_string()),
            Token::new(TokenType::Semicolon, ";".to_string()),
            Token::new(TokenType::RBrace, "}".to_string()),
            Token::new(TokenType::Semicolon, ";".to_string()),
            Token::new(TokenType::Let, "let".to_string()),
            Token::new(TokenType::Identifier, "result".to_string()),
            Token::new(TokenType::Assign, "=".to_string()),
            Token::new(TokenType::Identifier, "add".to_string()),
            Token::new(TokenType::LParen, "(".to_string()),
            Token::new(TokenType::Identifier, "five".to_string()),
            Token::new(TokenType::Comma, ",".to_string()),
            Token::new(TokenType::Identifier, "ten".to_string()),
            Token::new(TokenType::RParen, ")".to_string()),
            Token::new(TokenType::Semicolon, ";".to_string()),
            Token::new(TokenType::Eof, "\0".to_string()),
        ];

        let mut lex = Lexer::new(input);
        for expected_token in expected {
            let token = lex.next_token();
            assert_eq!(token, expected_token);
        }
    }

    #[test]
    fn test_token_stream_if_else() {
        let input = "
            if five < ten{
                return true;
            } else {
                return false;
            }
        ";   
        let expected = vec![
            Token::new(TokenType::If, "if".to_string()),
            Token::new(TokenType::Identifier, "five".to_string()),
            Token::new(TokenType::LessThan, "<".to_string()),
            Token::new(TokenType::Identifier, "ten".to_string()),
            Token::new(TokenType::LBrace, "{".to_string()),
            Token::new(TokenType::Return, "return".to_string()),
            Token::new(TokenType::True, "true".to_string()),
            Token::new(TokenType::Semicolon, ";".to_string()),
            Token::new(TokenType::RBrace, "}".to_string()),
            Token::new(TokenType::Else, "else".to_string()),
            Token::new(TokenType::LBrace, "{".to_string()),
            Token::new(TokenType::Return, "return".to_string()),
            Token::new(TokenType::False, "false".to_string()),
            Token::new(TokenType::Semicolon, ";".to_string()),
            Token::new(TokenType::RBrace, "}".to_string()),
            Token::new(TokenType::Eof, "\0".to_string()),
        ];
        let mut lex = Lexer::new(input);
        for expected_token in expected {
            let token = lex.next_token();
            assert_eq!(token, expected_token);
        }
    }
}
