use surf::Client;

pub struct CaptionsService {
  client: Client,
}

impl CaptionsService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }
}
