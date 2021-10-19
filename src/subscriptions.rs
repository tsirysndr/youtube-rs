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
}
