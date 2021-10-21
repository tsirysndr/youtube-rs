use surf::Client;

pub struct VideoAbuseReportReasonsService {
  pub client: Client,
}

impl VideoAbuseReportReasonsService {
  pub fn new(client: &Client) -> Self {
    Self {
      client: client.clone(),
    }
  }

  pub fn list(&self) {}
}
