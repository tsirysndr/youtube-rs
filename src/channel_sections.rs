use surf::Client;

pub struct ChannelSectionsService {
  client: Client,
}

impl ChannelSectionsService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }
  
  pub fn list(&self) {}

  pub fn insert(&self) {}

  pub fn update(&self) {}
}
