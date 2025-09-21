#[derive(Debug, Clone)]
pub enum CType {
    Int,
    Char,
    Float,
    Double,
    Void,
    Struct,
    Pointer(Box<CType>),
    Custom(String)
}

#[derive(Debug, Clone)]
pub enum CExpr {
    Ident(String),
    Digit(String),
    StringLiteral(String),
    BinaryOp { lhs: Box<CExpr>, op: String, rhs: Box<CExpr> },
    PreUnaryOp { op: String, rhs: Box<CExpr> },
    PostUnaryOp { lhs: Box<CExpr>, op: String },
    Call { funcName: String, args: Vec<CExpr> }
}

#[derive(Debug, Clone)]
pub enum CStmt {
    Block(Vec<CStmt>),
    Return(Option<CExpr>),
    Assign(String, CExpr),
    Expr(CExpr),
    Var { name: String, ty: CType, val: Option<CExpr> },
    If { cond: CExpr, thenB: Box<CStmt>, elseB: Option<Box<CStmt>> }
}


pub struct CBlock {
    pub stmts: Vec<CStmt>
}
pub struct CFunctionArg {
    pub ty: CType,
    pub ident: String
}
pub struct CFunction {
    pub retTy: CType,
    pub name: String,
    pub args: Vec<CFunctionArg>,
    pub body: CBlock
}

pub struct CStructField {
    pub name: String,
    pub ty: CType
}
pub struct CStruct {
    pub name: String,
    pub defs: Vec<CStructField>
}

pub struct CEnum {
    pub fields: Vec<String>
}

pub struct CFile {
    name: String,
    includes: Vec<String>,
    staticVars: Vec<CExpr>,
    functions: Vec<CFunction>,
}