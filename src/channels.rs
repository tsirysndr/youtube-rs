use surf::Client;

pub struct ChannelsService {
  client: Client,
}

impl ChannelsService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }
}
