mod lexer;

use lexer::{Lexer, TokenKind};

fn main() {
    let source = r#"
        var x = "hello": int;

        func greet() {
            print(x);
        }
        var y = "world";
    "#;

    let mut lexer = Lexer::new(source);

    loop {
        match lexer.nextToken() {
            Ok(token) => {
                println!("{:?}", token);

                if token.kind == TokenKind::Eof {
                    break;
                }
            }
            Err(err) => {
                println!("Lexer error: {:?}", err);
                break;}
            }
    }
}
