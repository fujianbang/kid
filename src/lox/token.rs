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
}

impl Token {
    pub fn new(
        kind: TokenType,
        lexeme: String,
        line: usize,
    ) -> Self {
        Self {
            kind,
            lexeme,
            line,
        }
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        format!("(Type: {:?}, Lexeme: {}, Line: {})", self.kind, self.lexeme, self.line)
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
        );
        assert_eq!(token.to_string(), String::from("(Type: LEFT_BRACE, Lexeme: { Line: 10)"))
    }
}