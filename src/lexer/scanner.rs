use crate::lexer::token::{Token, OperatorToken, DelimiterToken, KeywordToken};

#[derive(Debug, Clone)]
struct Scanner {
    source: String,
    last: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    /// create new lexer scanner
    fn new(input: &str) -> Self {
        let s = Scanner::from(input);
        s
    }

    /// read next char and update current char position
    fn read_char(&mut self) -> char {
        if self.current >= self.source.len() {
            return 0 as char;
        }


        let c = self.source.chars().nth(self.current);
        self.last = self.current;
        self.current += 1;

        match c {
            Some(v) => v,
            None => 0 as char,
        }
    }

    /// retrieve the next Token
    fn next_token(&mut self) -> anyhow::Result<Token> {
        if self.is_at_end() {
            println!("EOF");
            return Ok(Token::Keyword(KeywordToken::EOF));
        }

        let ch = self.read_char();
        println!("last: {}, current: {}, line: {}, ch: {}",
                 self.last, self.current, self.line, ch);

        match ch {
            '+' => Ok(Token::Operator(OperatorToken::Add)),
            '-' => Ok(Token::Operator(OperatorToken::Sub)),
            '*' => Ok(Token::Operator(OperatorToken::STAR)),
            '/' => Ok(Token::Operator(OperatorToken::SLASH)),
            '%' => Ok(Token::Operator(OperatorToken::REM)),
            '=' => Ok(Token::Operator(OperatorToken::EQUAL)),
            ';' => Ok(Token::Delimiter(DelimiterToken::SEMICOLON)),
            '(' => Ok(Token::Delimiter(DelimiterToken::LEFT_PAREN)),
            ')' => Ok(Token::Delimiter(DelimiterToken::RIGHT_PAREN)),
            '{' => Ok(Token::Delimiter(DelimiterToken::LEFT_BRACE)),
            '}' => Ok(Token::Delimiter(DelimiterToken::RIGHT_BRACE)),
            ',' => Ok(Token::Delimiter(DelimiterToken::COMMA)),
            '.' => Ok(Token::Delimiter(DelimiterToken::DOT)),
            _ => {
                if is_letter(ch) {
                    return Ok(Token::IDENT(self.read_ident()));
                }

                // cannot parse so return error
                Err(anyhow::anyhow!("InvalidCharacter: {}", ch))
            }
        }
    }

    /// read identifier
    fn read_ident(&mut self) -> String {
        let _pos = self.current;

        return String::from("");
    }

    /// lookahead
    fn peek(&self) -> char {
        if self.current >= self.source.len() {
            return 0 as char;
        }
        let c = self.source.chars().nth(self.current);
        match c {
            Some(v) => v,
            None => 0 as char,
        }
    }

    /// peek next
    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return 0 as char;
        }
        let c = self.source.chars().nth(self.current + 1);
        match c {
            Some(v) => v,
            None => 0 as char,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}

fn is_letter(c: char) -> bool {
    ('a'..='z').contains(&c) || ('A'..='Z').contains(&c) || c == '_'
}

impl From<&str> for Scanner {
    fn from(value: &str) -> Self {
        Self {
            source: String::from(value),
            last: 0,
            current: 0,
            line: 1,
        }
    }
}


#[cfg(test)]
mod test_cases {
    use super::*;

    #[test]
    fn test_single_char_token() {
        let source = "+-*/%=";

        let mut scanner = Scanner::new(source);
        let mut tokens = vec![];
        loop {
            let token = scanner.next_token();
            let token = token.unwrap();
            if token == Token::Keyword(KeywordToken::EOF) {
                break;
            }
            tokens.push(token);
        }

        let expected = vec![
            Token::Operator(OperatorToken::Add),
            Token::Operator(OperatorToken::Sub),
            Token::Operator(OperatorToken::STAR),
            Token::Operator(OperatorToken::SLASH),
            Token::Operator(OperatorToken::REM),
            Token::Operator(OperatorToken::EQUAL),
        ];
        assert_eq!(tokens, expected);
    }

    // #[test]
    // fn test_simple_statement() {
    //     let source = "let a = \"lower case letter 'a'";
    //
    //     let expected = vec![
    //         Token::Keyword(KeywordToken::LET),
    //         Token::IDENT("a".to_string()),
    //         Token::Operator(OperatorToken::EQUAL),
    //         Token::Literal(LiteralToken::STRING("lower case letter 'a'".to_string())),
    //         Token::Delimiter(DelimiterToken::SEMICOLON),
    //     ];
    //
    //     let mut scanner = Scanner::new(source);
    //     let mut tokens = vec![];
    //     loop {
    //         let token = scanner.next_token().unwrap();
    //         if token == Token::Keyword(KeywordToken::EOF) {
    //             break;
    //         }
    //         tokens.push(token);
    //     }
    //
    //     assert_eq!(tokens, expected);
    // }

//     #[test]
//     fn test_basic_tokens() {
//         let source = r#"
// let five = 5;
// let ten = 10;
//
// let add = fun(a, b) {
//     return a + b;
// };
//
// let result = add(five, ten);
//         "#;
//
//         let test_cases = vec![
//             Token::Keyword(KeywordToken::LET),
//             Token::IDENT(String::from("five")),
//             Token::Operator(OperatorToken::EQUAL),
//             Token::Literal(LiteralToken::INT(5)),
//             Token::Delimiter(DelimiterToken::SEMICOLON),
//             Token::Keyword(KeywordToken::LET),
//             Token::IDENT(String::from("ten")),
//             Token::Operator(OperatorToken::EQUAL),
//             Token::Literal(LiteralToken::INT(10)),
//             Token::Delimiter(DelimiterToken::SEMICOLON),
//             Token::Keyword(KeywordToken::LET),
//             Token::IDENT(String::from("add")),
//             Token::Operator(OperatorToken::EQUAL),
//             Token::Keyword(KeywordToken::FN),
//             Token::Delimiter(DelimiterToken::LEFT_PAREN),
//             Token::IDENT(String::from("a")),
//             Token::Delimiter(DelimiterToken::COMMA),
//             Token::IDENT(String::from("b")),
//             Token::Delimiter(DelimiterToken::RIGHT_PAREN),
//             Token::Delimiter(DelimiterToken::LEFT_BRACE),
//             Token::Keyword(KeywordToken::RETURN),
//             Token::IDENT(String::from("a")),
//             Token::Operator(OperatorToken::Add),
//             Token::IDENT(String::from("b")),
//             Token::Delimiter(DelimiterToken::SEMICOLON),
//             Token::Delimiter(DelimiterToken::RIGHT_BRACE),
//             Token::Delimiter(DelimiterToken::SEMICOLON),
//             Token::Keyword(KeywordToken::LET),
//             Token::IDENT(String::from("result")),
//             Token::Operator(OperatorToken::EQUAL),
//             Token::IDENT(String::from("add")),
//             Token::Delimiter(DelimiterToken::LEFT_PAREN),
//             Token::IDENT(String::from("five")),
//             Token::Delimiter(DelimiterToken::COMMA),
//             Token::IDENT(String::from("ten")),
//             Token::Delimiter(DelimiterToken::RIGHT_PAREN),
//             Token::Delimiter(DelimiterToken::SEMICOLON),
//             Token::Keyword(KeywordToken::EOF),
//         ];
//
//         let mut s = Scanner::new(source);
//         for test_case in test_cases {
//             let token = s.next_token();
//             assert_eq!(test_case, token);
//         }
//     }

    #[test]
    fn test_char_zero() {
        let zero: char = Default::default();
        assert_eq!('\x00', zero);
    }
}