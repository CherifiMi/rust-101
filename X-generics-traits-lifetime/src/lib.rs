use std::fmt::format;

pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String{
     "(read more......)".to_string()
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("this summary is by {}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("this tweet is by {}", self.username)
    }


    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
