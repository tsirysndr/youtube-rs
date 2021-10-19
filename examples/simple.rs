use youtube_rs::Youtube;

fn main() {
  let client = Youtube::new();
  client.search.list("doja cat");
}
