extern crate rustbitmap;

use rustbitmap::{BitMap, Rgba};

fn main() {
    // test 1
    // let pixels = vec![Rgba::black(),Rgba::white(), Rgba::black(), Rgba::rgb(255,0,0)];
    // let bitmap = BitMap::create(2, 2, pixels).unwrap();
    // bitmap.save_as("~/temp.bmp").unwrap();

    // // test 2
    // let pixels = vec![Rgba::black(),Rgba::white(), Rgba::black(), Rgba::rgb(255,0,0)];
    // let mut bitmap = BitMap::create(2, 2, pixels).unwrap();
    // bitmap.fast_resize_to(796, 796);
    // bitmap.save_as("~/temp.bmp").unwrap();

    // // test 3
    // let pixels = vec![Rgba::rgb(0,255,0),Rgba::white(), Rgba::rgb(0,0,255), Rgba::rgb(255,0,0)];
    // let mut bitmap = BitMap::create(2, 2, pixels).unwrap();
    let mut bitmap = BitMap::read("./out.bmp").unwrap();
    bitmap.resize_by(0.5).unwrap();
    bitmap.save_as("./bi_linear.bmp").unwrap();

    // test 4
    // let pixels = vec![Rgba::rgb(0,255,0),Rgba::white(), Rgba::rgb(0,0,255), Rgba::rgb(255,0,0)];
    // let mut bitmap = BitMap::create(2, 2, pixels).unwrap();
    let mut bitmap = BitMap::read("./out.bmp").unwrap();
    bitmap.slow_resize_by(0.5).unwrap();
    bitmap.save_as("./blah.bmp").unwrap();
}
