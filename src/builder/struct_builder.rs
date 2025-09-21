use crate::ast;
pub struct CStructBuilder {
    pub cstruct: ast::CStruct,
}

impl CStructBuilder {
    pub fn new(name: String) -> Self {
        Self {
            cstruct: ast::CStruct {
                name: name,
                defs: vec![]
            }
        }
    }
    pub fn add_field(mut self, ty: ast::CType, name: String) -> Self {
        self.cstruct.defs.push(ast::CStructField {
            name,
            ty
        });
        self
    }
}