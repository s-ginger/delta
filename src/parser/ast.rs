#[derive(Debug, Clone)]
pub enum Expr {
    Ident(String),
    Number(i64),
    Float(f64),
    Str(String),

    BinOp {
        left: BExpr,
        right: BExpr,
        op: OP,
    },
    UnaryOP {
        op: OP,
        value: BExpr
    },
}

type BExpr = Box<Expr>;

#[derive(Debug, Clone)]
pub enum OP {
    Plus,
    Minus,
    Mul,
    Div,

    AddressOf,
    Ref
}

#[derive(Debug, Clone)] 
pub enum Define {
    Assign { name: String, value: Expr },
    ShortAssign { name: String,  value: Expr },
    Procedure { name: String, params: Vec<Expr>, returntype: Vec<Expr>, body: Stmt, }
}


#[derive(Debug, Clone)] 
pub enum Stmt {
    Package(String),
    Import(String),
    Block(Vec<Stmt>),
    Return(Vec<Expr>),
    Define(Box<Define>),
    StmtExpr(Expr)
}

