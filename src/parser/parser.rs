use crate::*;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    cur: Token,
    peek: Token,
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Self {
        let cur = lexer.next_token();
        let peek = lexer.next_token();

        Self { lexer, cur, peek }
    }

    fn next(&mut self) {
        self.cur = std::mem::replace(&mut self.peek, self.lexer.next_token());
    }

    fn check(&self, kind: TokenKind) -> bool {
        self.cur.kind == kind
    }

    fn expect(&mut self, kind: TokenKind) {
        if self.cur.kind != kind {
            panic!("expected {:?}, got {:?}", kind, self.cur.kind);
        }
        self.next();
    }

    /// Новые строки пропускаем
    fn skip_newlines(&mut self) {
        while self.cur.kind == TokenKind::NewLine {
            self.next();
        }
    }

    pub fn parse_unary(&mut self) -> Expr {
        match self.cur.kind {
            TokenKind::Add => {
                self.next();
                let expr = self.parse_unary();
                Expr::Unary {
                    op: Op::Add,
                    expr: Box::new(expr),
                }
            }

            TokenKind::Sub => {
                self.next();
                let expr = self.parse_unary();
                Expr::Unary {
                    op: Op::Sub,
                    expr: Box::new(expr),
                }
            }

            TokenKind::Ampersand => {
                self.next();
                let expr = self.parse_unary();
                Expr::Unary {
                    op: Op::AddressOf,
                    expr: Box::new(expr),
                }
            }

            TokenKind::Caret => {
                self.next();
                let expr = self.parse_unary();
                Expr::Unary {
                    op: Op::Deref,
                    expr: Box::new(expr),
                }
            }

            _ => self.parse_primary(),
        }
    }

    pub fn parse_primary(&mut self) -> Expr {
        match self.cur.kind.clone() {
            TokenKind::Ident(name) => Expr::Ident(name),
            TokenKind::Int(i) => Expr::Int(i),
            TokenKind::Float(f) => Expr::Float(f),
            TokenKind::StringLiteral(string) => Expr::Str(string),
            TokenKind::CharLiteral(char) => Expr::Char(char),
            _ => panic!("unexpected token \n {:?}:{:?}", self.cur.span.start, self.cur.span.end),
        }
    }
}
