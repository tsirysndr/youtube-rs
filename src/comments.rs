use surf::Client;

pub struct CommentsService {
  client: Client,
}

impl CommentsService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }

  pub fn list(&self) {}

  pub fn insert(&self) {}

  pub fn update(&self) {}

  pub fn mark_as_spam(&self) {}

  pub fn set_moderation_status(&self) {}

  pub fn delete(&self, id: &str) {}
}
