mod error;
mod scanner;
mod token;

use scanner::Scanner;

pub fn lexer(source: &str) -> Vec<token::Token> {
    let mut scanner = Scanner::new(source);
    let mut tokens = vec![];

    while let Some(token) = scanner.next_token() {
        tokens.push(token);
    }
    tokens
}
