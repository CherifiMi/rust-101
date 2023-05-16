use std::fs::File;
use std::io::{Error, ErrorKind};
use std::str;

fn main() {
    println!("Hello, world!");

    let file = File::open("hillo.txt");
    let file = match file {
        Ok(file) => {file}
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                File::create("hillo.txt").unwrap_or_else(|error|{
                    panic!("no file{:?}", error);
                })
            }
            _ => {
                panic!("no file{:?}", error);
            }
        }
    };

    //unwrap panics if not found
    let file2 =
        File::open("mito.txt").unwrap();

    // expect same as unwrap, dose not use default err panic message
    let file2 =
        File::open("mito.txt").expect("smt wrong happened");

    dbg!(file2);
}
