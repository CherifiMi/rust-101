use std::error::Error;
use std::{env, fs, process};
use XII_cmd_program::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("problem parsing args: {}", err);
        process::exit(1);
    });

    if let Err(e) = config.run() {
        eprintln!("problem running: {}", e);
        process::exit(1);
    }
}
