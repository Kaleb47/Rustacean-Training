pub struct Post {
  content: String,
}

pub struct Draftpost {
  content: string,
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