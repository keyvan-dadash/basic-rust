

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

pub fn notify_param(item: &impl Summary) {
    println!("Notify with params {}", item.summrize());
}

pub fn notify_by_generic<T: Summary>(item: &T) 
{
    println!("Notify with generic on function sig {}", item.summrize());
}

pub fn notify_by_where<T>(item: &T)
where
    T: Summary
{
    println!("Notify with where {}", item.summrize());
}

fn main() {

    let tweet = Tweet {
        username: String::from("S.O.D"),
        content: String::from("Hello rust"),
        reply: false,
        retweet: false,
    };

    println!("summary of tweet {}", tweet.summrize());

    println!("here we are going to test summary as params");

    notify_param(&tweet);
    notify_by_generic(&tweet);
    notify_by_where(&tweet);
}
