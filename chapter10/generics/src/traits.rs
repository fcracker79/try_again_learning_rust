use std::fmt;

pub trait BasicSummary {
    fn summarize_author(&self) -> String;

}
pub trait Summary: BasicSummary {
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
    fn content(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl BasicSummary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

impl Summary for NewsArticle {
    fn content(&self) -> String {
        format!("@{}", self.content)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl BasicSummary for Tweet {
    fn summarize_author(&self) -> String {
        format!("{}", self.username)
    }
}

impl Summary for Tweet {
    fn content(&self) -> String {
        format!("{}", self.content)
    }

}

impl fmt::Display for Tweet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.username, self.content)
    }
}

impl fmt::Display for NewsArticle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.author, self.headline)
    }
}
pub fn notify(item: &(impl Summary + fmt::Display)) {
    println!("Breaking news! {}", item.summarize());
    notify_content(item);
    notify_with_display(item);
}

pub fn notify_content<T>(item: &T) where T: Summary + fmt::Display {
    println!("Contents: {}", item.content());
}

pub fn notify_with_display<T: fmt::Display>(item: &T) {
    println!("Display: {}", item);
}

pub fn notify_with_display_noref<T: fmt::Display>(item: T) {
    println!("Display: {}", item);
}

pub fn notify_with_display_noref2(item: impl fmt::Display) {
    println!("Display: {}", item);
}

pub fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };
    let article = NewsArticle {
        headline: "Great news!".to_string(),
        location: "North pole".to_string(),
        author: "John Burns".to_string(),
        content: "Quite a lot\nof info!".to_string()
    };

    println!("1 new tweet: {}", tweet.summarize());
    notify(&tweet);

    println!("1 news article: {}", article.summarize());
    notify(&article);

    notify_with_display_noref(article);
    notify_with_display_noref2(tweet);
}