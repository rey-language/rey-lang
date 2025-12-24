mod ast;
mod lexer;
mod parser;

use lexer::{Lexer, TokenKind};
use parser::Parser;

fn main() {
    let source = r#"
        func main(): Void {
            println("Hello World!");
        }
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
                break;
            }
        }
    }
    println!("Parsing Started.");
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();
    println!("Program parsed!");
    println!("AST:");
    for stmt in &ast {
        println!("{:?}", stmt);
    }
}
