use std::io;

use reader::reader::{read, Reader};

fn main() {
    // Read a line of input from the user
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let mut reader = Reader::from(input.trim());

    // Parse the input into a vector of ParserItem
    match read(&mut reader) {
        Ok(items) => println!("Parsed items: {:?}", items),
        Err(msg) => println!("Error: {}", msg),
    }
}
