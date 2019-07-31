mod bitmap;

fn main() {
    // let mut b1 = bitmap::file::File::read("/home/divitoa/Program/RUST/bmp-writer/test/w3c_home_2.bmp").unwrap();
    // println!("{}", b1);
    // let b3 = bitmap::file::File::read("/home/divitoa/Program/RUST/bmp-writer/save.bmp").unwrap();
    // println!("{}", b3);


    // b1.save("/home/divitoa/Program/RUST/bmp-writer/save.bmp", bitmap::bit_count::BitCount::BW);



    // let b2 = Bmp::read("/home/divitoa/Program/RUST/bmp-writer/test/w3c_home_gray.bmp");
    // println!("{}", b2);
    let mut b4 = bitmap::file::File::read("/home/divitoa/Program/RUST/bmp-writer/example.bmp").unwrap();
    // let mut b4 = bitmap::file::File::read("/home/divitoa/Program/RUST/bmp-writer/sm_ex.bmp").unwrap();
    // let mut b4 = bitmap::file::File::read("/home/divitoa/Program/RUST/bmp-writer/test/w3c_home.bmp").unwrap();
    println!("{}", b4);
    b4.save("/home/divitoa/Program/RUST/bmp-writer/save.bmp", bitmap::bit_count::BitCount::BW).unwrap();


    let b2 = bitmap::file::File::read("/home/divitoa/Program/RUST/bmp-writer/save.bmp").unwrap();
    println!("{}", b2);



    // part 1, read and write files

    // BitMap::File
    // The read function should be able to read in any bitmap with any type of
    // bit count, HOWEVER, it should store everything as a 32 Bit Count bit map
    // when saving,
    // let bitmap = BitMap::File::read("/path/to/bitmap.bmp").unwrap()

    // - simplify flag will covert the bit_depth to its lowest possible value
    // bitmap.save(simplify: true | false);     // save the currently read in file
    // - save the current read in file to new bitmap
    // bitmap.save_as("path/to/new/bitmap.bmp", simplify: true | false)

    // part 2, resize files

    // resize will resize the image by a percentage
    // bitmap.resize_to(x: u32, y: u32)
    // bitmap.resize_percentage(2.0)

    // part 3, build a bit map

    // let bits = BitMap::default(width: u32, height: u32, BitMap::Rgba = white)
    // or
    // let bits = BitMap::create(width: u32, height: u32, &[u8])

    // bits.set_stream(&[u8])
    // bits.set_stream_sub_group(&[u8], start_at_x: u32, end_at_x: u32, start_at_y: u32, end_at_y: u32)
    // bits.set_pixel(x: u32, y: u32, BitMap::Rgba)
    // bits.set_pixel_by_index(i: u32, BitMap::Rgba)
    // bits.copy_and_paste(x: u32, y: u32, &BitMap::File)
    // bits.copy_and_paste_sub_group(x: u32, y: u32, &BitMap::File, start_at_x: u32, end_at_x: u32, start_at_y: u32, end_at_y: u32)
    // let bits_cropped: BitMap::File = bits.crop(start_at_x: u32, end_at_x: u32, start_at_y: u32, end_at_y: u32)
    // bits.replace_color(replace: BitMap::Rgba, with: BitMap::Rgba)

    // part 4, I don't know how much I want to expose the BitCount

    // bits.blend_to(BitCount::BW)
}
