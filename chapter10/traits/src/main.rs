

pub trait Summary {
    fn summrize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summrize(&self) -> String {
        format!("{} by {}", self.headline, self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summrize(&self) -> String {
        format!("this tweet is from {}", self.username)
    }
}

fn main() {

    let tweet = Tweet {
        username: String::from("S.O.D"),
        content: String::from("Hello rust"),
        reply: false,
        retweet: false,
    };

    println!("summary of tweet {}", tweet.summrize());
}
