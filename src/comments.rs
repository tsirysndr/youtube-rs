use surf::Client;

pub struct CommentsService {
  client: Client,
}

impl CommentsService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }
}
