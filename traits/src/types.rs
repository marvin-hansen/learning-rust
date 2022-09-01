pub trait Summary {
    // Signature that must be implemented
    fn summarize_author(&self) -> String;

    // Default implementation for summary
    fn summarize(&self) -> String {
        return format!("(Read more from {} ...)", self.summarize_author());
    }
}


#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct NewsArticle {
    pub author: String,
    pub content: String,
    pub headline: String,
}


impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        return format!("{}", self.author);
    }

    fn summarize(&self) -> String {
        return format!("{} by:  {}", self.headline, self.author).to_string();
    }
}


#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}


impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        return format!("@{}", self.username);
    }

    fn summarize(&self) -> String {
        return format!("{}: {}", self.summarize_author(), self.content).to_string();
    }
}
