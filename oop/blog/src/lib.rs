
// Encoding States and Behavior as Types
// We’ll show you how to rethink the state pattern to get a different set of trade-offs.
// Rather than encapsulating the states and transitions completely
// so outside code has no knowledge of them,
// we’ll encode the states into different types.
// Consequently, Rust’s type checking system
// will prevent attempts to use draft posts where only published posts are allowed by issuing a compiler error.

// Both the Post and DraftPost structs have a private content field that stores the blog post text.
// The structs no longer have the state field because we’re moving the encoding of the state
// to the types of the structs.
// The Post struct will represent a published post, and it has a content method that returns the content.

pub struct DraftPost {
    content: String,
}

impl DraftPost {

    // We still enable the creation of new posts in the draft state
    // using Post::new and the ability to add text to the post’s content.
    // But instead of having a content method on a draft post that returns an empty string,
    // we’ll make it so draft posts don’t have the content method at all.
    // That way, if we try to get a draft post’s content,
    // we’ll get a compiler error telling us the method doesn’t exist.
    // As a result, it will be impossible for us to accidentally display draft post content in production,
    // because that code won’t even compile
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

// o how do we get a published post?
// We want to enforce the rule that a draft post
// has to be reviewed and approved before it can be published.
// A post in the pending review state should still not display any content.
// Let’s implement these constraints by adding another struct, PendingReviewPost,
// defining the request_review method on DraftPost to return a PendingReviewPost,
// and defining an approve method on PendingReviewPost to return a Post

pub struct PendingReviewPost {
    content: String,
}

// The request_review and approve methods take ownership of self,
// thus consuming the DraftPost and PendingReviewPost instances and
// transforming them into a PendingReviewPost and a published Post, respectively.
// This way, we won’t have any lingering DraftPost instances after we’ve called request_review on them, and so forth.
// The PendingReviewPost struct doesn’t have a content method defined on it,
// so attempting to read its content results in a compiler error, as with DraftPost.
// Because the only way to get a published Post instance that does have a content method defined
// is to call the approve method on a PendingReviewPost,
// and the only way to get a PendingReviewPost is to call the request_review method on a DraftPost,
// we’ve now encoded the blog post workflow into the type system.

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}


pub struct Post{
    content: String,
}

impl Post {
    // We still have a Post::new function,
    // but instead of returning an instance of Post, it returns an instance of DraftPost.
    // Because content is private and there aren’t any functions that return Post,
    // it’s not possible to create an instance of Post right now.

    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

