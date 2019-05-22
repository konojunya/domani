extern crate reqwest;
use std::io::Read;
mod utils;

fn main() {
  let mut res: reqwest::Response =
    reqwest::get("https://images-na.ssl-images-amazon.com/images/I/51GHTXFiklL.jpg")
      .expect("Fail to request");
  let mut buffer = Vec::new();
  res.read_to_end(&mut buffer).expect("Fail: read to end");
  let img = image::load_from_memory(&buffer);
  let img = match img {
    Ok(i) => i,
    Err(e) => panic!(e),
  };

  let dominant_color = utils::get_dominant_color(&mut img.to_rgb());
  println!(
    "rgb: ({},{},{})",
    dominant_color[0], dominant_color[1], dominant_color[2]
  );
}
