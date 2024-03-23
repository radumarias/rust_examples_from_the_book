pub struct Post {
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

struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> PreApprovedPost {
        PreApprovedPost {
            content: self.content,
        }
    }

    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }
}

pub struct PreApprovedPost {
    content: String,
}

impl PreApprovedPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }

    fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }
}

pub struct DraftPost {
    content: String,
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

fn main() {
    let mut post = Post::new();
    post.add_text("test");

    let post = post.request_review();
    let mut post = post.reject();
    post.add_text("test");

    let post = post.request_review();
    let post = post.approve();
    let post = post.approve();
    println!("{}", post.content());
}