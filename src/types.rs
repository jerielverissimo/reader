#[derive(Debug)]
pub enum Expr {
    Bool(bool),
    Sym(String),
    Str(String),
    Keyword(Keyword),
    Int(Int),
    Nil,
    List(Vec<Expr>),
    Vector(Vec<Expr>),
}

#[derive(Debug)]
pub enum Keyword {
    Simple(String),
    Namespaced(String, String),
}

pub type Int = i64;
