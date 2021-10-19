use surf::Client;

pub struct VideosService {
  client: Client,
}

impl VideosService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }
}
