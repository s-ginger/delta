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

    pub fn parse_file(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();

        while self.cur.kind != TokenKind::EndOfFile {
            self.skip_newlines(); // пропускаем пустые строки
            if self.cur.kind == TokenKind::EndOfFile {
                break;
            }
            let stmt = self.parse_stmt();
            stmts.push(stmt);
        }

        stmts
    }

    pub fn parse_stmt(&mut self) -> Stmt {
        self.skip_newlines(); // пропуск пустых строк в начале
        match self.cur.kind.clone() {
            TokenKind::Var => {
                self.next();
                self.parse_var()
            }
            TokenKind::Const => {
                self.next();
                self.parse_const()
            }
            TokenKind::Fn => {
                self.next();
                self.parse_fn()
            }
            TokenKind::Ident(name) => {
                if self.peek.kind == TokenKind::ShortAssign {
                    self.next(); // съесть идентификатор
                    self.parse_short_assign(name)
                } else {
                    let expr = self.parse_expr();
                    Stmt::Expr(expr)
                }
            }
            TokenKind::Struct => self.parse_struct(),
            TokenKind::Union => self.parse_union(),
            TokenKind::Int(_)
            | TokenKind::Float(_)
            | TokenKind::StringLiteral(_)
            | TokenKind::CharLiteral(_) => {
                // литерал сам по себе — выражение
                let expr = self.parse_expr();
                Stmt::Expr(expr)
            }
            TokenKind::NewLine => {
                self.skip_newlines();
                self.parse_stmt() // рекурсивно пропускаем пустые строки
            }
            _ => panic!(
                "unexpected token {:?}:{:?}",
                self.cur.span.start, self.cur.span.end
            ),
        }
    }

    fn parse_var(&mut self) -> Stmt {
        let mut names = Vec::new();

        // собираем все идентификаторы, разделённые запятой
        loop {
            match self.cur.kind.clone() {
                TokenKind::Ident(n) => {
                    names.push(n);
                    self.next();
                }
                _ => panic!("expected identifier in var declaration"),
            }

            // если следующая запятая — съедаем и продолжаем
            if self.check(TokenKind::Comma) {
                self.next();
            } else {
                break;
            }
        }

        // теперь идёт тип или =?
        let mut ty = None;
        if matches!(
            self.cur.kind,
            TokenKind::Caret | TokenKind::LBracket | TokenKind::Ident(_)
        ) {
            ty = Some(self.parse_type());
        }

        // если есть =
        let mut value = None;
        if self.check(TokenKind::Assign) {
            self.next();
            value = Some(self.parse_expr());
        }

        Stmt::Decl(Box::new(Decl::Var { names, ty, value }))
    }

    fn parse_short_assign(&mut self, name: String) -> Stmt {
        self.expect(TokenKind::ShortAssign);

        let value = self.parse_expr();

        let ty = match &value {
            Expr::Int(_) => Some(Type::I32),
            Expr::Float(_) => Some(Type::F64),
            Expr::Str(_) => Some(Type::String),
            Expr::Char(_) => Some(Type::Char),
            _ => None,
        };

        Stmt::Decl(Box::new(Decl::Var {
            names: vec![name],
            ty,
            value: Some(value),
        }))
    }

    fn parse_const(&mut self) -> Stmt {
        // Получаем имя константы
        let name = if let TokenKind::Ident(n) = self.cur.kind.clone() {
            self.next();
            n
        } else {
            panic!("expected identifier in const declaration");
        };

        // Опциональный тип
        let ty = if matches!(
            self.cur.kind,
            TokenKind::Caret | TokenKind::LBracket | TokenKind::Ident(_)
        ) {
            Some(self.parse_type())
        } else {
            None
        };

        // Должен быть =
        self.expect(TokenKind::Assign);

        // Значение
        let value = self.parse_expr();

        Stmt::Decl(Box::new(Decl::Const { name, ty, value }))
    }

    fn parse_fn(&mut self) -> Stmt {
        // Имя функции
        let name = if let TokenKind::Ident(n) = self.cur.kind.clone() {
            self.next();
            n
        } else {
            panic!("expected function name");
        };

        // Параметры функции
        let mut params = Vec::new();
        self.expect(TokenKind::LParen); // '('
        if !self.check(TokenKind::RParen) {
            loop {
                // Имя параметра
                let param_name = if let TokenKind::Ident(n) = self.cur.kind.clone() {
                    self.next();
                    n
                } else {
                    panic!("expected parameter name");
                };

                // Тип параметра
                let param_type = self.parse_type();

                params.push(Field {
                    name: param_name,
                    ty: param_type,
                });

                if self.check(TokenKind::Comma) {
                    self.next();
                } else {
                    break;
                }
            }
        }
        self.expect(TokenKind::RParen); // ')'

        // Возвращаемые типы
        let mut returns = Vec::new();
        if self.check(TokenKind::LParen) {
            self.next(); // '('
            loop {
                returns.push(self.parse_type());
                if self.check(TokenKind::Comma) {
                    self.next();
                } else {
                    break;
                }
            }
            self.expect(TokenKind::RParen); // ')'
        } else if matches!(self.cur.kind, TokenKind::Ident(_)) {
            returns.push(self.parse_type());
        }

        // Тело функции
        let body = self.parse_block();

        Stmt::Decl(Box::new(Decl::Func {
            name,
            params,
            returns,
            body,
        }))
    }

    fn parse_block(&mut self) -> Stmt {
        self.expect(TokenKind::LBrace); // съесть '{'

        let mut stmts = Vec::new();

        while !self.check(TokenKind::RBrace) {
            self.skip_newlines(); // пропускаем пустые строки/разделители
            if self.check(TokenKind::RBrace) {
                break; // конец блока
            }
            let stmt = self.parse_stmt();
            stmts.push(stmt);
            self.skip_newlines();
        }

        self.expect(TokenKind::RBrace); // съесть '}'

        Stmt::Block(stmts)
    }

    pub fn parse_union(&mut self) -> Stmt {
        // 1. Съедаем ключевое слово `union`
        self.expect(TokenKind::Union);

        // 2. Имя объединения
        let name = if let TokenKind::Ident(n) = self.cur.kind.clone() {
            self.next();
            n
        } else {
            panic!("expected union name");
        };

        // 3. Ожидаем '{'
        self.expect(TokenKind::LBrace);

        // 4. Пропускаем пустые строки перед полями
        self.skip_newlines();

        // 5. Парсим поля union
        let fields = self.parse_fields();

        // 6. Пропускаем пустые строки после полей
        self.skip_newlines();

        // 7. Ожидаем '}'
        self.expect(TokenKind::RBrace);

        // 8. Возвращаем как объявление union
        Stmt::Decl(Box::new(Decl::Union { name, fields }))
    }

    pub fn parse_struct(&mut self) -> Stmt {
        // 1. Съедаем ключевое слово `struct`
        self.expect(TokenKind::Struct);

        // 2. Имя структуры
        let name = if let TokenKind::Ident(n) = self.cur.kind.clone() {
            self.next();
            n
        } else {
            panic!("expected struct name");
        };

        // 3. Ожидаем '{'
        self.expect(TokenKind::LBrace);

        // 4. Пропускаем пустые строки перед полями
        self.skip_newlines();

        // 5. Парсим поля структуры
        let fields = self.parse_fields();

        // 6. Пропускаем пустые строки после полей
        self.skip_newlines();

        // 7. Ожидаем '}'
        self.expect(TokenKind::RBrace);

        // 8. Возвращаем как объявление структуры
        Stmt::Decl(Box::new(Decl::Struct { name, fields }))
    }

    pub fn parse_fields(&mut self) -> Vec<Field> {
        let mut fields = Vec::new();

        while let TokenKind::Ident(name) = self.cur.kind.clone() {
            self.next(); // съесть имя поля
            let ty = self.parse_type(); // читаем тип
            fields.push(Field { name, ty });

            // Пропускаем новые строки между полями, если есть
            self.skip_newlines();
        }

        fields
    }

    pub fn parse_type(&mut self) -> Type {
        match self.cur.kind.clone() {
            // Указатель ^Type
            TokenKind::Caret => {
                self.next();
                let inner = self.parse_type();
                Type::Ptr(Box::new(inner))
            }

            // Массив или срез [N]Type или []Type
            TokenKind::LBracket => {
                self.next(); // съесть '['
                let size: usize = match self.cur.kind.clone() {
                    TokenKind::Int(n) => {
                        self.next(); // съесть число
                        n as usize // безопасное преобразование
                    }
                    _ => panic!("expected array size"), // срез []
                };
                self.expect(TokenKind::RBracket); // съесть ']'
                let inner = self.parse_type();
                Type::Array(size, Box::new(inner))
            }

            // Идентификатор типа
            TokenKind::Ident(name) => {
                self.next();
                Type::from_str(&name)
            }

            _ => panic!(
                "unexpected token in type {:?} {:?}:{:?}",
                self.cur.kind, self.cur.span.start, self.cur.span.end
            ),
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
