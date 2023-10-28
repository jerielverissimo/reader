use reader::{
    error::ReadError,
    reader::{read, Reader},
};

fn main() {
    let mut files = vec![];

    for arg in std::env::args().skip(1) {
        let fname = arg;
        let contents = std::fs::read(fname.clone()).unwrap();

        files.push((fname.to_string(), contents.clone()));
    }

    for file in files {
        let content = &String::from_utf8(file.1).unwrap();
        let mut reader = Reader::from(content);

        loop {
            match read(&mut reader) {
                Ok(items) => println!("Parsed items: {:?}", items),
                Err(ReadError::EndOfInput) => break,
                Err(msg) => println!("Error: {}", msg),
            }
        }
    }
}
