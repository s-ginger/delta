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
    #[allow(dead_code)]
    fn skip_newlines(&mut self) {
        while self.cur.kind == TokenKind::NewLine {
            self.next();
        }
    }

    pub fn parse_expr(&mut self) -> Expr {
        let mut expr = self.parse_term();

        while matches!(self.cur.kind, TokenKind::Add | TokenKind::Sub) {
            let op = match self.cur.kind {
                TokenKind::Add => Op::Add,
                TokenKind::Sub => Op::Sub,
                _ => unreachable!(),
            };

            self.next(); // съесть оператор
            let right = self.parse_term(); // правый операнд
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        expr
    }

    pub fn parse_term(&mut self) -> Expr {
        // сначала левый операнд
        let mut expr = self.parse_unary();

        // пока текущий токен * или /
        while matches!(self.cur.kind, TokenKind::Mul | TokenKind::Div) {
            let op = match self.cur.kind {
                TokenKind::Mul => Op::Mul,
                TokenKind::Div => Op::Div,
                _ => unreachable!(),
            };

            self.next(); // съесть оператор
            let right = self.parse_unary(); // правый операнд
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        expr
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

            _ => self.parse_call(),
        }
    }

    pub fn parse_call(&mut self) -> Expr {
        // сначала парсим primary
        let mut expr = self.parse_primary();

        // пока идёт '(' — это вызов функции
        while self.check(TokenKind::LParen) {
            self.next(); // съесть '('
            let mut args = Vec::new();

            if !self.check(TokenKind::RParen) {
                loop {
                    args.push(self.parse_expr()); // аргумент — любое выражение
                    if self.check(TokenKind::Comma) {
                        self.next(); // съесть ','
                    } else {
                        break;
                    }
                }
            }

            self.expect(TokenKind::RParen); // ожидание ')'

            expr = Expr::Call {
                func: Box::new(expr),
                args,
            };
        }

        expr
    }

    pub fn parse_primary(&mut self) -> Expr {
        match self.cur.kind.clone() {
            TokenKind::Ident(name) => {
                self.next();
                Expr::Ident(name)
            }
            TokenKind::Int(i) => {
                self.next();
                Expr::Int(i)
            }
            TokenKind::Float(f) => {
                self.next();
                Expr::Float(f)
            }
            TokenKind::StringLiteral(s) => {
                self.next();
                Expr::Str(s)
            }
            TokenKind::CharLiteral(c) => {
                self.next();
                Expr::Char(c)
            }
            TokenKind::LParen => {
                self.next(); // съесть '('
                let expr = self.parse_expr();
                self.expect(TokenKind::RParen); // съесть ')'
                expr // НЕ вызываем self.next() здесь
            }
            _ => panic!(
                "unexpected token {:?}:{:?}",
                self.cur.span.start, self.cur.span.end
            ),
        }
    }
}
