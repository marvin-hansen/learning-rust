use std::fmt::{Debug, Display};
use crate::types::Summary;

mod types;
mod pair;

fn main() {
    test_summary_trait();
    test_summarize_return_type();
    test_pair();
}

fn test_pair(){
    let p = pair::Pair::new(1, 2);
    p.cmp_display()
}

fn test_summary_trait() {
    let tweet = types::Tweet {
        username: String::from("johndoe"),
        content: "Hello Twitter".to_string(),
        reply: false,
        retweet: false,
    };

    let article = types::NewsArticle {
        author: "John Doe".to_string(),
        content: "The sky is not actually falling".to_string(),
        headline: "The sky is falling".to_string(),
    };

    println!("Tweet summary: {}", tweet.summarize());
    println!("Article summary: {}", article.summarize());

    notify(&article);
}

fn test_summarize_return_type(){
    // here we can call summary because the return type is guaranteed
    println!("Tweet summary: {}", returns_summarizable().summarize());
}

// returns any type that implements or adheres to Summary
fn returns_summarizable() -> impl types::Summary{
    types::Tweet {
        username: String::from("horse_ebooks"),
        content: "As you already know, people! ".to_string(),
        reply: false,
        retweet: false,
    }
}

// impl syntax for simplicity
pub fn notify(item: &(impl Summary + Debug) ) {
    println!("Breaking news: {}", item.summarize());
}

// Generic syntax for more expressiveness
pub fn notify_two<T: Summary+ Debug>(_item1: &T, _item2: &T) {
    //
}

// generic syntax may get cluttered with more constraints
fn _some_function<T: Display + Clone, U: Clone + Debug>(_t: &T, _u: &U) -> i32 {
    42
}

// Where syntax de-clutters type constraints
fn _some_function_where<T, U>(_: &T, _u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
    42
}
