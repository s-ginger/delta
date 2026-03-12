use delta::*;

#[cfg(test)]
mod tests {
    use super::*;

    fn lex_all(source: &str) -> Vec<TokenKind> {
        let mut lexer = Lexer::new(source);
        let mut tokens = Vec::new();

        loop {
            let tok = lexer.next_token();
            if tok.kind == TokenKind::EndOfFile {
                break;
            }
            tokens.push(tok.kind);
        }

        tokens
    }

    #[test]
    fn test_int_number() {
        let tokens = lex_all("123");

        assert!(matches!(tokens[0], TokenKind::Int(_)));
    }

    #[test]
    fn test_float_number() {
        let tokens = lex_all("45.67");

        assert!(matches!(tokens[0], TokenKind::Float(_)));
    }

    #[test]
    fn test_identifiers() {
        let tokens = lex_all("foo bar _baz");

        assert!(matches!(tokens[0], TokenKind::Ident(_)));
        assert!(matches!(tokens[1], TokenKind::Ident(_)));
        assert!(matches!(tokens[2], TokenKind::Ident(_)));
    }

    #[test]
    fn test_string_literal() {
        let tokens = lex_all(r#""hello""#);

        assert!(matches!(tokens[0], TokenKind::StringLiteral(_)));
    }

    #[test]
    fn test_char_literal() {
        let tokens = lex_all("'a'");

        assert!(matches!(tokens[0], TokenKind::CharLiteral(_)));
    }

    #[test]
    fn test_operators() {
        let tokens = lex_all("+ - * /");

        assert_eq!(
            tokens,
            vec![
                TokenKind::Add,
                TokenKind::Sub,
                TokenKind::Mul,
                TokenKind::Div
            ]
        );
    }

    #[test]
    fn test_comparison_ops() {
        let tokens = lex_all("== != >= <= > <");

        assert_eq!(
            tokens,
            vec![
                TokenKind::Eq,
                TokenKind::NotEq,
                TokenKind::GreaterEq,
                TokenKind::LessEq,
                TokenKind::Greater,
                TokenKind::Less
            ]
        );
    }

    #[test]
    fn test_logic_ops() {
        let tokens = lex_all("&& ||");

        assert_eq!(tokens, vec![TokenKind::And, TokenKind::Or]);
    }

    #[test]
    fn test_assignment() {
        let tokens = lex_all("a = 5");

        assert!(matches!(tokens[0], TokenKind::Ident(_)));
        assert_eq!(tokens[1], TokenKind::Assign);
        assert!(matches!(tokens[2], TokenKind::Int(_)));
    }

    #[test]
    fn test_short_assign() {
        let tokens = lex_all("a := 5");

        assert!(matches!(tokens[0], TokenKind::Ident(_)));
        assert_eq!(tokens[1], TokenKind::ShortAssign);
        assert!(matches!(tokens[2], TokenKind::Int(_)));
    }

    #[test]
    fn test_proc_decl() {
        let tokens = lex_all("main :: proc() {}");

        assert!(matches!(tokens[0], TokenKind::Ident(_)));
        assert_eq!(tokens[1], TokenKind::ColonColon);
        assert_eq!(tokens[2], TokenKind::Proc);
        assert_eq!(tokens[3], TokenKind::LParen);
        assert_eq!(tokens[4], TokenKind::RParen);
        assert_eq!(tokens[5], TokenKind::LBrace);
        assert_eq!(tokens[6], TokenKind::RBrace);
    }

    #[test]
    fn test_var_decl_pointer() {
        let tokens = lex_all("m: ^int = 12");

        assert!(matches!(tokens[0], TokenKind::Ident(_)));
        assert_eq!(tokens[1], TokenKind::Colon);
        assert_eq!(tokens[2], TokenKind::Caret);
        assert!(matches!(tokens[3], TokenKind::Ident(_)));
        assert_eq!(tokens[4], TokenKind::Assign);
        assert!(matches!(tokens[5], TokenKind::Int(_)));
    }

    #[test]
    fn test_array_syntax() {
        let tokens = lex_all("[10]i32");

        assert_eq!(tokens[0], TokenKind::LBracket);
        assert!(matches!(tokens[1], TokenKind::Int(_)));
        assert_eq!(tokens[2], TokenKind::RBracket);
        assert!(matches!(tokens[3], TokenKind::Ident(_)));
    }

    #[test]
    fn test_comments() {
        let tokens = lex_all("// hello");

        assert_eq!(tokens[0], TokenKind::Comment);
        println!("{:?}", tokens)
    }

    #[test]
    fn test_block_comments() {
        let tokens = lex_all("/* hello */");

        assert_eq!(tokens[0], TokenKind::BlockComment);
        println!("{:?}", tokens)
    }

    #[test]
    fn test_newline() {
        let tokens = lex_all("a\nb");

        assert!(matches!(tokens[0], TokenKind::Ident(_)));
        assert_eq!(tokens[1], TokenKind::NewLine);
        assert!(matches!(tokens[2], TokenKind::Ident(_)));
    }

    #[test]
    fn test_keywords() {
        let tokens = lex_all("if else while for return");

        assert_eq!(
            tokens,
            vec![
                TokenKind::If,
                TokenKind::Else,
                TokenKind::While,
                TokenKind::For,
                TokenKind::Return
            ]
        );
    }

    #[test]
    fn test_struct_union() {
        let tokens = lex_all("struct union");

        assert_eq!(tokens, vec![TokenKind::Struct, TokenKind::Union]);
    }
}