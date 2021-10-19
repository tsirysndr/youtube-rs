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
}
