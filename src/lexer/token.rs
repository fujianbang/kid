#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub(super) enum Token {
    ILLEGAL,
    Keyword(KeywordToken),
    Delimiter(DelimiterToken),
    Operator(OperatorToken),
    IDENTIFIER(String),
    Literal(LiteralToken),
}

#[allow(non_camel_case_types)]
pub(super) enum KeywordToken {
    LET,
    FN,
    IF,
    ELSE,
    RETURN,
    TRUE,
    FALSE,
    EOF,
}

#[allow(non_camel_case_types)]
pub(super) enum DelimiterToken {
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
pub(super) enum OperatorToken {
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

pub(super) enum LiteralToken {
    INT(i64),
    FLOAT(f64),
    STRING(String),
    BOOLEAN(bool),
}
