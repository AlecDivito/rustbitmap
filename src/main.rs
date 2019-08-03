extern crate rustybitmap;

use rustybitmap::bitmap::map::BitMap;

fn main() {
    // part 1, read and write files

    // BitMap::File
    // The read function should be able to read in any bitmap with any type of
    // bit count, HOWEVER, it should store everything as a 32 Bit Count bit map
    // when saving,
    let bitmap1 = BitMap::read("./a/w3c_home.bmp").unwrap();
    let bitmap2 = BitMap::read("./a/w3c_home_2.bmp").unwrap();
    let bitmap3 = BitMap::read("./a/w3c_home_gray.bmp").unwrap();
    let bitmap4 = BitMap::read("./a/w3c_home_256.bmp").unwrap();

    bitmap1.save().unwrap();
    bitmap2.save().unwrap();
    bitmap3.save().unwrap();
    bitmap4.save().unwrap();

    bitmap1.save_as("./a/w3cc_home.bmp").unwrap();
    bitmap2.save_as("./a/w3cc_home_2.bmp").unwrap();
    bitmap3.save_as("./a/w3cc_home_gray.bmp").unwrap();
    bitmap4.save_as("./a/w3cc_home_256.bmp").unwrap();





    // - simplify flag will covert the bit_depth to its lowest possible value
    // bitmap.save(simplify: true | false);     // save the currently read in file
    // - save the current read in file to new bitmap
    // bitmap.save_as("path/to/new/bitmap.bmp", simplify: true | false)

    // part 2, resize files

    // resize will resize the image by a percentage
    // bitmap.resize_to(x: u32, y: u32)
    // bitmap.resize_percentage(2.0)

    // part 3, build a bit map

    // let bits = BitMap::new(width: u32, height: u32)
    // let bits = BitMap::new_colored(width: u32, height: u32, BitMap::Rgba)

    // bits.set_pixel(x: u32, y: u32, BitMap::Rgba)
    // bits.set_pixel_by_index(i: u32, BitMap::Rgba)
    // bits.copy_and_paste(x: u32, y: u32, &BitMap::File)
    // bits.copy_and_paste_sub_group(x: u32, y: u32, &BitMap::File, start_at_x: u32, end_at_x: u32, start_at_y: u32, end_at_y: u32)
    // let bits_cropped: BitMap::File = bits.crop(start_at_x: u32, end_at_x: u32, start_at_y: u32, end_at_y: u32)
    // bits.replace_color(replace: BitMap::Rgba, with: BitMap::Rgba)
    // bits.replace_color_in_region(replace: BitMap::Rgba, with: BitMap::Rgba, at: BitMap::Region)
    // bits.rotate_left()
    // bits.rotate_right()

    // pub struct Region {
    //     x: u32
    //     y: u32
    //     width: u32
    //     height: u32
    // }

    // part 4, I don't know how much I want to expose the BitDepth

    // bits.blend_to(BitDepth::BW)
}
