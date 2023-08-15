use super::token::*;

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
        let source = input.to_string();
        Self {
            source,
            last: 0,
            current: 0,
            line: 1,
        }
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
    fn next_token(&mut self) -> Option<Token> {
        if self.is_at_end() {
            println!("EOF");
            return None;
        }

        let ch = self.read_char();
        println!("ch: {:?}", ch);

        match ch {
            ' ' | '\t' | '\r' => self.next_token(),
            '\n' => {
                self.line += 1;
                self.next_token()
            }
            '+' => Some(Token::Operator(OperatorToken::Add)),
            '-' => Some(Token::Operator(OperatorToken::Sub)),
            '*' => Some(Token::Operator(OperatorToken::STAR)),
            '/' => Some(Token::Operator(OperatorToken::SLASH)),
            '%' => Some(Token::Operator(OperatorToken::REM)),
            '=' => {
                if self.peek() == '=' {
                    self.read_char();
                    return Some(Token::Operator(OperatorToken::EQUAL_EQUAL));
                }

                Some(Token::Operator(OperatorToken::EQUAL))
            }
            '!' => {
                if self.peek() == '=' {
                    self.read_char();
                    return Some(Token::Operator(OperatorToken::BANG_EQUAL));
                }
                Some(Token::Operator(OperatorToken::BANG))
            }
            '>' => {
                if self.peek() == '=' {
                    self.read_char();
                    return Some(Token::Operator(OperatorToken::GREATER_EQUAL));
                }
                Some(Token::Operator(OperatorToken::GREATER))
            }
            '<' => {
                if self.peek() == '=' {
                    self.read_char();
                    return Some(Token::Operator(OperatorToken::LESS_EQUAL));
                }
                Some(Token::Operator(OperatorToken::LESS))
            }
            ';' => Some(Token::Delimiter(DelimiterToken::SEMICOLON)),
            '(' => Some(Token::Delimiter(DelimiterToken::LEFT_PAREN)),
            ')' => Some(Token::Delimiter(DelimiterToken::RIGHT_PAREN)),
            '{' => Some(Token::Delimiter(DelimiterToken::LEFT_BRACE)),
            '}' => Some(Token::Delimiter(DelimiterToken::RIGHT_BRACE)),
            ',' => Some(Token::Delimiter(DelimiterToken::COMMA)),
            '.' => Some(Token::Delimiter(DelimiterToken::DOT)),
            '"' => {
                let token = self.parse_string();
                Some(token)
            }
            _ => {
                println!("ch: {:?}", ch);
                // check if keyword
                if ch.is_alphabetic() {
                    let start = self.current - 1;

                    while self.peek().is_alphabetic() {
                        self.read_char();
                    }

                    let plain = self.source.get(start..self.current).unwrap();

                    println!(
                        "plain: {:?}, start: {}, current: {}",
                        plain, start, self.current
                    );

                    if let Some(keyword) = is_keyword(plain) {
                        return Some(Token::Keyword(keyword));
                    }
                    // default is identifier
                    return Some(self.parse_identifier());
                }

                None
            }
        }
    }

    fn parse_string(&mut self) -> Token {
        let mut s = String::new();
        while self.peek() != '"' {
            println!("peek: {:?}", self.peek());
            if self.peek() == '\n' {
                self.line += 1;
            }
            if self.peek() == 0 as char {
                // TODO if at end of file, return error
                panic!("unexpected end of source");
            }
            s.push(self.read_char());
        }

        self.read_char(); // consume the closing '"'
        Token::Literal(LiteralToken::STRING(s))
    }

    /// read identifier
    fn parse_identifier(&mut self) -> Token {
        let _pos = self.current;

        // read until whitespace
        while self.peek().is_alphanumeric() {
            self.read_char();
        }

        let s = String::from(self.source.get(self.last..self.current).unwrap());
        Token::Ident(s)
    }

    /// lookup current char
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

/// check if string is a keyword
/// if yes, return Token.
/// if not, return None
fn is_keyword(s: &str) -> Option<KeywordToken> {
    match s {
        "let" => Some(KeywordToken::LET),
        "fn" => Some(KeywordToken::FN),
        "if" => Some(KeywordToken::IF),
        "else" => Some(KeywordToken::ELSE),
        "return" => Some(KeywordToken::RETURN),
        "true" => Some(KeywordToken::TRUE),
        "false" => Some(KeywordToken::FALSE),
        _ => None,
    }
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
    fn test_single_operators_tokens() {
        let source = "+-*/%=";

        let mut scanner = Scanner::new(source);
        let mut tokens = vec![];

        while let Some(token) = scanner.next_token() {
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

    #[test]
    fn test_parse_keyword() {
        let source = "let";
        let mut scanner = Scanner::new(source);
        let token = scanner.next_token().unwrap();
        assert_eq!(token, Token::Keyword(KeywordToken::LET));
    }

    #[test]
    fn test_simple_statement() {
        let source = "let a = \"lower case letter 'a'\";";

        let expected = vec![
            Token::Keyword(KeywordToken::LET),
            Token::Ident("a".to_string()),
            Token::Operator(OperatorToken::EQUAL),
            Token::Literal(LiteralToken::STRING("lower case letter 'a'".to_string())),
            Token::Delimiter(DelimiterToken::SEMICOLON),
        ];

        let mut scanner = Scanner::new(source);
        let mut tokens = vec![];

        while let Some(token) = scanner.next_token() {
            tokens.push(token);
        }

        assert_eq!(tokens, expected);
    }

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
