use error::{display_message_with_span, CompilerError, MessageSeverity};
use reader::Reader;

pub mod error;
pub mod reader;

pub fn start() {
    let mut reader = Reader::new();
    for arg in std::env::args().skip(1) {
        match reader.read(&arg) {
            Ok(v) => {
                println!("{:#?}", v);
            }
            Err(err) => match err {
                CompilerError::IOError(ioe) => println!("IO Error: {}", ioe),
                CompilerError::ParserError(msg, span) => {
                    display_message_with_span(MessageSeverity::Error, &reader, &msg, span)
                }
                CompilerError::StringError(_) => todo!(),
                CompilerError::ParserErrorWithHint(_, _, _, _) => todo!(),
            },
        }
    }
}
