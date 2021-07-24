// Defining Post and Creating a New Instance in the Draft State

pub struct Post {
  content: String,
}

impl Post {
  pub fn new() -> Post {
    Post {
      state: Some(Box::new(Draft {})),
      content: String::new(),
    }
  }
}

trait State {}

struct Draft {}

impl State for Draft {}

// Storing the Text of the Post Content

impl Post {
  // --snip--
  pub fn add_text(&mut self, text: &str) {
    self.content.push_str(text);
  }
}

// Ensuring the Content of a Draft Post Is Empty

impl Post {
  // --snip--
  pub fn content(&self) -> &str {
    ""
  }
}

// Requesting a Review of the Post Changes Its State
impl Post {
  // --snip--
  pub fn request_review(&mut self) {
    if let Some(s) = self.state.take() {
      self.state = Some(s.request_review())
    }
  }
}

trait State {
  fn request_review(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}

impl State for Draft {
  fn request_review(self: Box<Self>) -> Box<dyn State> {
    Box::new(PendingReview {})
  }
}

struct PendingReview {}

impl State for PendingReview {
  fn request_review(self: Box<Self>) -> Box<dyn State> {
    self
  }
}

// Adding the approve Method that Changes the Behavior of content
impl Post {
  // --snip--
  pub fn approve(&mut self) {
    if let Some(s) = self.state.take() {
      self.state = Some(s.approve())
    }
  }
}

trait State {
  fn request_review(self: Box<Self>) -> Box<dyn State>;
  fn approve(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}

impl State for Draft {
  // --snip--
  fn approve(self: Box<Self>) -> Box<dyn State> {
    self
  }
}

struct PendingReview {}

impl State for PendingReview {
  // --snip--
  fn approve(self: Box<Self>) -> Box<dyn State> {
    Box::new(Published {})
  }
}

struct Published {}

impl State for Published {
  fn request_review(self: Box<Self>) -> Box<dyn State> {
    self
  }

  fn approve(self: Box<Self>) -> Box<dyn State> {
    self
  }
}

impl Post {
  // --snip--
  pub fn content(&self) -> &str {
    self.state.as_ref().unwrap().content(self)
  }
  // --snip--
}

trait State {
  // --snip--
  fn content<'a>(&self, post: &'a Post) -> &'a str {
    ""
  }
}

// --snip--
struct Published {}

impl State for Published {
  // --snip--
  fn content<'a>(&self, post: &'a Post) -> &'a str {
    &post.content
  }
}

// Trade-offs of the State Pattern

// Add a reject method that changes the post’s state from PendingReview back to Draft.
// Require two calls to approve before the state can be changed to Published.
// Allow users to add text content only when a post is in the Draft state. Hint: have the state object responsible for what might change about the content but not responsible for modifying the Post.

// Encoding States and Behavior as Types
pub struct Post {
  content: String,
}

pub struct DraftPost {
  content: String,
}

impl Post {
  pub fn new() -> DraftPost {
    DraftPost {
      content: String::new(),
    }
  }

  pub fn content(&self) -> &str {
    &self.content
  }
}

impl DraftPost {
  pub fn add_text(&mut self, text: &str) {
    self.content.push_str(text);
  }
}

impl DraftPost {
  // --snip--
  pub fn request_review(self) -> PendingReviewPost {
    PendingReviewPost {
      content: self.content,
    }
  }
}

pub struct PendingReviewPost {
  content: String,
}

impl PendingReviewPost {
  pub fn approve(self) -> Post {
    Post {
      content: self.content,
    }
  }
}
