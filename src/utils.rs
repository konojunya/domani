extern crate image;
use image::RgbImage;

#[no_mangle]
pub fn get_dominant_color(image: &mut RgbImage) -> [f32; 3] {
  let mut rgb = [0_f32, 0_f32, 0_f32];
  let w = image.width();
  let h = image.height();
  let count = (w * h) as f32;

  image.enumerate_pixels_mut().for_each(|(_x, _y, pixel)| {
    rgb[0] = rgb[0] + pixel[0] as f32;
    rgb[1] = rgb[1] + pixel[1] as f32;
    rgb[2] = rgb[2] + pixel[2] as f32;
  });

  let red: f32 = rgb[0] / count;
  let green: f32 = rgb[1] / count;
  let blue: f32 = rgb[2] / count;

  return [red.ceil(), green.ceil(), blue.ceil()];
}
