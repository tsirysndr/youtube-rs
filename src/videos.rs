use surf::Client;

pub struct VideosService {
  client: Client,
}

impl VideosService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }

  pub fn list(&self) {}

  pub fn insert(&self) {}

  pub fn update(&self) {}

  pub fn rate(&self) {}

  pub fn get_rating(&self) {}

  pub fn report_abuse(&self) {}

  pub fn delete(&self) {}
}
