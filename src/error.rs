use crate::reader::{Reader, Span};

#[derive(Debug)]
pub enum CompilerError {
    IOError(std::io::Error),
    StringError(String),
    ParserError(String, Span),
    ParserErrorWithHint(String, Span, String, Span),
}

impl From<std::io::Error> for CompilerError {
    fn from(x: std::io::Error) -> Self {
        CompilerError::IOError(x)
    }
}

#[derive(Debug)]
pub enum MessageSeverity {
    Hint,
    Error,
}

impl MessageSeverity {
    pub fn name(&self) -> String {
        match self {
            MessageSeverity::Hint => "hint".to_string(),
            MessageSeverity::Error => "error".to_string(),
        }
    }

    pub fn ansi_color_code(&self) -> String {
        match self {
            MessageSeverity::Hint => "94".to_string(),  // Bright Blue
            MessageSeverity::Error => "31".to_string(), // Red
        }
    }
}

pub fn display_message_with_span(
    severity: MessageSeverity,
    reader: &Reader,
    msg: &str,
    span: Span,
) {
    println!(
        "\u{001b}[{}m{}\u{001b}[0m: {}",
        severity.ansi_color_code(),
        severity.name(),
        msg
    );

    let file_contents = reader.get_file_contents(span.file_id);
    let file_name = reader.get_file_name(span.file_id);

    let line_spans = line_spans(file_contents);

    let mut line_index = 0;
    let largest_line_number = line_spans.len();

    let width = format!("{}", largest_line_number).len();

    while line_index < line_spans.len() {
        if span.start >= line_spans[line_index].0 && span.start <= line_spans[line_index].1 {
            let end_index = span.start - line_spans[line_index].0;
            println!(
                "\u{001b}[{}m-->\u{001b}[0m  \u{001b}[33m{}:{}:{}\u{001b}[0m",
                "38;5;12",
                file_name.display(),
                line_index + 1,
                end_index + 1
            );
            if line_index > 0 {
                print_source_line(
                    &severity,
                    file_contents,
                    line_spans[line_index - 1],
                    span,
                    line_index - 1,
                    largest_line_number,
                );
            }
            print_source_line(
                &severity,
                file_contents,
                line_spans[line_index],
                span,
                line_index,
                largest_line_number,
            );

            print!(
                "{}",
                " ".repeat(span.start - line_spans[line_index].0 + width + 4)
            );
            println!(
                "\u{001b}[{}m^- {}\u{001b}[0m",
                severity.ansi_color_code(),
                msg
            );

            while line_index < line_spans.len() && span.end > line_spans[line_index].0 {
                line_index += 1;
                if line_index >= line_spans.len() {
                    break;
                }
                print_source_line(
                    &severity,
                    file_contents,
                    line_spans[line_index],
                    span,
                    line_index,
                    largest_line_number,
                );
            }

            break;
        } else {
            line_index += 1
        }
    }
}

fn print_source_line(
    severity: &MessageSeverity,
    file_contents: &[u8],
    file_span: (usize, usize),
    error_span: Span,
    line_number: usize,
    largest_line_number: usize,
) {
    let mut index = file_span.0;

    let width = format!("{}", largest_line_number).len();

    print!(" \u{001b}[38;5;12m{:<width$} | \u{001b}[0m", line_number);
    while index <= file_span.1 {
        let c;
        if index < file_span.1 {
            c = file_contents[index];
        } else if error_span.start == error_span.end && index == error_span.start {
            c = b'_';
        } else {
            c = b' ';
        }

        if (index >= error_span.start && index < error_span.end)
            || (error_span.start == error_span.end && index == error_span.start)
        {
            // In the error span

            print!("\u{001b}[{}m{}", severity.ansi_color_code(), c as char)
        } else {
            print!("\u{001b}[0m{}", c as char)
        }
        index += 1;
    }
    println!();
}

fn line_spans(contents: &[u8]) -> Vec<(usize, usize)> {
    let mut idx = 0;
    let mut output = vec![];

    let mut line = idx;
    while idx < contents.len() {
        if contents[idx] == b'\n' {
            output.push((line, idx));
            line = idx + 1;
        }
        idx += 1;
    }
    if line < idx {
        output.push((line, idx));
    }

    output
}
