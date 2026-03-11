pub mod object_style {
    pub struct Post {
        state: Option<Box<dyn State>>,
        content: String,
    }

    impl Post {
        pub fn new() -> Self {
            Self {
                state: Some(Box::new(Draft {})),
                content: String::new(),
            }
        }

        pub fn add_text(&mut self, text: &str) {
            self.content.push_str(text);
        }

        pub fn content(&self) -> &str {
            self.state.as_ref().unwrap().content(self)
        }

        pub fn request_review(&mut self) {
            if let Some(state) = self.state.take() {
                self.state = Some(state.request_review());
            }
        }

        pub fn approve(&mut self) {
            if let Some(state) = self.state.take() {
                self.state = Some(state.approve());
            }
        }
    }

    trait State {
        fn request_review(self: Box<Self>) -> Box<dyn State>;
        fn approve(self: Box<Self>) -> Box<dyn State>;

        fn content<'a>(&self, _post: &'a Post) -> &'a str {
            ""
        }
    }

    struct Draft {}
    struct PendingReview {}
    struct Published {}

    impl State for Draft {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            Box::new(PendingReview {})
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }
    }

    impl State for PendingReview {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            Box::new(Published {})
        }
    }

    impl State for Published {
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
}

pub mod typed_style {
    pub struct Post {
        content: String,
    }

    pub struct DraftPost {
        content: String,
    }

    pub struct PendingReviewPost {
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

        pub fn request_review(self) -> PendingReviewPost {
            PendingReviewPost {
                content: self.content,
            }
        }
    }

    impl PendingReviewPost {
        pub fn approve(self) -> Post {
            Post {
                content: self.content,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{object_style, typed_style};

    #[test]
    fn object_style_post_only_shows_content_after_approval() {
        let mut post = object_style::Post::new();
        post.add_text("I ate a salad for lunch today");
        assert_eq!(post.content(), "");

        post.request_review();
        assert_eq!(post.content(), "");

        post.approve();
        assert_eq!(post.content(), "I ate a salad for lunch today");
    }

    #[test]
    fn typed_style_workflow_reaches_published_post() {
        let mut draft = typed_style::Post::new();
        draft.add_text("Rust makes invalid states harder to represent");

        let pending = draft.request_review();
        let published = pending.approve();

        assert_eq!(
            published.content(),
            "Rust makes invalid states harder to represent"
        );
    }
}
