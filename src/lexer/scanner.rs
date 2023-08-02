use crate::lexer::token::{Token, OperatorToken};

use nom::{
    IResult,
    sequence::delimited,
    // character::complete::char,
    bytes::complete::is_not,
    branch::alt,
};

#[derive(Debug, Clone)]
struct Scanner {
    source: String,
    last: usize,
    current: usize,
    line: usize,
    ch: char,
}

impl Scanner {
    /// create new lexer scanner
    fn new(input: &str) -> Self {
        let mut s = Scanner::from(input);
        s.read_char();
        s
    }

    /// read next char
    fn read_char(&mut self) {
        if self.current >= self.source.len() {
            self.ch = 0 as char;
        } else {
            let c = self.source.chars().take(self.source.len()).next();
            self.ch = match c {
                Some(v) => v,
                None => 0 as char,
            }
        }

        self.last = self.current;
        self.current += 1;
    }

    fn next_token(&self) -> Token {
        match self.ch {
            '+' => Token::Operator(OperatorToken::Add),
            '-' => Token::Operator(OperatorToken::Sub),
            '*' => Token::Operator(OperatorToken::STAR),
            '/' => Token::Operator(OperatorToken::SLASH),
            '%' => Token::Operator(OperatorToken::REM),
            _ => Token::ILLEGAL
        }
    }
}

impl From<&str> for Scanner {
    fn from(value: &str) -> Self {
        Self {
            source: String::from(value),
            last: 0,
            current: 0,
            line: 1,
            ch: 0 as char,
        }
    }
}


#[cfg(test)]
mod test_case {
    use super::*;

    #[test]
    fn test_basic_tokens() {
        let source = r#"
let five = 5;
let ten = 10;

let add = fun(a, b) {
    return a + b;
};

let result = add(five, ten);
        "#;
        println!("{}", source);
    }
}