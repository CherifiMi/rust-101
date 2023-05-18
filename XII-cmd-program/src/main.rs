use std::{env, fs, process};
use std::error::Error;

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
    /*config.run().unwrap_or_else(|err|{
        println!("problem running: {}", err);
        process::exit(1);
    });;*/
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough argements");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { query, file_path })
    }
    fn run(self)-> Result<(), Box<dyn Error>>{
        let content = fs::read_to_string(&self.file_path)?;

        println!("Searching for {}", self.query);
        println!("In file {}", self.file_path);
        println!("{content}");

        Ok(())
    }
}

