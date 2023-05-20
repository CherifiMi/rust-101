use std::error::Error;
use std::fs;

#[cfg(test)]
mod tests;

///-------------

pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool
}
impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case =  args.get(3).unwrap_or(&"false".to_string()).contains("true");
        Ok(Config { query, file_path, ignore_case})
    }
    pub fn run(self) -> Result<(), Box<dyn Error>> {
        let content = fs::read_to_string(&self.file_path)?;

        match self.ignore_case {
            true => {
                for line in search_insensitive(&self.query, &content){
                    println!("{line}")
                }
            }
            false => {
                for line in search(&self.query, &content){
                    println!("{line}")
                }
            }
        }

        Ok(())
    }
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines(){
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

fn search_insensitive<'a>(q: &str, c: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in c.lines(){
        if line.to_lowercase().contains(&q.to_lowercase()) {
            results.push(line)
        }
    }

    results
}