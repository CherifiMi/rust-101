use std::fs::File;
use std::io::{Error, ErrorKind, Read};
use std::{fs, str};

fn main() {
    err_propagating();
}

fn err_propagating() {

    let text = "my name is boox\nmy fav is patato\nyoo";

    fn last_char_of_first_line(text: &str) -> Option<char>{
        text.lines().next()?.chars().last()
    }

    match last_char_of_first_line(text) {
        Some(s) => {
            println!("last character is{}", s);

        }
        None => {
            println!("there is nothing");
        }
    }

    //------------
    match read_username4() {
        Ok(a) => {
            println!("file is full with {}", a );
        }
        Err(e) => {
            println!("######error is {}",e);
        }
    }

    fn read_username4() -> Result<String, Error> {
        fs::read_to_string("hi.txt")
    }
    fn read_username3() -> Result<String, Error> {
        let mut username = String::new();
        File::open("hi.txt")?.read_to_string(&mut username)?;

        Ok(username)
    }
    fn read_username2() -> Result<String, Error> {
        let mut username_file = File::open("hi.txt")?;
        let mut username = String::new();
        username_file.read_to_string(&mut username)?;
        Ok(username)
    }
    fn read_username() -> Result<String, Error> {
        let username_file_result = File::open("hi.txt");
        let mut username_file = match username_file_result {
            Ok(file) => { file }
            Err(e) => return Err(e),
        };

        let mut username = String::new();

        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e)
        }
    }
}

fn err() {
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
        File::open("hillo.txt").unwrap();

    // expect same as unwrap, dose not use default err panic message
    let file2 =
        File::open("hillo.txt").expect("smt wrong happened");

    dbg!(file2);

}
