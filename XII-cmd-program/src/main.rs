use std::{env, fs, process};
use std::error::Error;
use XII_cmd_program::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config =Config::build(&args).unwrap_or_else(|err|{
        println!("problem parsing args: {}", err);
        process::exit(1);
    });

    if let Err(e) = config.run(){
        println!("problem running: {}", e);
        process::exit(1);
    }
}