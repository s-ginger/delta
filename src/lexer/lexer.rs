use crate::*;

pub struct Lexer<'a> {
    src: &'a str,
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self { src, pos: 0 }
    }

    fn peek(&self) -> Option<char> {
        self.src[self.pos..].chars().next()
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.pos += c.len_utf8();
        Some(c)
    }

    fn make_token(&self, kind: TokenKind, start: usize) -> Token {
        Token {
            kind,
            span: Span {
                start,
                end: self.pos,
            },
        }
    }

    pub fn next_token(&mut self) -> Token {
        
        while let Some(c) = self.peek() {
            if c.is_whitespace() && c != '\n' {
                self.advance();
            } else {
                break;
            }
        }

        let start = self.pos;

        let c = match self.advance() {
            Some(c) => c,
            None => return self.make_token(TokenKind::EndOfFile, start),
        };


        match c {
            '\n' => self.make_token(TokenKind::NewLine, start),

            '+' => self.make_token(TokenKind::Add, start),

            '-' => {
                if self.peek() == Some('>') {
                    self.advance();
                    self.make_token(TokenKind::Arrow, start)
                } else {
                    self.make_token(TokenKind::Sub, start)
                }
            }

            '*' => self.make_token(TokenKind::Mul, start),

            '/' => {
                if self.peek() == Some('/') {
                    self.read_line_comment(start)
                } else if self.peek() == Some('*') {
                    self.read_block_comment(start)
                } else {
                    self.make_token(TokenKind::Div, start)
                }
            }

            '=' => {
                if self.peek() == Some('=') {
                    self.advance();
                    self.make_token(TokenKind::Eq, start)
                } else {
                    self.make_token(TokenKind::Assign, start)
                }
            }

            ':' => {
                if self.peek() == Some('=') {
                    self.advance();
                    self.make_token(TokenKind::ShortAssign, start)
                } else if self.peek() == Some(':') {
                    self.advance();
                    self.make_token(TokenKind::ColonColon, start)
                } else {
                    self.make_token(TokenKind::Colon, start)
                }
            }

            '!' => {
                if self.peek() == Some('=') {
                    self.advance();
                    self.make_token(TokenKind::NotEq, start)
                } else {
                    self.make_token(TokenKind::Not, start)
                }
            }

            '>' => {
                if self.peek() == Some('=') {
                    self.advance();
                    self.make_token(TokenKind::GreaterEq, start)
                } else {
                    self.make_token(TokenKind::Greater, start)
                }
            }

            '<' => {
                if self.peek() == Some('=') {
                    self.advance();
                    self.make_token(TokenKind::LessEq, start)
                } else {
                    self.make_token(TokenKind::Less, start)
                }
            }

            '&' => {
                if self.peek() == Some('&') {
                    self.advance();
                    self.make_token(TokenKind::And, start)
                } else {
                    self.make_token(TokenKind::Ampersand, start)
                }
            }

            '|' => {
                if self.peek() == Some('|') {
                    self.advance();
                    self.make_token(TokenKind::Or, start)
                } else {
                    self.make_token(TokenKind::Error, start)
                }
            }

            '^' => self.make_token(TokenKind::Caret, start),

            '{' => self.make_token(TokenKind::LBrace, start),
            '}' => self.make_token(TokenKind::RBrace, start),

            '(' => self.make_token(TokenKind::LParen, start),
            ')' => self.make_token(TokenKind::RParen, start),

            '[' => self.make_token(TokenKind::LBracket, start),
            ']' => self.make_token(TokenKind::RBracket, start),

            ';' => self.make_token(TokenKind::Semicolon, start),
            ',' => self.make_token(TokenKind::Comma, start),
            '.' => self.make_token(TokenKind::Dot, start),

            '"' => self.read_string(start),
            '\'' => self.read_char(start),

            c if c.is_ascii_digit() => self.read_number(start),

            c if c.is_alphabetic() || c == '_' => self.read_ident(start),

            _ => self.make_token(TokenKind::Error, start),
        }
    }

    fn read_number(&mut self, start: usize) -> Token {
        let mut is_float = false;

        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                self.advance();
            } else if c == '.' && !is_float {
                is_float = true;
                self.advance();
            } else {
                break;
            }
        }

        let text = &self.src[start..self.pos];

        if is_float {
            let v = text.parse().unwrap_or(0.0);
            self.make_token(TokenKind::Float(v), start)
        } else {
            let v = text.parse().unwrap_or(0);
            self.make_token(TokenKind::Int(v), start)
        }
    }

    fn read_string(&mut self, start: usize) -> Token {
        let mut value = String::new();

        while let Some(c) = self.advance() {
            if c == '"' {
                break;
            }

            if c == '\\' {
                if let Some(next) = self.advance() {
                    match next {
                        'n' => value.push('\n'),
                        't' => value.push('\t'),
                        '"' => value.push('"'),
                        '\\' => value.push('\\'),
                        _ => value.push(next),
                    }
                }
            } else {
                value.push(c);
            }
        }

        self.make_token(TokenKind::StringLiteral(value), start)
    }

    fn read_char(&mut self, start: usize) -> Token {
        let c = match self.advance() {
            Some(c) => c,
            None => return self.make_token(TokenKind::Error, start),
        };

        if self.advance() != Some('\'') {
            return self.make_token(TokenKind::Error, start);
        }

        self.make_token(TokenKind::CharLiteral(c), start)
    }

    fn read_line_comment(&mut self, start: usize) -> Token {
        self.advance();

        while let Some(c) = self.peek() {
            if c == '\n' {
                break;
            }
            self.advance();
        }

        self.make_token(TokenKind::Comment, start)
    }

    fn read_block_comment(&mut self, start: usize) -> Token {
        self.advance();

        while let Some(c) = self.advance() {
            if c == '*' && self.peek() == Some('/') {
                self.advance();
                break;
            }
        }

        self.make_token(TokenKind::BlockComment, start)
    }

    fn read_ident(&mut self, start: usize) -> Token {
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                self.advance();
            } else {
                break;
            }
        }

        let text = &self.src[start..self.pos];

        let kind = match text {
            "package" => TokenKind::Package,
            "import" => TokenKind::Import,
            "proc" => TokenKind::Proc,
            "return" => TokenKind::Return,
            "struct" => TokenKind::Struct,
            "union" => TokenKind::Union,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "while" => TokenKind::While,
            "for" => TokenKind::For,
            _ => TokenKind::Ident(text.into()),
        };

        self.make_token(kind, start)
    }
}
