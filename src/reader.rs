<<<<<<< Updated upstream
#[derive(Debug)]
pub enum Expr {}

pub fn read(input: &str) -> Vec<Expr> {
    let mut parsed = vec![];
    parsed
=======
use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use crate::error::CompilerError;

pub type FileId = usize;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Span {
    pub file_id: FileId,
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(file_id: FileId, start: usize, end: usize) -> Self {
        Self {
            file_id,
            start,
            end,
        }
    }

    pub fn contains(self, span: Span) -> bool {
        self.file_id == span.file_id && span.start >= self.start && span.end <= self.end
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenContents {
    Number(i64),
    Str(String),
    Sym(String),
    Nil,
    Bool(bool),
    Keyword(String),
    List(Vec<Token>),
    Vector(Vec<Token>),
    Eof,
    Garbage,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub contents: TokenContents,
    pub span: Span,
}

impl Token {
    pub fn new(contents: TokenContents, span: Span) -> Self {
        Self { contents, span }
    }

    pub fn unknown(span: Span) -> Self {
        Self {
            contents: TokenContents::Garbage,
            span,
        }
    }
}

pub struct SourceInfo {
    pub file_id: FileId,
    pub path: PathBuf,
    pub content: String,
}

pub struct Reader {
    raw_files: Vec<(String, Vec<u8>)>,
    pub loaded_files: Vec<SourceInfo>,
}

impl Reader {
    pub fn new() -> Self {
        Self {
            raw_files: vec![],
            loaded_files: vec![],
        }
    }

    pub fn read(&mut self, fname: &str) -> Result<Vec<Token>, CompilerError> {
        let contents = std::fs::read(fname)?;

        self.raw_files.push((fname.to_string(), contents.clone()));

        self.loaded_files.push(SourceInfo {
            file_id: self.loaded_files.len(),
            path: PathBuf::from_str(fname).unwrap(),
            content: String::from_utf8(contents).unwrap(),
        });

        let (lexed, err) = _read(
            self.raw_files.len() - 1,
            &self.raw_files[self.raw_files.len() - 1].1,
        );

        match err {
            Some(err) => {
                return Err(err);
            }
            _ => {}
        }

        Ok(lexed)
    }

    pub fn get_file_contents(&self, file_id: FileId) -> &[u8] {
        &self.raw_files[file_id].1
    }

    pub fn get_file_name(&self, file_id: FileId) -> &Path {
        self.loaded_files[file_id].path.as_path()
    }
}

fn _read(file_id: FileId, bytes: &[u8]) -> (Vec<Token>, Option<CompilerError>) {
    let mut output = Vec::new();
    let mut index = 0;
    let mut error = None;

    while index < bytes.len() {
        let c = bytes[index];

        // We skip whitespace and comma
        if c.is_ascii_whitespace() || c == b',' {
            index += 1;
        } else {
            // Otherwise, try to consume a token.

            let (token, err) = read_form(file_id, bytes, &mut index);
            error = error.or(err);

            output.push(token);
        }
    }

    output.push(Token {
        contents: TokenContents::Eof,
        span: Span {
            file_id,
            start: index,
            end: index,
        },
    });

    (output, error)
}
fn read_form(file_id: FileId, bytes: &[u8], index: &mut usize) -> (Token, Option<CompilerError>) {
    let mut error = None;

    while let Some(b) = bytes.get(*index) {
        if b.is_ascii_whitespace() || *b as char == ',' {
            *index += 1;
        } else {
            break;
        }
    }

    match bytes[*index] {
        b'\'' => {
            let start = *index;
            *index += 1;
            (
                Token::new(
                    TokenContents::List(vec![
                        Token::new(
                            TokenContents::Sym("quote".to_string()),
                            Span::new(file_id, start, *index),
                        ),
                        read_form(file_id, bytes, index).0,
                    ]),
                    Span::new(file_id, start, *index),
                ),
                error,
            )
        }
        b'`' => {
            let start = *index;
            *index += 1;
            (
                Token::new(
                    TokenContents::List(vec![
                        Token::new(
                            TokenContents::Sym("quasiquote".to_string()),
                            Span::new(file_id, start, *index),
                        ),
                        read_form(file_id, bytes, index).0,
                    ]),
                    Span::new(file_id, start, *index),
                ),
                error,
            )
        }
        b'~' => {
            let start = *index;
            *index += 1;
            if bytes[*index] == b'@' {
                *index += 1;
                (
                    Token::new(
                        TokenContents::List(vec![
                            Token::new(
                                TokenContents::Sym("splice-unquote".to_string()),
                                Span::new(file_id, start, *index),
                            ),
                            read_form(file_id, bytes, index).0,
                        ]),
                        Span::new(file_id, start, *index),
                    ),
                    error,
                )
            } else {
                (
                    Token::new(
                        TokenContents::List(vec![
                            Token::new(
                                TokenContents::Sym("unquote".to_string()),
                                Span::new(file_id, start, *index),
                            ),
                            read_form(file_id, bytes, index).0,
                        ]),
                        Span::new(file_id, start, *index),
                    ),
                    error,
                )
            }
        }
        b'^' => {
            let start = *index;
            *index += 1;
            (
                Token::new(
                    TokenContents::List(vec![
                        Token::new(
                            TokenContents::Sym("with-meta".to_string()),
                            Span::new(file_id, start, *index),
                        ),
                        read_form(file_id, bytes, index).0,
                    ]),
                    Span::new(file_id, start, *index),
                ),
                error,
            )
        }
        b'@' => {
            let start = *index;
            *index += 1;
            (
                Token::new(
                    TokenContents::List(vec![
                        Token::new(
                            TokenContents::Sym("deref".to_string()),
                            Span::new(file_id, start, *index),
                        ),
                        read_form(file_id, bytes, index).0,
                    ]),
                    Span::new(file_id, start, *index),
                ),
                error,
            )
        }
        b')' => {
            let span = Span::new(file_id, *index, *index + 1);

            error = error.or(Some(CompilerError::ParserError(
                "unexpected ')'".to_string(),
                span,
            )));

            *index += 1;

            (Token::unknown(span), error)
        }
        b'(' => read_seq(file_id, bytes, index, b')'),
        b']' => {
            let span = Span::new(file_id, *index, *index + 1);

            error = error.or(Some(CompilerError::ParserError(
                "unexpected ']'".to_string(),
                span,
            )));

            *index += 1;

            (Token::unknown(span), error)
        }
        b'[' => read_seq(file_id, bytes, index, b']'),
        _ => read_atom(file_id, bytes, index),
    }
}

fn read_seq(
    file_id: FileId,
    bytes: &[u8],
    index: &mut usize,
    end: u8,
) -> (Token, Option<CompilerError>) {
    let mut seq = vec![];
    let start = *index;
    *index += 1;

    loop {
        let token = match bytes.get(*index) {
            Some(t) => t,
            None => {
                return (
                    Token::unknown(Span::new(file_id, start, *index)),
                    Some(CompilerError::ParserError(
                        format!("expected '{}', got EOF", end as char),
                        Span::new(file_id, start, *index),
                    )),
                )
            }
        };

        if *token as char == end as char {
            break;
        }

        seq.push(read_form(file_id, bytes, index).0);
    }

    *index += 1;

    match end {
        b')' => (
            Token::new(TokenContents::List(seq), Span::new(file_id, start, *index)),
            None,
        ),
        b']' => (
            Token::new(
                TokenContents::Vector(seq),
                Span::new(file_id, start, *index),
            ),
            None,
        ),
        _ => (
            Token::unknown(Span::new(file_id, start, *index)),
            Some(CompilerError::ParserError(
                "read_list unknown end value".to_string(),
                Span::new(file_id, start, *index),
            )),
        ),
    }
}

fn read_atom(file_id: FileId, bytes: &[u8], index: &mut usize) -> (Token, Option<CompilerError>) {
    let mut error = None;
    if bytes[*index].is_ascii_digit() {
        // Number
        let start = *index;
        while *index < bytes.len() && bytes[*index].is_ascii_digit() {
            *index += 1;
        }

        let str = String::from_utf8_lossy(&bytes[start..*index]);
        let number: Result<i64, _> = str.parse();

        match number {
            Ok(number) => (
                Token::new(
                    TokenContents::Number(number),
                    Span::new(file_id, start, *index),
                ),
                None,
            ),
            Err(_) => (
                Token::unknown(Span::new(file_id, start, *index)),
                Some(CompilerError::ParserError(
                    "could not parse int".to_string(),
                    Span::new(file_id, start, *index),
                )),
            ),
        }
    } else if bytes[*index] == b'"' {
        // Quoted string

        let start = *index;
        *index += 1;

        let mut escaped = false;

        while *index < bytes.len() && (escaped || bytes[*index] != b'"') {
            if !escaped && bytes[*index] == b'\\' {
                escaped = true;
            } else {
                escaped = false;
            }

            *index += 1;
        }

        if *index == bytes.len() || bytes[*index] != b'"' {
            error = error.or(Some(CompilerError::ParserError(
                "Expected end quote".to_string(),
                Span::new(file_id, start, start + 1),
            )));
        }

        // Everything but the quotes
        let str = String::from_utf8_lossy(&bytes[(start + 1)..(*index)]);

        let end = *index;
        *index += 1;

        (
            Token::new(
                TokenContents::Str(str.to_string()),
                Span::new(file_id, start, end),
            ),
            error,
        )
    } else if bytes[*index].is_ascii_alphabetic()
        || (bytes[*index] == b'.')
        || (bytes[*index] == b'*')
        || (bytes[*index] == b'+')
        || (bytes[*index] == b'!')
        || (bytes[*index] == b'-')
        || bytes[*index] == b'_'
        || bytes[*index] == b'?'
        || bytes[*index] == b'$'
        || bytes[*index] == b'%'
        || bytes[*index] == b'&'
        || bytes[*index] == b'='
        || bytes[*index] == b'<'
        || bytes[*index] == b'>'
    {
        // Symbol name
        let start = *index;
        *index += 1;

        let mut escaped = false;

        while *index < bytes.len()
            && (bytes[*index].is_ascii_alphanumeric()
                || (bytes[*index] == b'.')
                || (bytes[*index] == b'*')
                || (bytes[*index] == b'+')
                || (bytes[*index] == b'!')
                || (bytes[*index] == b'-')
                || bytes[*index] == b'_'
                || bytes[*index] == b'?'
                || bytes[*index] == b'$'
                || bytes[*index] == b'%'
                || bytes[*index] == b'&'
                || bytes[*index] == b'='
                || bytes[*index] == b'<'
                || bytes[*index] == b'>'
                || bytes[*index] == b':'
                || bytes[*index] == b'#'
                || bytes[*index] == b'/')
        {
            if !escaped && bytes[*index] == b'\\' {
                escaped = true;
            } else {
                escaped = false;
            }

            *index += 1;
        }

        // Everything but the quotes
        let str = String::from_utf8_lossy(&bytes[start..*index]);

        match str.to_string().as_str() {
            "nil" => (
                Token::new(TokenContents::Nil, Span::new(file_id, start, *index)),
                error,
            ),
            "true" => (
                Token::new(TokenContents::Bool(true), Span::new(file_id, start, *index)),
                error,
            ),
            "false" => (
                Token::new(
                    TokenContents::Bool(false),
                    Span::new(file_id, start, *index),
                ),
                error,
            ),
            _ => (
                Token::new(
                    TokenContents::Sym(str.to_string()),
                    Span::new(file_id, start, *index),
                ),
                error,
            ),
        }
    } else if bytes[*index] == b':' {
        // Symbol name
        let start = *index;
        *index += 1;

        let mut escaped = false;

        while *index < bytes.len()
            && (bytes[*index].is_ascii_alphanumeric()
                || (bytes[*index] == b'.')
                || (bytes[*index] == b'*')
                || (bytes[*index] == b'+')
                || (bytes[*index] == b'!')
                || (bytes[*index] == b'-')
                || bytes[*index] == b'_'
                || bytes[*index] == b'?'
                || bytes[*index] == b'$'
                || bytes[*index] == b'%'
                || bytes[*index] == b'&'
                || bytes[*index] == b'='
                || bytes[*index] == b'<'
                || bytes[*index] == b'>'
                || bytes[*index] == b'/')
        {
            if !escaped && bytes[*index] == b'\\' {
                escaped = true;
            } else {
                escaped = false;
            }

            *index += 1;
        }

        // Everything but the quotes
        let str = String::from_utf8_lossy(&bytes[start..*index]);

        (
            Token::new(
                TokenContents::Keyword(str.to_string()),
                Span::new(file_id, start, *index),
            ),
            error,
        )
    } else {
        let span = Span::new(file_id, *index, *index + 1);

        error = error.or(Some(CompilerError::ParserError(
            "unknown character".to_string(),
            span,
        )));

        *index += 1;

        (Token::unknown(span), error)
    }
>>>>>>> Stashed changes
}
