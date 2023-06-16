pub type Program = Vec<Stmt>;

pub enum Stmt {
    VarStmt {
        line: i32,
        ident: String,
        exp: Exp,
        constant: bool
    },
    RetStmt {
        line: i32,
        exp: Exp
    },
    ExpStmt {
        line: i32,
        exp: Exp
    }
}

pub enum Exp {
    IdentExp(String),
    NumberExp(f64),
    BinExp(Box<Exp>, Box<Exp>, char)
}
