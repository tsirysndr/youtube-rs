use surf::Client;

pub struct CommentThreadsService {
  client: Client,
}

impl CommentThreadsService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }

  pub fn list(&self) {}
  
  pub fn insert(&self) {}
}
