use surf::Client;

pub struct WatermarksService {
  client: Client,
}

impl WatermarksService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }
}
