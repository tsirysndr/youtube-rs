use surf::Client;

pub struct CaptionsService {
  client: Client,
}

impl CaptionsService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }

  pub fn list(&self, video_id: &str) {}

  pub fn insert(&self, video_id: &str, language: &str, name: &str, is_draft: bool) {}

  pub fn update(&self, id: &str, video_id: &str, language: &str, is_draft: bool) {}

  pub fn download(&self, id: &str) {}

  pub fn delete(&self, id: &str) {}
}
