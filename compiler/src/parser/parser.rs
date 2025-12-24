use crate::lexer::{Token, TokenKind};

//impl for recursive descent parser
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,}
    }
    pub fn parse(&mut self) {
        while !self.isAtEnd() {
            self.parseStatement();
        }
    }

    //parsing statements
    fn parseStatement(&mut self) {
        if self.matchToken(&TokenKind::Var) {
            self.parseVarDeclaration();} 
        else {
            self.parseExpressionStatement();}
    }
    fn parseVarDeclaration(&mut self) {
        self.consumeIdentifier("Expected variable name.");
        self.consume(&TokenKind::Equal, "Expected '=' after variable name.");
        self.parseExpression();
        self.consume(&TokenKind::Semicolon, "Expected ';' after variable declaration.");
    }
    fn parseExpressionStatement(&mut self) {
        self.parseExpression();
        self.consume(&TokenKind::Semicolon, "Expected ';' after expression.");
    }

    fn parseTypeAnnotation(&mut self) -> Option<Type> {
    if self.matchToken(&TokenKind::Colon) {
        match &self.peek().kind {
            TokenKind::Identifier(name) => {
                let ty = Type { name: name.clone() };
                self.advance();
                Some(ty);
            } _ => self.error("Expected type name after ':'"),}}
    else {None}
    }
    fn parseAdditive(&mut self) -> Expr {
        let mut expr = self.parseUnary();

        while matches!(self.peek().kind, TokenKind::Plus | TokenKind::Minus) {
            let op = self.peek().kind.clone();
            self.advance();
            let right = self.parseUnary();

            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        expr
    }


   //expressions
    fn parseExpression(&mut self) {
        match &self.peek().kind {
            TokenKind::Identifier(_) |
            TokenKind::StringLiteral(_) |
            TokenKind::NumberLiteral(_) |
            TokenKind::True |
            TokenKind::False |
            TokenKind::Null => {
                self.advance();
            }
            _ => self.error("Expected expression."),
        }
    }


    //token utils
    fn matchToken(&mut self, kind: &TokenKind) -> bool {
        if self.check(kind) {
            self.advance();
            true
        } 
        else {
            false }
    }
    fn consume(&mut self, kind: &TokenKind, message: &str) {
        if self.check(kind) {
            self.advance();} 
        else {
            self.error(message);}
    }
    fn consumeIdentifier(&mut self, message: &str) {
        match &self.peek().kind {
            TokenKind::Identifier(_) => {
                self.advance();
            }
            _ => self.error(message),
        }
    }
    fn check(&self, kind: &TokenKind) -> bool {
        if self.isAtEnd() {
            return false;}
        std::mem::discriminant(&self.peek().kind) == std::mem::discriminant(kind)
    }
    fn advance(&mut self) -> &Token {
        if !self.isAtEnd() {
            self.current += 1;}
        self.previous()
    }
    fn isAtEnd(&self) -> bool {
        matches!(self.peek().kind, TokenKind::Eof)
    }
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }
    fn previous(&self) -> &Token {
       if self.current == 0 {
        &self.tokens[0]
    } else {
        &self.tokens[self.current - 1]
    }
    }

    //error
    fn error(&self, message: &str) -> ! {
        panic!(
            "Parse error at {:?}: {}",
            self.peek().span, message
        );
    }
}
