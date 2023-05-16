use std::fs::File;
use std::io::{Error, ErrorKind};
use std::str;

fn main() {
    println!("Hello, world!");

    let file = File::open("hi.txt");
    let file = match file {
        Ok(file) => {file}
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                match File::create("hi.txt") {
                    Ok(file) => {file}
                    Err(error) => {
                        panic!("no file{:?}", error);
                    }
                }
            }
            _ => {
                panic!("no file{:?}", error);
            }
        }
    };
}
