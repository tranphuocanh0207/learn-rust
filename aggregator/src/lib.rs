use std::fmt::Display;

#[derive(Debug)]
pub struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    pub fn new(x: T, y: T) -> Self {
        Self {
            x,
            y
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    pub fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest number is x = {}", self.x);
        } else {
            println!("The largest number is y = {}", self.y);
        }
    }
}

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }

    fn recall_summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn recall_summarize(&self) -> String {
        format!("Read more from {}", self.summarize())
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn recall_summarize(&self) -> String {
        format!("Read more from {}", self.summarize())
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify_1(item: &impl Summary, item2: &impl Summary) {
    println!("Breaking news! {} - {}", item.summarize(), item2.summarize());
}

pub fn notify_2<T: Summary>(item: &T, item2: &T) {
    println!("Breaking news! {} - {}", item.summarize(), item2.summarize());
}

//pub fn notify_3(item: &(impl Summary + Display) {}
