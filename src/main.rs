use delta::Parser;
use delta::lexer::{Lexer, Token};

fn main() {
    let lexer = Lexer::new("{ m: ^int = 10 + 12 } ");
    let mut tokens: Vec<Token> = vec![];

    for t in lexer {
        tokens.push(t);
    }

    println!("{:#?}", tokens);

    let mut parser = Parser::new(tokens);

    let stmt = parser.parse_stmts();

    println!("\n{:#?}", stmt);
}
