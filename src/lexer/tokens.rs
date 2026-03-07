use logos::Logos;

/// All tokens produced by the lexer.  New keywords or operators can be added here.
#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    // keywords
    #[token("package")]
    Package,
    #[token("import")]
    Import,
    #[token("proc")]
    Proc,
    #[token("use")]
    Use,
    #[token("return")]
    Return,
    #[token("assert")]
    Assert,
    #[token("distinct")]
    Distinct,

    // operators and punctuation
    #[token("+")]
    Add,
    #[token("-")]
    Sub,
    #[token("*")]
    Mul,
    #[token("/")]
    Div,

    #[token(":=")]
    ShortAssign,
    #[token("==")]
    Eq,
    #[token("=")]
    Assign,
    #[token("::")]
    ColonColon,
    #[token("->")]
    Arrow,
    #[token(":")]
    Colon,
    #[token(";")]
    Semicolon,
    #[token(",")]
    Comma,
    #[token(".")]
    Dot,

    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,

    #[token("&")]
    Ampersand,
    #[token("^")]
    Caret,

    // literal tokens
    #[regex(r"[0-9]+\.[0-9]+", |lex| lex.slice().parse::<f64>().ok())]
    Float(f64),

    #[regex(r"[0-9]+", |lex| lex.slice().parse::<i64>().ok())]
    Int(i64),
    
    #[regex(r#""([^"\\]|\\.)*""#, |lex| {
        let slice = lex.slice();
        Some(slice[1..slice.len()-1].to_string())
    })]
    StringLiteral(String),
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| { lex.slice().to_string() })]
    Ident(String),

    // comments and whitespace are skipped
    #[regex(r"//[^\n]*", logos::skip, allow_greedy = true)]
    Comment,
    #[regex(r"/\*([^*]|\*[^/])*\*/", logos::skip, allow_greedy = true)]
    BlockComment,
    #[regex(r"[ \t\n\r]+", logos::skip)]
    Whitespace,

    // fallback error token (logos derives this automatically)
    Error,
}
