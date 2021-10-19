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
}
