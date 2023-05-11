#[macro_use]
extern crate lazy_static;

use colored::Colorize;

mod test;
mod lexer;
mod token;

fn main() {
    print!("ONYX REPL ");
    let version = env!("CARGO_PKG_VERSION");
    println!("{}{}", "v".green(), version.green());
    println!("Press Ctrl+C to exit");
}
