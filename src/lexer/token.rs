#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Ident(String),
    Keyword(KeywordToken),
    Delimiter(DelimiterToken),
    Operator(OperatorToken),
    Literal(LiteralToken),
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub enum KeywordToken {
    LET,
    FN,
    IF,
    ELSE,
    RETURN,
    TRUE,
    FALSE,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub enum DelimiterToken {
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
    /// ;
    SEMICOLON,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub enum OperatorToken {
    /// +
    Add,
    /// -
    Sub,
    /// *
    STAR,
    /// /
    SLASH,
    /// %
    REM,
    /// &
    AND,
    /// |
    OR,
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
}

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralToken {
    INT(i64),
    FLOAT(f64),
    STRING(String),
    BOOLEAN(bool),
}
