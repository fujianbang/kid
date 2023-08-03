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

    fn next_token(&mut self) -> Token {
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
mod test_cases {
    use crate::lexer::token::LiteralToken;
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

        let test_cases = vec![
            Token::Keyword(KeywordToken::LET),
            Token::IDENT(String::from("five")),
            Token::Operator(OperatorToken::EQUAL),
            Token::Literal(LiteralToken::INT(5)),
            Token::Delimiter(DelimiterToken::SEMICOLON),
            Token::Keyword(KeywordToken::LET),
            Token::IDENT(String::from("ten")),
            Token::Operator(OperatorToken::EQUAL),
            Token::Literal(LiteralToken::INT(10)),
            Token::Delimiter(DelimiterToken::SEMICOLON),
            Token::Keyword(KeywordToken::LET),
            Token::IDENT(String::from("add")),
            Token::Operator(OperatorToken::EQUAL),
            Token::Keyword(KeywordToken::FN),
            Token::Delimiter(DelimiterToken::LEFT_PAREN),
            Token::IDENT(String::from("a")),
            Token::Delimiter(DelimiterToken::COMMA),
            Token::IDENT(String::from("b")),
            Token::Delimiter(DelimiterToken::RIGHT_PAREN),
            Token::Delimiter(DelimiterToken::LEFT_BRACE),
            Token::Keyword(KeywordToken::RETURN),
            Token::IDENT(String::from("a")),
            Token::Operator(OperatorToken::Add),
            Token::IDENT(String::from("b")),
            Token::Delimiter(DelimiterToken::SEMICOLON),
            Token::Delimiter(DelimiterToken::RIGHT_BRACE),
            Token::Delimiter(DelimiterToken::SEMICOLON),
            Token::Keyword(KeywordToken::LET),
            Token::IDENT(String::from("result")),
            Token::Operator(OperatorToken::EQUAL),
            Token::IDENT(String::from("add")),
            Token::Delimiter(DelimiterToken::LEFT_PAREN),
            Token::IDENT(String::from("five")),
            Token::Delimiter(DelimiterToken::COMMA),
            Token::IDENT(String::from("ten")),
            Token::Delimiter(DelimiterToken::RIGHT_PAREN),
            Token::Delimiter(DelimiterToken::SEMICOLON),
            Token::Keyword(KeywordToken::EOF),
        ];

        let mut s = Scanner::new(source);
        for test_case in test_cases {
            let token = s.next_token();
            assert_eq!(test_case, token);
        }
    }

    #[test]
    fn test_char_zero() {
        let zero: char = Default::default();
        assert_eq!('\x00', zero);
    }
}