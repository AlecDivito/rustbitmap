extern crate rustbitmap;

use rustbitmap::BitMap;
use rustbitmap::Rgba;

fn main() {
    // load bitmap from file
    let _bitmap = BitMap::read("example.bmp").unwrap();

    // create a new bitmap that is 24 pixels by 24 pixels
    let _bitmap = BitMap::new(24, 24);

    // create 2 by 2 bitmap that is colored all black
    let pixels: Vec<Rgba> = vec![Rgba::black(), Rgba::white(), Rgba::white(), Rgba::black()];
    let _bitmap = BitMap::create(2, 2, pixels).unwrap();
}
