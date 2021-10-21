use surf::Client;

pub struct PlaylistItemsService {
  client: Client,
}

impl PlaylistItemsService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }

  pub fn list(&self) {}

  pub fn insert(&self) {}

  pub fn update(&self) {}

  pub fn delete(&self) {}
}
