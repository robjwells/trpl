use std::fmt::Display;

fn main() {
    let s = 3.to_string();
    println!("Hello, world! {s}");
}

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location,)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// impl Summary for Tweet {
//     fn summarize(&self) -> String {
//         format!("{}: {}", self.username, self.content,)
//     }
// }

impl Summary for Tweet {}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn n(item1: &impl Summary, item2: &impl Summary) {
    println!("{} -- {}", item1.summarize(), item2.summarize());
}

pub fn m<T: Summary>(item1: &T, item2: &T) {
    println!("{} -- {}", item1.summarize(), item2.summarize());
}

pub fn o(item: &(impl Summary + Display)) {
    println!("{} -- {}", item, item.summarize());
}

pub fn p<T>(item: &T)
where
    T: Summary + Display,
{
    println!("{} -- {}", item, item.summarize());
}

pub fn returns_summarizable(switch: bool) -> Box<dyn Summary> {
    if switch {
        Box::new(Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        })
    } else {
        Box::new(NewsArticle {
            headline: "".to_owned(),
            location: "".to_owned(),
            author: "".to_owned(),
            content: "".to_owned(),
        })
    }
}
