use std::fmt::{Display, format};

pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
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

impl Summary for i32 {
    fn summarize_author(&self) -> String {
        format!("mito i32 {}", self)
    }

    fn summarize(&self) -> String {
        format!("{} is the summary", self)
    }
}
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("this tweet is by {}", self.username)
    }
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify(item: &impl Summary) {
    println!("BREAKING NEWS!!! {}", item.summarize())

}
pub fn notify2<T: Summary>(item: &T){
    println!("BREAKING NEWS!!! {}", item.summarize())
}

pub fn notify3<T: Summary+PartialOrd+Display>(item: &T){
    println!("this item is an i32 and a summary instance:: {}",item)
}
pub fn notify4<T>(item: &T)
where
    T: Summary + PartialOrd + Display,
{
    println!("this item is an i32 and a summary instance:: {}",item)
}

pub fn return_summary() -> impl Summary{
    Tweet{
        username: "hiiiiiii".to_string(),
        content: "hooooooooooo".to_string(),
        reply: false,
        retweet: false,
    }
}

