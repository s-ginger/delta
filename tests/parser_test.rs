use delta::*;

#[cfg(test)]
mod tests {
    use super::*;

    // Вспомогательная функция для создания Parser и вызова parse_stmt
    fn parse_stmt_from(src: &str) -> Stmt {
        let lexer = Lexer::new(src);
        let mut parser = Parser::new(lexer);
        parser.parse_stmt()
    }

    #[test]
    fn test_var_single() {
        let stmt = parse_stmt_from("var a i32");

        if let Stmt::Decl(decl) = stmt {
            let decl = decl.as_ref(); // разыменовываем Box<Decl>
            if let Decl::Var { names, ty, value } = decl {
                assert_eq!(names, &vec!["a"]);
                assert_eq!(ty, &Some(Type::I32));
                assert!(value.is_none());
            } else {
                panic!("expected Decl::Var");
            }
        } else {
            panic!("expected Stmt::Decl");
        }
    }

    #[test]
    fn test_var_multiple() {
        let stmt = parse_stmt_from("var a, b, c f64");

        if let Stmt::Decl(decl) = stmt {
            let decl = decl.as_ref();
            if let Decl::Var { names, ty, value } = decl {
                assert_eq!(names, &vec!["a", "b", "c"]);
                assert_eq!(ty, &Some(Type::F64));
                assert!(value.is_none());
            } else {
                panic!("expected Decl::Var");
            }
        } else {
            panic!("expected Stmt::Decl");
        }
    }

    #[test]
    fn test_var_with_value() {
        let stmt = parse_stmt_from("var x = 42");

        if let Stmt::Decl(decl) = stmt {
            let decl = decl.as_ref();
            if let Decl::Var { names, ty, value } = decl {
                assert_eq!(names, &vec!["x"]);
                assert!(ty.is_none());
                if let Some(Expr::Int(v)) = value {
                    assert_eq!(*v, 42);
                } else {
                    panic!("expected int value");
                }
            } else {
                panic!("expected Decl::Var");
            }
        } else {
            panic!("expected Stmt::Decl");
        }
    }

    #[test]
    fn test_const_with_value() {
        let stmt = parse_stmt_from("const x = 42");

        if let Stmt::Decl(decl) = stmt {
            let decl = decl.as_ref();
            if let Decl::Const { name, ty, value } = decl {
                assert_eq!(name, "x");
                assert!(ty.is_none());
                if let Expr::Int(v) = value {
                    assert_eq!(*v, 42);
                } else {
                    panic!("expected int value");
                }
            } else {
                panic!("expected Decl::Var");
            }
        } else {
            panic!("expected Stmt::Decl");
        }
    }

    #[test]
    fn test_expr_simple() {
        let stmt = parse_stmt_from("1 + 2 * 3");

        if let Stmt::Expr(expr) = stmt {
            if let Expr::Binary { op, left, right } = expr {
                assert_eq!(op, Op::Add);

                if let Expr::Int(v) = *left {
                    assert_eq!(v, 1);
                } else {
                    panic!("expected left int");
                }

                if let Expr::Binary {
                    op: op2,
                    left: l2,
                    right: r2,
                } = *right
                {
                    assert_eq!(op2, Op::Mul);
                    if let Expr::Int(v) = *l2 {
                        assert_eq!(v, 2);
                    } else {
                        panic!();
                    }
                    if let Expr::Int(v) = *r2 {
                        assert_eq!(v, 3);
                    } else {
                        panic!();
                    }
                } else {
                    panic!("expected right binary");
                }
            } else {
                panic!("expected binary expression");
            }
        } else {
            panic!("expected Stmt::Expr");
        }
    }

    #[test]
    fn test_call() {
        let stmt = parse_stmt_from("print(1, 2 + 3)");

        if let Stmt::Expr(expr) = stmt {
            if let Expr::Call { func, args } = expr {
                if let Expr::Ident(name) = *func {
                    assert_eq!(name, "print");
                } else {
                    panic!("expected function ident");
                }

                assert_eq!(args.len(), 2);

                // Можно дополнительно проверить аргументы
                if let Expr::Int(v) = args[0] {
                    assert_eq!(v, 1);
                } else {
                    panic!("expected first arg int");
                }

                if let Expr::Binary { op, left, right } = &args[1] {
                    assert_eq!(*op, Op::Add);
                    if let Expr::Int(v) = **left {
                        assert_eq!(v, 2);
                    } else {
                        panic!();
                    }
                    if let Expr::Int(v) = **right {
                        assert_eq!(v, 3);
                    } else {
                        panic!();
                    }
                } else {
                    panic!("expected second arg binary");
                }
            } else {
                panic!("expected call expression");
            }
        } else {
            panic!("expected Stmt::Expr");
        }
    }

    #[test]
    fn test_short_assign() {
        let stmt = parse_stmt_from("a := 12");
        println!("{:?}", stmt);
    }

    #[test]
    fn test_parse_fn_simple() {
        let code = "
        fn main(a i32, b i32) i32 {
            var x i32 = 10
        }
        ";

        let lexer = Lexer::new(code);
        let mut parser = Parser::new(lexer);

        let stmt = parser.parse_stmt();
        println!("{:?}", stmt);
        match stmt {
            Stmt::Decl(decl) => {
                match *decl {
                    Decl::Func {
                        ref name,
                        ref params,
                        ref returns,
                        ref body,
                    } => {
                        assert_eq!(name, "main");
                        assert_eq!(params.len(), 2);
                        assert_eq!(params[0].name, "a");
                        assert_eq!(params[0].ty, Type::I32);
                        assert_eq!(params[1].name, "b");
                        assert_eq!(params[1].ty, Type::I32);

                        assert_eq!(returns.len(), 1);
                        assert_eq!(returns[0], Type::I32);

                        // Проверяем тело
                        match body {
                            Stmt::Block(stmts) => {
                                assert_eq!(stmts.len(), 1);
                                match &stmts[0] {
                                    Stmt::Decl(var_decl) => match **var_decl {
                                        Decl::Var {
                                            ref names,
                                            ref ty,
                                            ref value,
                                        } => {
                                            assert_eq!(names, &vec!["x".to_string()]);
                                            assert_eq!(ty.as_ref().unwrap(), &Type::I32);
                                            match value {
                                                Some(Expr::Int(i)) => assert_eq!(*i, 10),
                                                _ => panic!("expected Int(10)"),
                                            }
                                        }
                                        _ => panic!("expected Var decl"),
                                    },
                                    _ => panic!("expected Var statement"),
                                }
                            }
                            _ => panic!("expected Block"),
                        }
                    }
                    _ => panic!("expected Func declaration"),
                };
            }
            _ => panic!("expected Decl statement"),
        }
    }

    #[test]
    fn test_parse_struct() {
        // Пример кода
        let code = r#"
        struct Point {
            x I32
            y I32
        }
        "#;

        // Создаём лексер и парсер
        let lexer = Lexer::new(code);
        let mut parser = Parser::new(lexer);

        // Парсим struct
        let stmt = parser.parse_stmt();
        if let Stmt::Decl(decl) = stmt {
            match *decl {
                Decl::Struct {
                    ref name,
                    ref fields,
                } => {
                    assert_eq!(name, "Point");
                    assert_eq!(fields.len(), 2);
                    assert_eq!(fields[0].name, "x");
                    assert_eq!(fields[1].name, "y");
                }
                _ => panic!("expected struct declaration"),
            }
        } else {
            panic!("expected declaration stmt");
        }
    }

    #[test]
    fn test_parse_union() {
        // Пример кода
        let code = r#"
        union Value {
            i I32
            f F64
        }
        "#;

        // Создаём лексер и парсер
        let lexer = Lexer::new(code);
        let mut parser = Parser::new(lexer);

        // Парсим union
        let stmt = parser.parse_stmt();
        if let Stmt::Decl(decl) = stmt {
            match *decl {
                Decl::Union {
                    ref name,
                    ref fields,
                } => {
                    assert_eq!(name, "Value");
                    assert_eq!(fields.len(), 2);
                    assert_eq!(fields[0].name, "i");
                    assert_eq!(fields[1].name, "f");
                }
                _ => panic!("expected union declaration"),
            }
        } else {
            panic!("expected declaration stmt");
        }
    }
}
