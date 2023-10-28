use std::io;

use reader::reader::{read, Expr};

fn main() {
    // Read a line of input from the user
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Parse the input into a vector of ParserItem
    let items: Vec<Expr> = read(input.trim());

    // Print the parsed items
    println!("Parsed items: {:?}", items);
}
