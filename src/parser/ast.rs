#[derive(Debug, Clone)]
pub enum Expr {
    Ident(String),
    Number(i64),
    Float(f64),
    Str(String),

    BinOp { left: BExpr, right: BExpr, op: OP },
    UnaryOP { op: OP, value: BExpr },
    Call {
        name: String,
        params: Vec<Expr>
    }
}

type BExpr = Box<Expr>;

#[derive(Debug, Clone)]
pub enum OP {
    Plus,
    Minus,
    Mul,
    Div,

    AddressOf,
    Ref,
}

#[derive(Debug, Clone)]
pub enum Define {
    Assign {
        name: String,
        value: Expr,
    },
    ShortAssign {
        name: String,
        value: Expr,
    },
    Const {
        name: String,
        value: Expr,
    },
    DefVar {
        names: Vec<String>,
        types: Vec<Type>,
        value: Option<Expr>,
    },
    Procedure {
        name: String,
        params: Box<Define>,
        returntype: Vec<Expr>,
        body: Stmt,
    },
    Struct {
        name: String,
        params: Box<Define>,
        
    },
    Unions {
        name: String,
        params: Box<Define>,
    }
}

#[derive(Debug, Clone)]
pub enum Type {
    Name(String),
    Ref(Box<Type>),
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Package(String),
    Import(String),
    Block(Vec<Stmt>),
    Return(Vec<Expr>),
    Define(Box<Define>),
    StmtExpr(Expr),
}
