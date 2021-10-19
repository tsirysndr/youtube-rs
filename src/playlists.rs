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
}
