extern crate reqwest;
extern crate wasm_bindgen;

use std::io::Read;
use wasm_bindgen::prelude::*;

mod utils;

#[wasm_bindgen]
pub fn domani(url: &str) -> [f32; 3] {
  let mut res: reqwest::Response = reqwest::get(url).expect("Fail to send request");
  let mut buffer = Vec::new();
  res.read_to_end(&mut buffer).expect("Fail read to end");
  let img = image::load_from_memory(&buffer);
  let img = match img {
    Ok(i) => i,
    Err(e) => panic!(e),
  };

  let dominant_color = utils::get_dominant_color(&mut img.to_rgb());
  dominant_color
}
