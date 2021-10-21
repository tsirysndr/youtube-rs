use surf::Client;

pub struct MembershipsLevelsService {
  client: Client,
}

impl MembershipsLevelsService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }

  pub fn list(&self) {}
}
