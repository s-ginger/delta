use delta::*;

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(src: &str) -> Expr {
        let lexer = Lexer::new(src);
        let mut parser = Parser::new(lexer);
        parser.parse_expr()
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
                Expr::Unary { op: Op::AddressOf, expr } => match *expr {
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
            Expr::Binary { op: Op::Sub, left, right } => {
                match (*left, *right) {
                    (
                        Expr::Binary { op: Op::Add, left: l2, right: r2 },
                        Expr::Int(5)
                    ) => {
                        match (*l2, *r2) {
                            (Expr::Int(2), Expr::Binary { op: Op::Mul, left: l3, right: r3 }) => {
                                match (*l3, *r3) {
                                    (Expr::Int(3), Expr::Int(4)) => {}
                                    _ => panic!("expected 3*4"),
                                }
                            }
                            _ => panic!("expected 2+(3*4)"),
                        }
                    }
                    _ => panic!("expected (2+(3*4))-5"),
                }
            }
            _ => panic!("expected binary subtraction"),
        }
    }

    #[test]
    fn parse_parentheses() {
        let expr = parse("(2 + 3) * 4");
        // AST: ((2 + 3) * 4)
        match expr {
            Expr::Binary { op: Op::Mul, left, right } => {
                match (*left, *right) {
                    (
                        Expr::Binary { op: Op::Add, left: l2, right: r2 },
                        Expr::Int(4)
                    ) => {
                        match (*l2, *r2) {
                            (Expr::Int(2), Expr::Int(3)) => {}
                            _ => panic!("expected (2+3)"),
                        }
                    }
                    _ => panic!("expected (2+3)*4"),
                }
            }
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
                    Expr::Binary { op: Op::Add, left, right } => {
                        match (&**left, &**right) {
                            (Expr::Int(2), Expr::Int(3)) => {}
                            _ => panic!("expected 2+3"),
                        }
                    }
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
                    Expr::Call { func: g_func, args: g_args } => {
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
                    Expr::Call { func: h_func, args: h_args } => {
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
}