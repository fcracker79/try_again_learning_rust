pub struct Post {
    state: Box<dyn State>,
    content: String,
}


// Box takes ownership of the state, so you cannot use it any more and you are forced
// to adopt the new state
trait State {
    fn add_content(&mut self, old_content: &mut String, content: &str) -> Box<dyn State>;
    fn request_review(&mut self) -> Box<dyn State>;
    fn get_content<'a>(&'a self, _post: &'a Post) -> &'a str {
        ""
    }
    fn approve(&mut self) -> Box<dyn State>;
}

struct Draft {}

impl State for Draft {
    fn add_content(&mut self, old_content: &mut String, content: &str) -> Box<dyn State> {
        old_content.push_str(content);
        Box::new(Draft {})
    }

    fn request_review(&mut self) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    fn approve(&mut self) -> Box<dyn State> {
        panic!("Invalid state")
    }
}

struct PendingReview {}


impl State for PendingReview {
    fn add_content(&mut self, _old_content: &mut String, _content: &str) -> Box<dyn State> {
        panic!("Invalid state");
    }

    fn request_review(&mut self) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(&mut self) -> Box<dyn State> {
        Box::new(Approved {})
    }
}


struct Approved {}


impl State for Approved {
    fn add_content(&mut self, _old_content: &mut String, _content: &str) -> Box<dyn State> {
        panic!("Invalid state");
    }

    fn request_review(&mut self) -> Box<dyn State> {
        panic!("Invalid state");
    }

    fn get_content<'a>(&'a self, post: &'a Post) -> &'a str {
        &post.content
    }

    fn approve(&mut self) -> Box<dyn State> {
        Box::new(Approved {})
    }
}


impl Post {
    pub fn new() -> Self {
        Post {
            state: Box::new(Draft {}),
            content: String::new(),
        }
    }

    fn add_text(&mut self, s: &str) {
        self.state.add_content(&mut self.content,s);
    }

    pub fn request_review(&mut self) {
        self.state = self.state.request_review();
    }

    pub fn approve(&mut self) {
        self.state = self.state.approve();
    }

    pub fn content(&self) -> &str {
        &self.state.get_content(self)
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_states() {
        let mut post = Post::new();

        post.add_text("I ate a salad for lunch today");
        assert_eq!("", post.content());

        post.request_review();
        assert_eq!("", post.content());

        post.approve();
        assert_eq!("I ate a salad for lunch today", post.content());
    }
}