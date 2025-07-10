// src/parser/ast.rs

// src/parser/ast.rs

#[derive(Debug, PartialEq, Clone)]
pub enum Visibility {
    Public,
    Private,
    Protected,
}

impl Default for Visibility {
    fn default() -> Self {
        Visibility::Public
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Number(f64),
    String(String),
    Bool(bool),
    Null,
    Array(Vec<Expr>),
    Index {
        object: Box<Expr>,
        index: Box<Expr>,
    },
    Identifier(String),
    This,
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
    },
    Call {
        function: String,
        args: Vec<Expr>,
    },
    Get {
        object: Box<Expr>,
        name: String,
    },
    Set {
        object: Box<Expr>,
        name: String,
        value: Box<Expr>,
    },
    New {
        class: String,
        args: Vec<Expr>,
    },
    Assign {
        name: String,
        value: Box<Expr>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    And,
    Or,
}

#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOp {
    Not,    // !
    Minus,  // -
}

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    Let { name: String, value: Expr },
    Assign { name: String, value: Expr },
    Expr(Expr),
    Block(Vec<Stmt>),
    If { cond: Expr, then_branch: Box<Stmt>, else_branch: Option<Box<Stmt>> },
    While { cond: Expr, body: Box<Stmt> },
    For { init: Option<Box<Stmt>>, cond: Option<Expr>, post: Option<Expr>, body: Box<Stmt> },
    FunDecl { 
        name: String, 
        params: Vec<String>, 
        body: Box<Stmt>,
        visibility: Visibility,
        is_static: bool,
    },
    ClassDecl {
        name: String,
        methods: Vec<Stmt>,
        fields: Vec<FieldDecl>,
        visibility: Visibility,
    },
    NamespaceDecl {
        name: String,
        body: Vec<Stmt>,
    },
    Using {
        module_path: String,
        alias: Option<String>,
    },
    Use {
        file_path: String,
    },
    Export {
        item: Box<Stmt>,
    },
    Return(Option<Expr>),
    Try {
        try_block: Box<Stmt>,
        catch_param: Option<String>,
        catch_block: Box<Stmt>,
    },
    Throw {
        value: Expr,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub struct FieldDecl {
    pub name: String,
    pub visibility: Visibility,
}
