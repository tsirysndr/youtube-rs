use surf::Client;

pub struct PlaylistsService {
  client: Client,
}

impl PlaylistsService {
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
