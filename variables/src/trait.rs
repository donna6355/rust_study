pub trait Summary {
    fn summarize(&self) -> String; //method signature
    fn author(&self) -> String;
    fn say_ho(&self) -> String {
        //specify default implementation
        String::from("Say Ho!!! from {self.author}") //call the default implementation from an overriding implementation
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    //Each type implementing this trait must provide its own custom behavior
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    //Each type implementing this trait must provide its own custom behavior
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    fn author(&self) -> String {
        format!("@{}", self.username)
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.say_ho());
}

//Trait Bound Syntax
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
pub fn notify<T: Summary>(itme: &T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify(item1: &impl Summary, item2: &impl Summary) {} // can have diff arg
pub fn notify<T: Summary>(item1: &T, item2: &T) {} // must have same type for arg

fn returns_summarizable() -> impl Summary {
    //returns some type that implements the Summary trait without naming the concrete type
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

use std::fmt::Display;
//Using Trait Bounds to Conditionally Implement Methods
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn lifetime_main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");
}
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
