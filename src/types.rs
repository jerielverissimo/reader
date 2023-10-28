#[derive(Debug)]
pub enum Expr {
    Bool(bool),
    Sym(String),
    Int(Int),
    Nil,
    List(Vec<Expr>),
}

pub type Int = i64;
