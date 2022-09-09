use blog::*;

fn main() {

    // The request_review and approve methods return new instances
    // rather than modifying the struct they’re called on,
    // so we need to add more let post = shadowing assignments to save the returned instances.
    // We also can’t have the assertions about the draft and pending review posts’ contents
    // be empty strings, nor do we need them: we can’t compile code that tries to use the content of posts in those states any longer.

    let mut post: DraftPost = Post::new();
    post.add_text("I ate a salad for lunch today");
    let post: PendingReviewPost = post.request_review();
    let post: Post = post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());


    // The changes we needed to make to main to reassign post mean that this implementation
    // doesn’t quite follow the object-oriented state pattern anymore:
    // the transformations between the states are no longer encapsulated entirely within the Post implementation.
    // However, our gain is that invalid states are now impossible because of the type system
    // and the type checking that happens at compile time!
    // This ensures that certain bugs, such as display of the content of an unpublished post,
    // will be discovered before they make it to production.

}

// We’ve seen that even though Rust is capable of implementing object-oriented design patterns,
// other patterns, such as encoding state into the type system,
// are also available in Rust. These patterns have different trade-offs.
// Although you might be very familiar with object-oriented patterns,
// rethinking the problem to take advantage of Rust’s features can provide benefits,
// such as preventing some bugs at compile time.
// Object-oriented patterns won’t always be the best solution in Rust due to certain features,
// like ownership, that object-oriented languages don’t have.
