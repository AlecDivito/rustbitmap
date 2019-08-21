extern crate rustbitmap;

use rustbitmap::BitMap;

fn main() {
    let bitmap = BitMap::new(24, 24);

    // the bitmap will be saved as a 24 bit image
    bitmap.save_as("24_bit_white_square.bmp").unwrap();

    // because the image is just a white square, it will be saved as a 1 bit image
    bitmap
        .simplify_and_save_as("1_bit_white_square.bmp")
        .unwrap();
}
