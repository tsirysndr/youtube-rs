use surf::Client;

pub struct ThumbnailsService {
  client: Client,
}

impl ThumbnailsService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }
}
