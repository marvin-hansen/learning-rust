pub trait Summary {
    // Signature that must be implemented
    fn summarize_author(&self) -> String;

    // Default implementation for summary
    fn summarize(&self) -> String {
        format!("(Read more from {} ...)", self.summarize_author())
    }
}


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct NewsArticle {
    pub author: String,
    pub content: String,
    pub headline: String,
}


impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        self.author.to_string()
    }

    fn summarize(&self) -> String {
        format!("{} by:  {}", self.headline, self.author)
    }
}


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}


impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.summarize_author(), self.content)
    }
}
