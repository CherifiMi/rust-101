use super::*;

#[test]
fn one_result() {

        let query =  "duct";
        let content =  "Rust:\
safe, fast, productive.\
Pick three.";

    assert_eq!(vec!["safe, fast, productive."], search(query, content));
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    for line in contents.lines(){

    }

    vec![]
}