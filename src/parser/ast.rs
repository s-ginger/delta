
#[derive(Debug, Clone)]
pub enum Expr {
    Ident(String),
    Int(i64),
    Float(f64),
    Str(String),
    Char(char),

    Binary {
        left: Box<Expr>,
        op: Op,
        right: Box<Expr>,
    },

    Unary {
        op: Op,
        expr: Box<Expr>,
    },

    Call {
        func: Box<Expr>,
        args: Vec<Expr>,
    },
}

#[derive(Debug, Clone)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    AddressOf,
    Deref,
}


#[derive(Debug, Clone)]
pub enum Type {
    // пользовательские типы
    Named(String),

    // целые
    I8,
    I16,
    I32,
    I64,
    I128,
    U8,
    U16,
    U32,
    U64,
    U128,

    // float
    F32,
    F64,

    // логический
    Bool,

    // символ и строка
    Char,
    String,

    // специальный
    Void,
    Never, // !

    // контейнеры
    Array(usize, Box<Type>),
    Slice(Box<Type>),

    // указатели
    Ptr(Box<Type>),

    // функции
    Proc(Vec<Type>, Box<Type>), // args -> return
}

#[derive(Debug, Clone)]
pub enum Decl {
    Var {
        names: Vec<String>,
        ty: Option<Type>,
        value: Option<Expr>,
    },

    Const {
        name: String,
        value: Expr,
    },

    Proc {
        name: String,
        params: Vec<Field>,
        returns: Vec<Type>,
        body: Stmt,
    },

    Struct {
        name: String,
        fields: Vec<Field>,
    },

    Union {
        name: String,
        fields: Vec<Field>,
    },
}

#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub ty: Type,
}


#[derive(Debug, Clone)]
pub enum Stmt {
    Package(String),

    Import(String),

    Block(Vec<Stmt>),

    Return(Vec<Expr>),

    Decl(Box<Decl>),

    Expr(Expr),
}