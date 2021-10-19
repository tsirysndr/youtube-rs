use surf::Client;

pub struct I18nLanguagesService {
  client: Client,
}

impl I18nLanguagesService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }
}
