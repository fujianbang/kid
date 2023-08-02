use crate::lexer::token::{Token, OperatorToken, DelimiterToken, KeywordToken};

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
            '=' => Token::Operator(OperatorToken::EQUAL),
            ';' => Token::Delimiter(DelimiterToken::SEMICOLON),
            '(' => Token::Delimiter(DelimiterToken::LEFT_PAREN),
            ')' => Token::Delimiter(DelimiterToken::RIGHT_PAREN),
            '{' => Token::Delimiter(DelimiterToken::LEFT_BRACE),
            '}' => Token::Delimiter(DelimiterToken::RIGHT_BRACE),
            ',' => Token::Delimiter(DelimiterToken::COMMA),
            '.' => Token::Delimiter(DelimiterToken::DOT),

            // EOF
            '\x00' => Token::Keyword(KeywordToken::EOF),
            _ => {
                if is_letter(self.ch) {
                    return Token::IDENT(self.read_ident());
                }
                Token::ILLEGAL
            }
        }
    }

    fn read_ident(&mut self) -> String {
        let pos = self.current;
        // TODO
        return String::from("");
    }
}

fn is_letter(c: char) -> bool {
    'a' <= c && c <= 'z' || 'A' <= c && c <= 'Z' || c == '_'
}

impl From<&str> for Scanner {
    fn from(value: &str) -> Self {
        Self {
            source: String::from(value),
            last: 0,
            current: 0,
            line: 1,
            ch: Default::default(),
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

    #[test]
    fn test_char_zero() {
        let zero: char = Default::default();
        assert_eq!('\x00', zero);
    }
}