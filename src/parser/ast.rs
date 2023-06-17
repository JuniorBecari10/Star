pub type Program = Vec<Stmt>;

#[derive(Eq, PartialEq)]
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
    },
    EndStmt
}

#[derive(Eq, PartialEq)]
pub enum Exp {
    IdentExp(String),
    NumberExp(f64),
    BinExp(Box<Exp>, Box<Exp>, char)
}

/*
i8, byte
i16, short
i32, int
i64, long
i128, longer

ui8, ubyte
ui16, ushort
ui32, uint
ui64, ulong
ui128, ulonger

f32, float
f64, double

str
char
bool
*/
#[derive(Eq, PartialEq)]
pub enum Type {
    Byte,
    Short,
    Int,
    Long,
    Longer,

    Ubyte,
    Ushort,
    Uint,
    Ulong,
    Ulonger,

    Float,
    Double,

    Ufloat,
    Udouble,

    Str,
    Char,
    Bool
}
