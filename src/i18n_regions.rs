use surf::Client;

pub struct I18nRegionsService {
  client: Client,
}

impl I18nRegionsService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }

  pub fn list(&self) {}
}
