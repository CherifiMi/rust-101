use super::*;

#[test]
fn one_result() {
    let config = Config {
        query: "duct".to_string(),
        file_path: "test.txt".to_string(),
    };
    assert_eq!(vec!["safe, fast, productive."], config.search());
}