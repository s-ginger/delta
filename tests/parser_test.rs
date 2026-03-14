use delta::*;

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(src: &str) -> Expr {
        let lexer = Lexer::new(src);
        let mut parser = Parser::new(lexer);
        parser.parse_expr()
    }

    fn parse_stmt(src: &str) -> Stmt {
        let lexer = Lexer::new(src);
        let mut parser = Parser::new(lexer);
        parser.parse_stmt()
    }

    fn parse_type(src: &str) -> Type {
        let lexer = Lexer::new(src);
        let mut parser = Parser::new(lexer);
        parser.parse_type()
    }

    #[test]
    fn parse_int() {
        let expr = parse("42");
        match expr {
            Expr::Int(v) => assert_eq!(v, 42),
            _ => panic!("expected Expr::Int"),
        }
    }

    #[test]
    fn parse_float() {
        let expr = parse("3.14");
        match expr {
            Expr::Float(f) => assert_eq!(f, 3.14),
            _ => panic!("expected Expr::Float"),
        }
    }

    #[test]
    fn parse_ident() {
        let expr = parse("x");
        match expr {
            Expr::Ident(name) => assert_eq!(name, "x"),
            _ => panic!("expected Expr::Ident"),
        }
    }

    #[test]
    fn parse_unary_minus() {
        let expr = parse("-5");
        match expr {
            Expr::Unary { op: Op::Sub, expr } => match *expr {
                Expr::Int(v) => assert_eq!(v, 5),
                _ => panic!("expected inner Expr::Int"),
            },
            _ => panic!("expected Expr::Unary"),
        }
    }

    #[test]
    fn parse_unary_plus_and_address() {
        let expr = parse("+&x");
        match expr {
            Expr::Unary { op: Op::Add, expr } => match *expr {
                Expr::Unary {
                    op: Op::AddressOf,
                    expr,
                } => match *expr {
                    Expr::Ident(name) => assert_eq!(name, "x"),
                    _ => panic!("expected Expr::Ident"),
                },
                _ => panic!("expected Expr::Unary AddressOf"),
            },
            _ => panic!("expected Expr::Unary Add"),
        }
    }

    #[test]
    fn parse_binary_operations() {
        let expr = parse("2 + 3 * 4 - 5");
        // AST будет: ((2 + (3 * 4)) - 5)
        match expr {
            Expr::Binary {
                op: Op::Sub,
                left,
                right,
            } => match (*left, *right) {
                (
                    Expr::Binary {
                        op: Op::Add,
                        left: l2,
                        right: r2,
                    },
                    Expr::Int(5),
                ) => match (*l2, *r2) {
                    (
                        Expr::Int(2),
                        Expr::Binary {
                            op: Op::Mul,
                            left: l3,
                            right: r3,
                        },
                    ) => match (*l3, *r3) {
                        (Expr::Int(3), Expr::Int(4)) => {}
                        _ => panic!("expected 3*4"),
                    },
                    _ => panic!("expected 2+(3*4)"),
                },
                _ => panic!("expected (2+(3*4))-5"),
            },
            _ => panic!("expected binary subtraction"),
        }
    }

    #[test]
    fn parse_parentheses() {
        let expr = parse("(2 + 3) * 4");
        // AST: ((2 + 3) * 4)
        match expr {
            Expr::Binary {
                op: Op::Mul,
                left,
                right,
            } => match (*left, *right) {
                (
                    Expr::Binary {
                        op: Op::Add,
                        left: l2,
                        right: r2,
                    },
                    Expr::Int(4),
                ) => match (*l2, *r2) {
                    (Expr::Int(2), Expr::Int(3)) => {}
                    _ => panic!("expected (2+3)"),
                },
                _ => panic!("expected (2+3)*4"),
            },
            _ => panic!("expected binary multiplication"),
        }
    }

    #[test]
    fn parse_function_call_no_args() {
        let expr = parse("f()");
        match expr {
            Expr::Call { func, args } => {
                match *func {
                    Expr::Ident(name) => assert_eq!(name, "f"),
                    _ => panic!("expected ident f"),
                }
                assert_eq!(args.len(), 0);
            }
            _ => panic!("expected Expr::Call"),
        }
    }

    #[test]
    fn parse_function_call_with_args() {
        let expr = parse("sum(1, 2 + 3, x)");
        match expr {
            Expr::Call { func, args } => {
                match *func {
                    Expr::Ident(name) => assert_eq!(name, "sum"),
                    _ => panic!("expected ident sum"),
                }
                assert_eq!(args.len(), 3);

                // 1
                match &args[0] {
                    Expr::Int(v) => assert_eq!(*v, 1),
                    _ => panic!("expected 1"),
                }
                // 2+3
                match &args[1] {
                    Expr::Binary {
                        op: Op::Add,
                        left,
                        right,
                    } => match (&**left, &**right) {
                        (Expr::Int(2), Expr::Int(3)) => {}
                        _ => panic!("expected 2+3"),
                    },
                    _ => panic!("expected 2+3"),
                }
                // x
                match &args[2] {
                    Expr::Ident(name) => assert_eq!(name, "x"),
                    _ => panic!("expected x"),
                }
            }
            _ => panic!("expected Expr::Call"),
        }
    }

    #[test]
    fn parse_nested_call() {
        let expr = parse("f(g(1), h(x))");
        match expr {
            Expr::Call { func, args } => {
                match *func {
                    Expr::Ident(name) => assert_eq!(name, "f"),
                    _ => panic!("expected ident f"),
                }
                assert_eq!(args.len(), 2);

                // g(1)
                match &args[0] {
                    Expr::Call {
                        func: g_func,
                        args: g_args,
                    } => {
                        match **g_func {
                            Expr::Ident(ref n) => assert_eq!(n, "g"),
                            _ => panic!("expected g"),
                        }
                        assert_eq!(g_args.len(), 1);
                        match g_args[0] {
                            Expr::Int(1) => {}
                            _ => panic!("expected 1"),
                        }
                    }
                    _ => panic!("expected g(1)"),
                }

                // h(x)
                match &args[1] {
                    Expr::Call {
                        func: h_func,
                        args: h_args,
                    } => {
                        match **h_func {
                            Expr::Ident(ref n) => assert_eq!(n, "h"),
                            _ => panic!("expected h"),
                        }
                        assert_eq!(h_args.len(), 1);
                        match h_args[0] {
                            Expr::Ident(ref n) => assert_eq!(n, "x"),
                            _ => panic!("expected x"),
                        }
                    }
                    _ => panic!("expected h(x)"),
                }
            }
            _ => panic!("expected f(...)"),
        }
    }

    #[test]
    fn parse_import_package_stmt() {
        let stmt = parse_stmt("package main");
        match stmt {
            Stmt::Package(name) => {
                if name != "main" {
                    panic!("unexpected package name")
                }
            }
            _ => panic!("stmt is null"),
        }

        let stmt2 = parse_stmt("import \"main\"");
        match stmt2 {
            Stmt::Import(path) => {
                if path != "main" {
                    panic!("unexpected import name")
                }
            }
            _ => panic!("stmt is null"),
        }
    }

    #[test]
    fn test_parse_type_array_pointer() {
        // Парсим тип
        let ty = parse_type("[5]^i8");

        // Ожидаемый AST
        let expected = Type::Array(5, Box::new(Type::Ptr(Box::new(Type::I8))));

        println!("{:?}", ty);
        // Сравниваем
        assert_eq!(format!("{:?}", ty), format!("{:?}", expected));
    }

    #[test]
    fn test_decl() {
        let stmt = parse_stmt("m:^i8 = 1");

        match stmt {
            Stmt::Decl(decl_box) => {
                let decl = *decl_box; // распаковали Box на stack
                match decl {
                    Decl::Var { names, ty, value } => {
                        assert_eq!(names.len(), 1);
                        assert_eq!(names[0], "m");

                        // проверяем тип
                        if let Some(Type::Ptr(inner)) = ty {
                            // inner: &Box<Type> если ty: &Option<Type>, или Box<Type> если ty: Option<Type>
                            // Можно распаковать
                            match *inner {
                                Type::I8 => println!("it's a pointer to i8"),
                                _ => panic!("expected pointer to i8"),
                            }
                        } else {
                            panic!("expected Some(Type::Ptr)");
                        }

                        // проверяем значение
                        match value {
                            Some(Expr::Int(v)) => assert_eq!(v, 1),
                            _ => panic!("expected Expr::Int(1)"),
                        }
                    }
                    _ => unreachable!(),
                }
            }
            _ => panic!("expected Stmt::Decl"),
        }
    }

    #[test]
    fn test_expr() {
        let stmt = parse_stmt("1 + 2");

        match stmt {
            Stmt::Expr(Expr::Binary { left, op, right }) => {
                // Проверяем левую часть
                match *left {
                    Expr::Int(val) => assert_eq!(val, 1),
                    _ => panic!("Левый операнд должен быть числом"),
                }

                // Проверяем оператор
                assert_eq!(op, Op::Add);

                // Проверяем правую часть
                match *right {
                    Expr::Int(val) => assert_eq!(val, 2),
                    _ => panic!("Правый операнд должен быть числом"),
                }
            }
            _ => panic!("Должно быть бинарное выражение"),
        }
    }

    #[test]
    fn test_short_assign() {
        let stmt = parse_stmt("m := 12");

        match stmt {
            Stmt::Decl(box_) => {
                match *box_ {
                    Decl::Var { names, ty, value } => {
                        assert_eq!(names.len(), 1);
                        assert_eq!(names[0], "m");

                        if let Some(Type::I32) = ty {
                        } else {
                            panic!("expected Some(Type::Ptr)");
                        }

                        // проверяем значение
                        match value {
                            Some(Expr::Int(v)) => assert_eq!(v, 12),
                            _ => panic!("expected Expr::Int(12)"),
                        }
                    }
                    _ => panic!(),
                }
            }
            _ => panic!(),
        }
    }
}
