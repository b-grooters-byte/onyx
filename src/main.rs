#[macro_use]
extern crate lazy_static;

use colored::Colorize;

mod test;
mod lexer;
mod token;


const PROMPT: &str = ">> ";

fn main() {
    print!("ONYX REPL ");
    let version = env!("CARGO_PKG_VERSION");
    println!("{}{}", "v".green(), version.green());
    println!("Press Ctrl+C to exit");

    scan();
}

fn scan() {
    let mut input = String::new();
    loop {
        print!("{}", PROMPT);
        std::io::stdin().read_line(&mut input).unwrap();
        let mut l = lexer::Lexer::new(&input);
        loop {
            let tok = l.next_token();
            if tok.token_type() == token::TokenType::Eof {
                break;
            }
            println!("{:?}", tok);
        }
        input.clear();
    }
}