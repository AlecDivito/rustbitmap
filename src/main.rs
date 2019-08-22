extern crate rustbitmap;

use rustbitmap::{BitMap, Rgba};

fn main() {
    // test 1
    let pixels = vec![Rgba::black(),Rgba::white(), Rgba::black(), Rgba::rgb(255,0,0)];
    let bitmap = BitMap::create(2, 2, pixels).unwrap();
    bitmap.save_as("default.bmp").unwrap();

    // test 2
    let pixels = vec![Rgba::black(),Rgba::white(), Rgba::black(), Rgba::rgb(255,0,0)];
    let mut bitmap = BitMap::create(2, 2, pixels).unwrap();
    bitmap.fast_resize_to(8,8);
    bitmap.save_as("nearest_neighbor.bmp").unwrap();

    // test 3
    let pixels = vec![Rgba::black(),Rgba::white(), Rgba::black(), Rgba::rgb(255,0,0)];
    let mut bitmap = BitMap::create(2, 2, pixels).unwrap();
    bitmap.resize_to(8,8);
    bitmap.save_as("bi_linear.bmp").unwrap();

    // test 4
    let pixels = vec![Rgba::black(),Rgba::white(), Rgba::black(), Rgba::rgb(255,0,0)];
    let mut bitmap = BitMap::create(2, 2, pixels).unwrap();
    bitmap.slow_resize_to(8,8);
    bitmap.save_as("bi_cubic.bmp").unwrap();

}
