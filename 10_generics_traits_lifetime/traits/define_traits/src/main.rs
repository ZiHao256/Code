pub trait Summary {
    fn summerize(&self) -> String{
        format!("Read more from {}", self.summerize_author())
    }

    fn summerize_author(&self) -> String;
}

#[derive(Debug)]
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}


impl Summary for NewsArticle{
    fn summerize_author(&self) -> String{
        format!("@{}", self.author)
    }
}

#[derive(Debug)]
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet{
    fn summerize(&self) -> String{
        format!("{} by {}", self.content, self.username)
    }

    fn summerize_author(&self) -> String{
        format!("@{}", self.username)
    }
}

fn main() {
    let news = NewsArticle{
        headline: String::from("Hello"),
        location: String::from("Xian"),
        author: String::from("Alex"),
        content: String::from("xxxxx")
    };
    
    println!("{}", news.summerize());

    println!("{}", notify(&news, &news));

    // dbg!("return_summariable:", return_summariable());
}


fn notify<T:Summary>(item1: &T, item2: &T) -> String{
    format!("notify: item1:{} \nitem2:{}",item1.summerize(), item2.summerize())
}

fn return_summariable() -> impl Summary{
    let news = NewsArticle{
        headline: String::from("Hello"),
        location: String::from("Xian"),
        author: String::from("Alex"),
        content: String::from("xxxxx")
    };
    news
}