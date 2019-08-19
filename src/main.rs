extern crate rustybitmap;

use rustybitmap::bitmap::image::BitMap;
use rustybitmap::bitmap::rgba::Rgba;

fn main() {
    // part 1, read and write files

    // BitMap::File
    // The read function should be able to read in any bitmap with any type of
    // bit count, HOWEVER, it should store everything as a 32 Bit Count bit map
    // when saving,
    // let mut bitmap1 = BitMap::read("./a/w3c_home.bmp").unwrap();
    // let mut bitmap2 = BitMap::read("./a/w3c_home_2.bmp").unwrap();
    // let mut bitmap3 = BitMap::read("./a/w3c_home_gray.bmp").unwrap();
    // let mut bitmap4 = BitMap::read("./a/w3c_home_256.bmp").unwrap();

    // bitmap1.save().unwrap();
    // bitmap2.save().unwrap();
    // bitmap3.save().unwrap();
    // bitmap4.save().unwrap();

    // // resize will resize the image by a percentage
    // bitmap1.resize_by(4.0);
    // bitmap2.resize_by(0.25);
    // bitmap3.resize_to(50, 48);
    // bitmap4.resize_to(500, 50);

    // bitmap1.save_as("./a/w3cc_home.bmp").unwrap();
    // bitmap2.save_as("./a/w3cc_home_2.bmp").unwrap();
    // bitmap3.save_as("./a/w3cc_home_gray.bmp").unwrap();
    // bitmap4.save_as("./a/w3cc_home_256.bmp").unwrap();

    // let pixels = vec![Rgba::rgb(127, 127, 127), Rgba::white(), Rgba::black(), Rgba::rgb(255, 0, 0)];
    // let mut bitmap = BitMap::create(4, 1, pixels);
    // bitmap.save_as("new-0.bmp").unwrap();
    // bitmap.rotate_right();
    // bitmap.save_as("new-1.bmp").unwrap();
    // bitmap.rotate_right();
    // bitmap.save_as("new-2.bmp").unwrap();
    // bitmap.rotate_right();
    // bitmap.save_as("new-3.bmp").unwrap();
    // bitmap.rotate_right();
    // bitmap.save_as("new-4.bmp").unwrap();
    // bitmap.rotate_right();
    // bitmap.rotate_right();
    // bitmap.rotate_right();

    // bitmap.rotate_left();
    // bitmap.rotate_left();
    // bitmap.rotate_left();
    // bitmap.save_as("new-5.bmp").unwrap();

    let red = Rgba::rgb(255, 0, 0);
    let blue = Rgba::rgb(0, 0, 255);
    let green = Rgba::rgb(0, 255, 0);

    let mut b = BitMap::new(2, 2);
    b.set_pixel(0, 0, Rgba::white()).unwrap();
    b.set_pixel(1, 0, /*Rgba::black()*/ red).unwrap();
    b.set_pixel(0, 1, /*Rgba::black()*/ blue).unwrap();
    b.set_pixel(1, 1, /*Rgba::white()*/ green).unwrap();
    b.resize_by(25.0);
    // b.color_to_gray();
    b.simplify_and_save_as("hahahaha_stay_in_the_light.bmp")
        .unwrap();
    b.save_as("hahahaha_stay_in_the_light2.bmp").unwrap();

    // let mut bitmap = BitMap::read("./test.bmp").unwrap();
    // bitmap.rotate_right();
    // bitmap.save_as("./after-right-temp-1.bmp").unwrap();
    // bitmap.rotate_right();
    // bitmap.save_as("./after-right-temp-2.bmp").unwrap();
    // bitmap.rotate_right();
    // bitmap.save_as("./after-right-temp-3.bmp").unwrap();
    // bitmap.rotate_right();
    // bitmap.save_as("./after-right-temp-0.bmp").unwrap();
    // // bitmap.resize_to(100, 100);
    // // bitmap.resize_by(10.0);
    // let mut new_bits = bitmap.crop(0, 0, 200, 200).unwrap();
    // let bits = BitMap::new(50, 50);
    // new_bits.paste(&bits, 0, 0).unwrap();

    // bitmap.color_to_gray();
    // for x in 0..100 {
    //     for y in 0..100 {
    //         bitmap.set_pixel(x, y, Rgba::rgb(255, 255, 255)).unwrap();
    //     }
    // }
    // new_bits.replace_all_color(Rgba::white(), Rgba::black());
    // new_bits.fill_region(0, 0, Rgba::rgb(255, 0, 0)).unwrap();
    // new_bits.save_as("./temp1.bmp").unwrap();
    // bitmap
    //     .paste(&new_bits, bitmap.get_width() - new_bits.get_width(), 0)
    //     .unwrap();
    // bitmap.save_as("./before-temp.bmp").unwrap();

    // Possibilities to add
    // simplify_and_save()
    // simplify_and_save_as()
    // slow_resize_to() // bicubic
    // slow_resize_by() // bicubic
    // bits.rotate_right()
    // bits.rotate_right()
}
