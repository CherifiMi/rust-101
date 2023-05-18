use std::{env, fs};

fn main() {

    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    let content = fs::read_to_string(file_path)
        .expect("should have been able to read the text");

    println!("\nsearching for: {}", query);
    println!("if file: {}", file_path);
    println!("with text:\n{}", content);
}
