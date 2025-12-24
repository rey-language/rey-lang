mod lexer;
mod parser;

use lexer::{Lexer, TokenKind};
use parser::{Parser};

fn main() {
    let source = r#"
        var x = "hello";
    "#;

    let mut lexer = Lexer::new(source);
    let mut tokens = Vec::new();
    
    loop {
        match lexer.nextToken() {
            Ok(token) => {
                println!("{:?}", token);
                tokens.push(token.clone());
                if token.kind == TokenKind::Eof {
                    break;
                }
            }
            Err(err) => {
                println!("Lexer error: {:?}", err);
                break;}
            }
    }
    println!("Parsing Started.");
    let mut parser = Parser::new(tokens);
    parser.parse();
    println!("Program parsed!");

}
