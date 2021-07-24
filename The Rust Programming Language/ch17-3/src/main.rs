// Implementing an Object-Oriented Design Pattern
//
// We’ll implement a blog post workflow in an incremental way. The blog’s final functionality will look like this:
// 1. A blog post starts as an empty draft.
// 2. When the draft is done, a review of the post is requested.
// 3. When the post is approved, it gets published.
// 4. Only published blog posts return content to print, so unapproved posts can’t accidentally be published.

use blog::Post;

fn main() {
    println!("ch17-3!");
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
