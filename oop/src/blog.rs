// 实现面向对象的设计模式
// 状态模式
pub fn run() {
    let mut post = Post::new();
    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    println!("{:#?}", post.content());
    assert_eq!("I ate a salad for lunch today", post.content());
}

// #[derive(Debug)]
pub struct Post {
    content: String,
    state: Option<Box<dyn State>>,
}

impl Post {
    fn new() -> Self {
        Post {
            content: String::new(),
            state: Some(Box::new(Draft {})),
        }
    }
    fn add_text(&mut self, text: &str) {
        self.content.push_str(text)
    }
    fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(&self)
    }
    fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }
    fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve());
        }
    }
}

trait State {
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
}

struct Draft {}
impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}
impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Publishd {})
    }
}

struct Publishd {}
impl State for Publishd {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
