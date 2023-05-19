use std::error::Error;
use std::fs;

#[cfg(test)]
mod tests;

///-------------

pub struct Config {
    query: String,
    file_path: String,
}
impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { query, file_path })
    }
    pub fn run(self) -> Result<(), Box<dyn Error>> {
        let content = fs::read_to_string(&self.file_path)?;

        println!("Searching for {}", self.query);
        println!("In file {}", self.file_path);
        println!("{content}");

        Ok(())
    }
}
