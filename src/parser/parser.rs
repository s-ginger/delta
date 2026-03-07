use crate::{Define, Expr, OP, Stmt, Token};

pub struct Parser {
    pub tokens: Vec<Token>, // динамический массив токенов
    pub pos: usize,         // позиция лучше делать usize
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    pub fn current(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    pub fn next(&self) -> Option<&Token> {
        self.tokens.get(self.pos + 1)
    }

    pub fn advance(&mut self) {
        self.pos += 1;
    }

    pub fn parse_stmts(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();

        while let Some(tok) = self.current() {
            match tok {
                Token::RBrace => break, // конец блока
                Token::Package | Token::Import | Token::LBrace => {
                    let stmt = self.parse_stmt();
                    stmts.push(stmt);
                }
                Token::Ident(_) => {
                    let stmt = self.parse_ident_variant();
                    stmts.push(stmt);
                }
                Token::Float(_)
                | Token::Int(_)
                | Token::StringLiteral(_)
                | Token::Sub
                | Token::Ampersand
                | Token::Caret
                | Token::Add => {
                    let expr = self.parse_expr();
                    stmts.push(Stmt::StmtExpr(expr));
                }
                _ => panic!("unexpected token in parse_stmts: {:?}", tok),
            }

            self.advance(); // обязательно двигаем позицию после обработки
        }

        stmts
    }

    fn parse_ident_variant(&mut self) -> Stmt {
        let name = match self.current() {
            Some(Token::Ident(name)) => name.clone(),
            Some(tok) => panic!("expected identifier, found {:?}", tok),
            None => panic!("unexpected EOF"),
        };

        self.advance(); // пропускаем Ident

        match self.current() {
            Some(Token::ShortAssign) => {
                self.advance(); // пропускаем :=

                let expr = self.parse_expr();

                Stmt::Define(Box::new(Define::ShortAssign { name, value: expr }))
            }
            Some(Token::Assign) => {
                self.advance();
                let expr = self.parse_expr();
                Stmt::Define(Box::new(Define::Assign { name: name, value: expr }))
            },
            
            Some(tok) => panic!("unexpected token after identifier: {:?}", tok),
            None => panic!("unexpected EOF"),
        }
    }

    pub fn parse_stmt(&mut self) -> Stmt {
        match self.current() {
            Some(Token::Package) => self.parse_package(),
            Some(Token::Import) => self.parse_import(),
            Some(Token::LBrace) => self.parse_block(),
            Some(tk) => panic!("unexpected token {:?}", tk),
            None => panic!("invalid"),
        }
    }

    fn parse_package(&mut self) -> Stmt {
        self.advance();
        match self.current() {
            Some(Token::Ident(name)) => Stmt::Package(name.to_string()),
            Some(tk) => panic!("unexpected token {:?}", tk),
            None => panic!("invalid"),
        }
    }

    fn parse_import(&mut self) -> Stmt {
        self.advance();
        match self.current() {
            Some(Token::StringLiteral(name)) => Stmt::Import(name.to_string()),
            Some(tk) => panic!("unexpected token {:?}", tk),
            None => panic!("invalid"),
        }
    }

    fn parse_block(&mut self) -> Stmt {
        self.advance();
        let stmts = self.parse_stmts();
        Stmt::Block(stmts)
    }

    pub fn parse_expr(&mut self) -> Expr {
        self.parse_binary_expr(0)
    }

    // приоритет операторов
    fn get_precedence(op: &OP) -> u8 {
        match op {
            OP::Mul | OP::Div => 2,
            OP::Plus | OP::Minus => 1,
            _ => 0,
        }
    }

    // парсинг бинарного выражения с рекурсией по приоритету
    fn parse_binary_expr(&mut self, min_prec: u8) -> Expr {
        let mut left = self.parse_unary();

        while let Some(op) = self.current_op() {
            let prec = Self::get_precedence(&op);
            if prec < min_prec {
                break;
            }
            self.advance(); // пропускаем оператор
            let right = self.parse_binary_expr(prec + 1); // рекурсивно
            left = Expr::BinOp {
                left: Box::new(left),
                right: Box::new(right),
                op,
            };
        }

        left
    }

    // парсинг унарного выражения
    fn parse_unary(&mut self) -> Expr {
        match self.current() {
            Some(Token::Add) => {
                self.advance();
                Expr::UnaryOP {
                    op: OP::Plus,
                    value: Box::new(self.parse_unary()),
                }
            }
            Some(Token::Sub) => {
                self.advance();
                Expr::UnaryOP {
                    op: OP::Minus,
                    value: Box::new(self.parse_unary()),
                }
            }
            Some(Token::Ampersand) => {
                self.advance();
                Expr::UnaryOP {
                    op: OP::AddressOf,
                    value: Box::new(self.parse_unary()),
                }
            }
            Some(Token::Caret) => {
                self.advance();
                Expr::UnaryOP {
                    op: OP::Ref,
                    value: Box::new(self.parse_unary()),
                }
            }
            _ => self.parse_primary(),
        }
    }

    // парсинг первичных выражений
    fn parse_primary(&mut self) -> Expr {
        match self.current() {
            Some(Token::Int(n)) => {
                let v = *n;
                self.advance();
                Expr::Number(v)
            }
            Some(Token::Float(f)) => {
                let v = *f;
                self.advance();
                Expr::Float(v)
            }
            Some(Token::Ident(name)) => {
                let n = name.clone();
                self.advance();
                Expr::Ident(n)
            }
            Some(Token::StringLiteral(s)) => {
                let v = s.clone();
                self.advance();
                Expr::Str(v)
            }
            Some(Token::LParen) => {
                self.advance();
                let expr = self.parse_expr();
                match self.current() {
                    Some(Token::RParen) => {
                        self.advance();
                        expr
                    }
                    Some(tok) => panic!("Expected ), found {:?}", tok),
                    None => panic!("Expected ), found EOF"),
                }
            }
            Some(tok) => panic!("Unexpected token in primary: {:?}", tok),
            None => panic!("Unexpected end of input"),
        }
    }

    fn current_op(&self) -> Option<OP> {
        match self.current() {
            Some(Token::Add) => Some(OP::Plus),
            Some(Token::Sub) => Some(OP::Minus),
            Some(Token::Mul) => Some(OP::Mul),
            Some(Token::Div) => Some(OP::Div),
            _ => None,
        }
    }
}
