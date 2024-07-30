pub trait Summary{
    fn summarize(&self) -> String; //method signature
    fn author(&self) -> String;
    fn say_ho(&self) -> String { //specify default implementation
        String::from("Say Ho!!! from {self.author}") //call the default implementation from an overriding implementation
    }
}

pub struct NewsArticle {
    pub headline:String,
    pub location:String,
    pub author:String,
    pub content:String,
}

impl Summary for NewsArticle { //Each type implementing this trait must provide its own custom behavior
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

impl Summary for Tweet { //Each type implementing this trait must provide its own custom behavior
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    fn author(&self) -> String {
        format!("@{}", self.username)
    }
}

fn main () {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.say_ho());
    
}

//Trait Bound Syntax
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
pub fn notify<T:Summary> (itme: &T){
    println!("Breaking news! {}", item.summarize());
}