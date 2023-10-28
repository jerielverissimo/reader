use std::{iter::Peekable, vec::IntoIter};

use crate::{
    error::ReadError,
    types::{Expr, Int},
};

#[derive(Debug)]
pub struct Reader {
    cache: Peekable<IntoIter<char>>,
}

impl Reader {
    pub fn from(s: &str) -> Self {
        Self {
            cache: s.chars().collect::<Vec<_>>().into_iter().peekable(),
        }
    }

    #[inline]
    fn peek(&mut self) -> Result<char, ReadError> {
        self.cache.peek().ok_or(ReadError::EndOfInput).cloned()
    }

    #[inline]
    fn next(&mut self) -> Result<char, ReadError> {
        self.cache.next().ok_or(ReadError::EndOfInput)
    }
}

pub fn read(r: &mut Reader) -> Result<Expr, ReadError> {
    skip_whitespace(r)?;

    let c = r.peek()?;
    match c {
        '(' => {
            r.next()?;
            read_seq(r, ')')
        }
        '[' => {
            r.next()?;
            read_seq(r, ']')
        }
        '"' => {
            r.next()?;
            read_string(r)
        }
        _ => {
            let atom = read_atom(r)?;
            Ok(parse_atom(atom))
        }
    }
}

fn read_seq(r: &mut Reader, end: char) -> Result<Expr, ReadError> {
    let mut seq = vec![];

    loop {
        skip_whitespace(r)?;

        match r.peek() {
            Ok(c) if c == end => {
                r.next()?;
                break;
            }
            Ok(_) => (),
            Err(ReadError::EndOfInput) => {
                return Err(ReadError::Missing(String::from("closing bracket")))
            }
            Err(msg) => return Err(msg),
        }
        let expr = read(r)?;
        seq.push(expr);
    }

    let result = match end {
        ')' => Expr::List(seq),
        ']' => Expr::Vector(seq),
        c => return Err(ReadError::Unexpected(c)),
    };

    Ok(result)
}

fn parse_atom(atom: String) -> Expr {
    match atom.as_str() {
        "true" => Expr::Bool(true),
        "false" => Expr::Bool(false),
        "nil" => Expr::Nil,
        _ => atom_expr(atom),
    }
}

fn atom_expr(atom: String) -> Expr {
    if let Ok(num) = atom.parse::<Int>() {
        Expr::Int(num)
    } else if atom.starts_with(":") {
        if atom.contains("/") {
            let split: Vec<&str> = atom.split("/").collect();
            let (namespace, name) = (split[0], split[1]);
            Expr::Keyword(crate::types::Keyword::Namespaced(
                namespace[1..].to_string(),
                name.to_string(),
            ))
        } else {
            Expr::Keyword(crate::types::Keyword::Simple(atom[1..].to_string()))
        }
    } else {
        Expr::Sym(atom)
    }
}

fn read_atom(r: &mut Reader) -> Result<String, ReadError> {
    let mut chars = vec![];
    loop {
        match r.peek() {
            Ok(c) => {
                if is_word_boundary(c) {
                    break;
                }
            }
            Err(ReadError::EndOfInput) => break,
            Err(msg) => return Err(msg),
        }

        if let Ok(c) = r.next() {
            chars.push(c);
        }
    }
    Ok(chars.into_iter().collect())
}

fn read_string(r: &mut Reader) -> Result<Expr, ReadError> {
    let mut chars: Vec<char> = vec![];
    loop {
        match r.next() {
            Err(ReadError::EndOfInput) => return Err(ReadError::Missing(String::from("\""))),
            Ok('"') => break,
            Ok('\\') => todo!(),
            Ok(c) => chars.push(c),
            Err(msg) => return Err(msg),
        }
    }
    let string = chars.into_iter().collect();
    Ok(Expr::Str(string))
}

#[inline]
fn is_special(c: char) -> bool {
    matches!(c, '(' | ')' | '[' | ']' | '\'' | '`' | ',' | '"' | ';')
}

#[inline]
fn is_word_boundary(c: char) -> bool {
    c.is_whitespace() || is_special(c)
}

fn skip_whitespace(r: &mut Reader) -> Result<(), ReadError> {
    loop {
        match r.peek() {
            Ok(c) => {
                if c.is_whitespace() || c == ',' {
                    r.next()?;
                } else {
                    break;
                }
            }
            Err(ReadError::EndOfInput) => break,
            Err(msg) => return Err(msg),
        }
    }
    Ok(())
}
