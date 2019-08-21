extern crate rustbitmap;

use rustbitmap::{BitMap, Rgba};

fn main() {
   let red = Rgba::rgb(255, 0, 0);
   let blue = Rgba::rgb(0, 0, 255);
   let green = Rgba::rgb(0, 255, 0);
   let white = Rgba::rgb(255, 255, 255);
   let pixels = vec![red, blue, green, white];
   let mut bitmap = BitMap::create(2, 2, pixels).unwrap();
   bitmap.resize_by(100.0);
   bitmap.save_as("gradient.bmp").unwrap();
}