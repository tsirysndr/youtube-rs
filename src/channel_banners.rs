use surf::Client;

pub struct ChannelBannersService {
  client: Client,
}

impl ChannelBannersService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }

  pub fn insert(&self, channel_id: &str, banner: &str) {}
}
