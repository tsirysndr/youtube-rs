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
}
