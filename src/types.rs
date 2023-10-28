#[derive(Debug)]
pub enum Expr {
    Bool(bool),
    Sym(String),
    Int(Int),
    Nil,
    List(Vec<Expr>),
    Vector(Vec<Expr>),
}

pub type Int = i64;
