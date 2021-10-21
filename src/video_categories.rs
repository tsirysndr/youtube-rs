use surf::Client;

pub struct VideoCategoriesService {
  pub client: Client,
}

impl VideoCategoriesService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }

  pub fn list(&self) {}
}
