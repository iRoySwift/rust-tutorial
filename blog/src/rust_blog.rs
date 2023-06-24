// 状态模式的取舍权衡
// 缺点：代码耦合
// 重复实现某些代码

// 将状态和行为编码为类型
// 使用rust 实现blog
// 将编码为不同行为
// Rust类型检查系统会通过编译时来阻止用户使用无效的状态
pub fn run() {
    let mut post = Post::new();

    post.add_text("i test");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("i test", post.content());
}

pub struct Post {
    content: String,
}
pub struct DraftPost {
    content: String,
}
pub struct PendingReview {
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
    pub fn request_review(self) -> PendingReview {
        PendingReview {
            content: self.content,
        }
    }
}
impl PendingReview {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}
