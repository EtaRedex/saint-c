use crate::*;

pub trait Emit {
    fn emit(&self) -> String;
}

impl Emit for CType {
    fn emit(&self) -> String {
        match self {
            CType::Int => "int".into(),
            CType::Char => "char".into(),
            CType::Void => "void".into(),
            CType::Float => "float".into(),
            CType::Double => "double".into(),
            CType::Custom(s) => s.to_string(),
            CType::Pointer(p) => format!("{}*", p.emit()),
            CType::Struct => "".into(),
        }
    }
}

impl Emit for CExpr {
    fn emit(&self) -> String {
        match self {
            CExpr::StringLiteral(s) => format!("\"{}\"", s),
            CExpr::Ident(name) => name.to_string(),
            CExpr::Call { funcName, args } => {
                let mut s_args: String = String::from("");
                if args.len() > 0 {
                    for arg in args {
                        s_args.push_str(&format!("{},", arg.emit()));
                    }
                    s_args.pop();
                }
                format!("{}({})", funcName, s_args)
            }
            CExpr::Digit(d) => d.into(),
            CExpr::PreUnaryOp { op, rhs } => format!("{}{}", op, rhs.emit()),
            CExpr::PostUnaryOp { lhs, op } => format!("{}{}", lhs.emit(), op),
            CExpr::BinaryOp { lhs, op, rhs } => format!("{} {} {}", lhs.emit(), op, rhs.emit()),
        }
    }
}

impl Emit for CStmt {
    fn emit(&self) -> String {
        match self {
            CStmt::Expr(e) => format!("{};", e.emit()),
            CStmt::Return(e) => {
                if e.is_none() {
                    "return".into()
                } else {
                    format!("return {};", e.as_ref().unwrap().emit())
                }
            }
            CStmt::Assign(s, cexpr) => {
                format!("{} = {};", s, cexpr.emit())
            }
            CStmt::Var { name, ty, val } => {
                if val.is_some() {
                    format!("{} {} = {};", ty.emit(), name, val.as_ref().unwrap().emit())
                } else {
                    format!("{} {};", ty.emit(), name)
                }
            }
            CStmt::Block(stmts) => {
                let mut s = String::from("{");
                for stmt in stmts {
                    let stmt_str = stmt.emit();
                    s.push_str(&stmt_str);
                }
                s.push('}');
                s
            }
            CStmt::If { cond, thenB, elseB } => {
                if elseB.is_some() {
                    format!(
                        "if({}){{{}}}else{{{}}}",
                        cond.emit(),
                        thenB.emit(),
                        elseB.as_ref().unwrap().emit()
                    )
                } else {
                    format!("if({}){{{}}}", cond.emit(), thenB.emit())
                }
            }
        }
    }
}

impl Emit for CBlock {
    fn emit(&self) -> String {
        let mut s = String::from("{");
        for stmt in &self.stmts {
            let stmt_str = stmt.emit();
            s.push_str(&stmt_str);
        }
        s.push('}');
        s
    }
}

impl Emit for CFunction {
    fn emit(&self) -> String {
        let mut s_args: String = String::from("");
        if self.args.len() > 0 {
            for arg in &self.args {
                s_args.push_str(&format!("{} {},", arg.ty.emit(), arg.ident));
            }
            s_args.pop();
        }
        format!(
            "{} {}({}){}",
            self.retTy.emit(),
            self.name,
            s_args,
            self.body.emit()
        )
    }
}
