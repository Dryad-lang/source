// src/parser/ast.rs

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Number(f64),
    String(String),
    Identifier(String),
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
    Call {
        function: String,
        args: Vec<Expr>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    Let { name: String, value: Expr },
    Expr(Expr),
    Block(Vec<Stmt>),
    If { cond: Expr, then_branch: Box<Stmt>, else_branch: Option<Box<Stmt>> },
    While { cond: Expr, body: Box<Stmt> },
    For { init: Option<Box<Stmt>>, cond: Option<Expr>, post: Option<Expr>, body: Box<Stmt> },
}
