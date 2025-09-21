use crate::{ast, CFunctionArg};
pub mod struct_builder;

pub enum CFileContents {
    Include(String, bool),
    StaticVar,
    StructDecl,
    StructDef,
    FunctionDecl,
    FunctionDef,
}

pub struct CFileBuilder {
    pub name: String,

}
pub struct CFunctionBuilder {
    pub function: ast::CFunction,
}

pub struct CBlockBuilder {
    pub block: ast::CBlock,
}

impl CBlockBuilder {
    pub fn new() -> Self {
        Self {
            block: ast::CBlock{
                stmts: vec![]
            }
        }
    }
    pub fn addStatment(mut self, stmt: ast::CStmt) -> Self {
        self.block.stmts.push(stmt);
        self
    }
}

pub fn argument(ty: ast::CType, ident: String) -> ast::CFunctionArg {
    CFunctionArg{
        ty,
        ident
    }
}

pub fn function(retTy: ast::CType, name: String, args: Vec<ast::CFunctionArg>, body: CBlockBuilder) -> CFunctionBuilder {
    CFunctionBuilder {
            function: ast::CFunction{
                retTy,
                name,
                args,
                body: body.block
            }
        }
}

pub fn functionCall(name: String, args: Vec<ast::CExpr>) -> ast::CExpr {
    ast::CExpr::Call {
        funcName: name,
        args
    }
}


pub fn block() -> CBlockBuilder {
    CBlockBuilder { 
        block: ast::CBlock{
            stmts: vec![]
        }
     }
}
