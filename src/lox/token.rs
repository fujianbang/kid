#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Debug)]
pub enum TokenType {
    /// (
    LEFT_PAREN,
    /// )
    RIGHT_PAREN,
    /// {
    LEFT_BRACE,
    /// }
    RIGHT_BRACE,
    /// ,
    COMMA,
    /// .
    DOT,
    /// -
    MINUS,
    /// +
    PLUS,
    /// ;
    SEMICOLON,
    /// /
    SLASH,
    /// *
    STAR,

    /// !
    BANG,
    /// !=
    BANG_EQUAL,
    /// =
    EQUAL,
    /// ==
    EQUAL_EQUAL,
    /// >
    GREATER,
    /// >=
    GREATER_EQUAL,
    /// <
    LESS,
    /// <=
    LESS_EQUAL,

    /// Literals
    IDENTIFIER,
    STRING,
    NUMBER,

    /// Keywords
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,
    EOF,
}

#[derive(Debug)]
pub struct Token {
    kind: TokenType,
    lexeme: String,
    line: usize,
    line_offset: usize,
}

impl Token {
    pub fn new(
        kind: TokenType,
        lexeme: String,
        line: usize,
        line_offset: usize,
    ) -> Self {
        Self {
            kind,
            lexeme,
            line,
            line_offset,
        }
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        format!("(Type: {:?}, Line: {}, Offset: {})", self.kind, self.line, self.line_offset)
    }
}

#[cfg(test)]
mod test_case {
    use super::*;

    #[test]
    fn test_to_string() {
        let token = Token::new(
            TokenType::LEFT_BRACE,
            String::from("{"),
            10,
            0,
        );
        assert_eq!(token.to_string(), String::from("(Type: LEFT_BRACE, Line: 10, Offset: 0)"))
    }
}