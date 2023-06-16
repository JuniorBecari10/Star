pub type Program = Vec<Stmt>;

pub enum Stmt {
    VarStmt {
        line: i32,
        ident: String,
        exp: Exp,
        is_const: bool
    },
    RetStmt {
        line: i32,
        exp: Exp
    },
    ExpStmt {
        line: i32,
        exp: Exp
    },
    ErrorStmt {
        line: i32,
        msg: String
    }
}

pub enum Exp {
    IdentExp(String),
    NumberExp(f64),
    BinExp(Box<Exp>, Box<Exp>, char)
}
