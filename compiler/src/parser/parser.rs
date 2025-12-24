use crate::ast::{Expr, Literal, Parameter, Stmt, Type};
use crate::lexer::{Token, TokenKind};

//impl for recursive descent parser
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }
    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();
        while !self.isAtEnd() {
            if let Some(stmt) = self.parseStatement() {
                statements.push(stmt);
            }
        }
        statements
    }

    //parsing statements
    fn parseStatement(&mut self) -> Option<Stmt> {
        if self.isAtEnd() {
            return None;
        }
        if self.matchToken(&TokenKind::Var) {
            Some(self.parseVarDeclaration())
        } else if self.matchToken(&TokenKind::Func) {
            Some(self.parseFuncDeclaration())
        } else {
            Some(self.parseExpressionStatement())
        }
    }
    fn parseVarDeclaration(&mut self) -> Stmt {
        let name = match &self.peek().kind {
            TokenKind::Identifier(name) => name.clone(),
            _ => self.error("Expected variable name."),
        };
        self.advance();

        let ty = self.parseTypeAnnotation();

        self.consume(&TokenKind::Equal, "Expected '=' after variable name.");
        let initializer = self.parseExpression();
        self.consume(
            &TokenKind::Semicolon,
            "Expected ';' after variable declaration.",
        );

        Stmt::VarDecl { name, ty, initializer }
    }

    fn parseFuncDeclaration(&mut self) -> Stmt {
        let name = match &self.peek().kind {
            TokenKind::Identifier(name) => name.clone(),
            _ => self.error("Expected function name."),
        };
        self.advance();

        self.consume(&TokenKind::LeftParen, "Expected '(' after function name.");

        let mut params = Vec::new();
        if !self.check(&TokenKind::RightParen) {
            loop {
                let param_name = match &self.peek().kind {
                    TokenKind::Identifier(name) => name.clone(),
                    _ => self.error("Expected parameter name."),
                };
                self.advance();

                let param_ty = self.parseTypeAnnotation();

                params.push(Parameter {
                    name: param_name,
                    ty: param_ty,
                });

                if !self.matchToken(&TokenKind::Comma) {
                    break;
                }
            }
        }
        self.consume(&TokenKind::RightParen, "Expected ')' after parameters.");

        let return_ty = self.parseTypeAnnotation();

        self.consume(&TokenKind::LeftBrace, "Expected '{' before function body.");

        let mut body = Vec::new();
        while !self.check(&TokenKind::RightBrace) && !self.isAtEnd() {
            if let Some(stmt) = self.parseStatement() {
                body.push(stmt);
            }
        }
        self.consume(&TokenKind::RightBrace, "Expected '}' after function body.");

        Stmt::FuncDecl {
            name,
            params,
            return_ty,
            body,
        }
    }

    fn parseExpressionStatement(&mut self) -> Stmt {
        let expr = self.parseExpression();
        self.consume(&TokenKind::Semicolon, "Expected ';' after expression.");
        Stmt::ExprStmt(expr)
    }

    fn parseTypeAnnotation(&mut self) -> Option<Type> {
        if self.matchToken(&TokenKind::Colon) {
            match &self.peek().kind {
                TokenKind::Identifier(name) => {
                    let ty = Type { name: name.clone() };
                    self.advance();
                    Some(ty)
                }
                _ => self.error("Expected type name after ':'"),
            }
        } else {
            None
        }
    }

    fn parseUnary(&mut self) -> Expr {
        match &self.peek().kind {
            TokenKind::Minus => {
                self.advance();
                let expr = self.parseUnary();
                Expr::Binary {
                    left: Box::new(Expr::Literal(Literal::Number(0.0))),
                    op: TokenKind::Minus,
                    right: Box::new(expr),
                }
            }
            _ => self.parsePrimary(),
        }
    }

    fn parsePrimary(&mut self) -> Expr {
        match self.peek().kind.clone() {
            TokenKind::Identifier(name) => {
                self.advance();
                if self.matchToken(&TokenKind::LeftParen) {
                    // Function call
                    let callee = Expr::Variable(name);
                    let mut args = Vec::new();

                    if !self.check(&TokenKind::RightParen) {
                        loop {
                            args.push(self.parseExpression());
                            if !self.matchToken(&TokenKind::Comma) {
                                break;
                            }
                        }
                    }
                    self.consume(&TokenKind::RightParen, "Expected ')' after function arguments.");
                    Expr::Call {
                        callee: Box::new(callee),
                        args,
                    }
                } else {
                    Expr::Variable(name)
                }
            }
            TokenKind::StringLiteral(value) => {
                self.advance();
                Expr::Literal(Literal::String(value))
            }
            TokenKind::NumberLiteral(value) => {
                self.advance();
                Expr::Literal(Literal::Number(value))
            }
            TokenKind::True => {
                self.advance();
                Expr::Literal(Literal::Bool(true))
            }
            TokenKind::False => {
                self.advance();
                Expr::Literal(Literal::Bool(false))
            }
            TokenKind::Null => {
                self.advance();
                Expr::Literal(Literal::Null)
            }
            _ => self.error("Expected expression."),
        }
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
    fn parseExpression(&mut self) -> Expr {
        self.parseAdditive()
    }

    //token utils
    fn matchToken(&mut self, kind: &TokenKind) -> bool {
        if self.check(kind) {
            self.advance();
            true
        } else {
            false
        }
    }
    fn consume(&mut self, kind: &TokenKind, message: &str) {
        if self.check(kind) {
            self.advance();
        } else {
            self.error(message);
        }
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
            return false;
        }
        std::mem::discriminant(&self.peek().kind) == std::mem::discriminant(kind)
    }
    fn advance(&mut self) -> &Token {
        if !self.isAtEnd() {
            self.current += 1;
        }
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
        panic!("Parse error at {:?}: {}", self.peek().span, message);
    }
}
