use crate::token::{Token, TokenType};
use crate::lexer::Lexer;

#[test]
fn test_token_steam_symbols() {
    let input = "=+*/(){},;";
    let tests = vec![
        TokenType::Assign,
        TokenType::Plus,
        TokenType::Asterisk,
        TokenType::Slash,
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

#[test]
fn test_token_stream_comparison() {
    let input = "
        10 == 10;
        10 != 9;
        a > b;
        a < b;
        a >= b;
        a <= b;
    ";
    let expected = vec![
        Token::new(TokenType::Integer, "10".to_string()),
        Token::new(TokenType::Equal, "==".to_string()),
        Token::new(TokenType::Integer, "10".to_string()),
        Token::new(TokenType::Semicolon, ";".to_string()),
        Token::new(TokenType::Integer, "10".to_string()),
        Token::new(TokenType::NotEqual, "!=".to_string()),
        Token::new(TokenType::Integer, "9".to_string()),
        Token::new(TokenType::Semicolon, ";".to_string()),
        Token::new(TokenType::Identifier, "a".to_string()),
        Token::new(TokenType::GreaterThan, ">".to_string()),
        Token::new(TokenType::Identifier, "b".to_string()),
        Token::new(TokenType::Semicolon, ";".to_string()),
        Token::new(TokenType::Identifier, "a".to_string()),
        Token::new(TokenType::LessThan, "<".to_string()),
        Token::new(TokenType::Identifier, "b".to_string()),
        Token::new(TokenType::Semicolon, ";".to_string()),
        Token::new(TokenType::Identifier, "a".to_string()),
        Token::new(TokenType::GreaterThanOrEqual, ">=".to_string()),
        Token::new(TokenType::Identifier, "b".to_string()),
        Token::new(TokenType::Semicolon, ";".to_string()),
        Token::new(TokenType::Identifier, "a".to_string()),
        Token::new(TokenType::LessThanOrEqual, "<=".to_string()),
        Token::new(TokenType::Identifier, "b".to_string()),
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
fn test_token_stream_assignment() {
    let input = "
        a = b * c;
        d = e / f;
        g = h + i;
        j = k - l;
        a *= 4;
        b /= 5;
        c += 6;
        d -= 7;
        e &= 8;
        f |= 9;
    ";

    let expected = vec![
        Token::new(TokenType::Identifier, "a".to_string()),
        Token::new(TokenType::Assign, "=".to_string()),
        Token::new(TokenType::Identifier, "b".to_string()),
        Token::new(TokenType::Asterisk, "*".to_string()),
        Token::new(TokenType::Identifier, "c".to_string()),
        Token::new(TokenType::Semicolon, ";".to_string()),
        Token::new(TokenType::Identifier, "d".to_string()),
        Token::new(TokenType::Assign, "=".to_string()),
        Token::new(TokenType::Identifier, "e".to_string()),
        Token::new(TokenType::Slash, "/".to_string()),
        Token::new(TokenType::Identifier, "f".to_string()),
        Token::new(TokenType::Semicolon, ";".to_string()),
        Token::new(TokenType::Identifier, "g".to_string()),
        Token::new(TokenType::Assign, "=".to_string()),
        Token::new(TokenType::Identifier, "h".to_string()),
        Token::new(TokenType::Plus, "+".to_string()),
        Token::new(TokenType::Identifier, "i".to_string()),
        Token::new(TokenType::Semicolon, ";".to_string()),
        Token::new(TokenType::Identifier, "j".to_string()),
        Token::new(TokenType::Assign, "=".to_string()),
        Token::new(TokenType::Identifier, "k".to_string()),
        Token::new(TokenType::Minus, "-".to_string()),
        Token::new(TokenType::Identifier, "l".to_string()),
        Token::new(TokenType::Semicolon, ";".to_string()),
        Token::new(TokenType::Identifier, "a".to_string()),
        Token::new(TokenType::Assign, "=".to_string()),
        Token::new(TokenType::Identifier, "a".to_string()),
        Token::new(TokenType::MultiplyAssign, "*=".to_string()),
        Token::new(TokenType::Integer, "4".to_string()),
        Token::new(TokenType::Semicolon, ";".to_string()),
        Token::new(TokenType::Identifier, "b".to_string()),
        Token::new(TokenType::DivideAssign, "/=".to_string()),
        Token::new(TokenType::Integer, "5".to_string()),
        Token::new(TokenType::Semicolon, ";".to_string()),
        Token::new(TokenType::Identifier, "c".to_string()),
        
        
    ];
}