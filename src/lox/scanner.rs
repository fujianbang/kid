use crate::lox::token::{Token, TokenType};

use nom::{
    IResult,
    sequence::delimited,
    character::complete::char,
    bytes::complete::is_not,
    branch::alt,
};

#[derive(Debug, Copy, Clone)]
struct Scanner {
    source: String,
    // tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl From<&str> for Scanner {
    fn from(value: &str) -> Self {
        Self {
            source: String::from(value),
            // tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }
}

impl Scanner {
    fn next(&self) -> Token {
        Token::new()
    }

    fn scan_tokens(&mut self) -> Vec<Token> {
        let tokens = vec![];
        // TODO 循环解析token

        let eof = Token::new(TokenType::EOF, String::from(""), self.line);
        self.tokens.push(eof);

        self.tokens
    }
}

fn parse_token(s: &str) -> TokenType {
    // switch case
    if 
}

#[cfg(test)]
mod tests {
    use super::Scanner;

    #[test]
    fn test_from() {
        let scanner = Scanner::from("let a = 100;");
        println!("{:?}", scanner);
    }
}