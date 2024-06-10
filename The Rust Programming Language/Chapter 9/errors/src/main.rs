use std::fs::File;
use std::io::{Error, ErrorKind, Read};
use std::ops::Add;

fn reads_file() -> Result<String, Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut string = String::new();

    match username_file.read_to_string(&mut string) {
        Ok(_) => Ok(string),
        Err(error) => return Err(Error::new(ErrorKind::InvalidInput, "testing error2")),
    }
}

fn main() {
    match reads_file() {
        Ok(string) => {
            println!("{}", string);
        }
        Err(e) => { println!("{}", e); }
    }


    // panic!("panic :O");
}


