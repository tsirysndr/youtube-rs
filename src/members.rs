use surf::Client;

pub struct MembersService {
  client: Client,
}

impl MembersService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }
}
