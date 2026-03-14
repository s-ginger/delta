#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Package,
    Import,
    Fn,
    Const,
    Var,
    Type,

    Return,
    Struct,
    Union,
    Using,

    If,
    Else,
    While,
    For,

    Add,
    Sub,
    Mul,
    Div,

    ShortAssign,
    Assign,

    Eq,        // ==
    Not,       // !
    NotEq,     // !=
    GreaterEq, // >=
    LessEq,    // <=
    Greater,   // >
    Less,      // <

    And, // &&
    Or,  // ||

    Colon,
    ColonColon,

    Arrow,
    Semicolon,

    Comma,
    Dot,

    LBrace,
    RBrace,

    LParen,
    RParen,

    LBracket,
    RBracket,

    Ampersand, // &
    Caret,     // ^

    Float(f64),
    Int(i64),
    StringLiteral(String),
    CharLiteral(char),

    Ident(String),

    Comment,
    BlockComment,

    NewLine,
    EndOfFile,
    Error,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}
