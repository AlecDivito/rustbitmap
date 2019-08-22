extern crate rustbitmap;

use rustbitmap::{BitMap, Rgba};

fn main() {
    // test 1
    // let pixels = vec![Rgba::white(), Rgba::black(), Rgba::white()];
    // let bitmap = BitMap::create(3, 1, pixels).unwrap();
    // bitmap.save_as("default.bmp").unwrap();

    // // test 2
    // let pixels = vec![Rgba::white(), Rgba::black(), Rgba::white()];
    // let mut bitmap = BitMap::create(3, 1, pixels).unwrap();
    // bitmap.fast_resize_to(16, 1);
    // bitmap.save_as("nearest_neighbor.bmp").unwrap();

    // // test 3
    // let pixels = vec![Rgba::white(), Rgba::black(), Rgba::white()];
    // let mut bitmap = BitMap::create(3, 1, pixels).unwrap();
    // bitmap.resize_to(16, 1);
    // bitmap.save_as("bi_linear.bmp").unwrap();

    // test 4
    let pixels = vec![Rgba::white(), Rgba::black(), Rgba::white()];
    let mut bitmap = BitMap::create(3, 1, pixels).unwrap();
    bitmap.slow_resize_to(16, 1);
    bitmap.save_as("bi_cubic.bmp").unwrap();

}
