use std::fs::{File, Metadata};
use std::io::{BufWriter, Error, Write};

#[tokio::main]
async fn main() {
    write_to_file(256).await.expect("TODO: panic message");
    another_function_two(5, 5, '\u{2665}').await;
}

async fn another_function_two(x: i32, y: i32, char: char) {
    println!("{} {}", x * y, char);
}

async fn write_to_file(number: usize) -> Result<(), Error> {
    let file: File = File::create("file.txt").expect("unable to create file");
    let mut buf = BufWriter::new(&file);

    for x in 0..number {
        buf.write((x.to_string()+"\r\n").as_bytes()).expect("TODO: panic message");
    }

    file.sync_all()?;
    buf.flush()?;

    file.metadata()
        .iter()
        .for_each(|v: &Metadata| println!("{:?}", v));

    Ok(())
}