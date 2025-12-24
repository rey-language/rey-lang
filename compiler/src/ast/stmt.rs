use super::{Expr, Type};

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub ty: Option<Type>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    VarDecl {
        name: String,
        ty: Option<Type>,
        initializer: Expr,
    },
    FuncDecl {
        name: String,
        params: Vec<Parameter>,
        return_ty: Option<Type>,
        body: Vec<Stmt>,
    },
    ExprStmt(Expr),
}
