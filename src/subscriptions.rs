use surf::Client;

pub struct SubscriptionsService {
  client: Client,
}

impl SubscriptionsService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }

  pub fn list(&self) {}

  pub fn insert(&self) {}

  pub fn delete(&self) {}
}
