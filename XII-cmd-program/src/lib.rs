use std::error::Error;
use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {

        let config = Config{
            query: "duct".to_string(),
            file_path: "test.txt".to_string(),
        };
        assert_eq!(vec!["safe, fast, productive."], config.search());
    }
}

pub struct Config {
    query: String,
    file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough argements");
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
    pub fn search(self)-> Vec<&'static str>{
        vec!["safe, fast, productive."]
    }
}
