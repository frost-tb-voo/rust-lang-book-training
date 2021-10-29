pub struct Post {
    state: Option<Box<State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
    pub fn add_text(&mut self, text: &str) {
        if *self.state.as_ref().unwrap().can_edit_text(&self) {
            self.content.push_str(text);
        }
    }
    pub fn clear_text(&mut self) {
        if *self.state.as_ref().unwrap().can_edit_text(&self) {
            self.content.clear()
        }
    }
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(&self)
    }
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject())
        }
    }
}

trait State {
    // cannot impl methods which return self
    fn request_review(self: Box<Self>) -> Box<State>;
    fn approve(self: Box<Self>) -> Box<State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
    fn reject(self: Box<Self>) -> Box<State>;
    fn can_edit_text<'a>(&self, post: &'a Post) -> &'a bool {
        &false
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<State> {
        Box::new(PendingReview {})
    }
    fn approve(self: Box<Self>) -> Box<State> {
        // invalid action
        self
    }
    fn reject(self: Box<Self>) -> Box<State> {
        // invalid action
        self
    }
    fn can_edit_text<'a>(&self, post: &'a Post) -> &'a bool {
        &true
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<State> {
        Box::new(PendingSecondReview {})
    }
    fn reject(self: Box<Self>) -> Box<State> {
        Box::new(Draft {})
    }
}

struct PendingSecondReview {}

impl State for PendingSecondReview {
    fn request_review(self: Box<Self>) -> Box<State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<State> {
        Box::new(Published {})
    }
    fn reject(self: Box<Self>) -> Box<State> {
        Box::new(Draft {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<State> {
        // invalid action
        self
    }
    fn approve(self: Box<Self>) -> Box<State> {
        // invalid action
        self
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
    fn reject(self: Box<Self>) -> Box<State> {
        // invalid action
        self
    }
}
