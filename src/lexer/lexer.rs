use crate::Token;
use logos::Logos;

/// A simple wrapper around `logos::Lexer` providing an iterator of tokens.
pub struct Lexer<'a> {
    inner: logos::Lexer<'a, Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Lexer { inner: Token::lexer(source) }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(result) = self.inner.next() {
            match result {
                Ok(tok) => return Some(tok),
                Err(_) => continue,
            }
        }
        None
    }
}

/// Convenience function that lexes the entire source and returns a Vec<Token>.
pub fn tokenize(source: &str) -> Vec<Token> {
    Lexer::new(source).collect()
}
