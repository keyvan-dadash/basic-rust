

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


struct Pair<T>
{
    x: T,
    y: T,
}

impl<T> Pair<T> 
{
    fn display(&self) {
        println!("Pair struct from generic function");
    }
}

impl<T> Pair<T>
where
    T: Summary
{
    fn display_summary(&self) {
        println!("Pair struct summary for x={} and y={}", self.x.summrize(), self.y.summrize());
    }
}


trait Another {
    fn simple_print(&self) {
        println!("blanket implementation");
    }
}

impl<T> Another for T
where
    T: Summary
{

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


    println!();
    println!();
    println!();

    println!("start conditionally method implementation");

    let p_without = Pair{x: 5, y: 6};
    p_without.display();
    // p_without.display_summary(); //wont compile

    let tweet1 = Tweet {
        username: String::from("Another?"),
        content: String::from("Bye rust"),
        reply: false,
        retweet: false,
    };

    let p_with = Pair{x: tweet, y: tweet1};
    p_with.display();
    p_with.display_summary();

    println!();
    println!();
    println!();

    println!("start blanket implementation");
    let tweet2 = Tweet {
        username: String::from("For blanket"),
        content: String::from("Bye rust"),
        reply: false,
        retweet: false,
    };

    tweet2.simple_print();
}
