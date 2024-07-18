use rustbitmap::BitMap;

fn main() {
    let bitmap = BitMap::read("images/building.bmp").unwrap();
    let width = (bitmap.get_width() / 3) - 1;
    
    let mut img1 = bitmap.crop(0, 0, width, 100).unwrap();
    let mut img2 = bitmap.crop(width, 100, width * 2, 200).unwrap();
    let mut img3 = bitmap.crop(width * 2, 200, bitmap.get_width(), 300).unwrap();

    let (width, hight) = (300, 600);

    img1.fast_resize_to(width, hight);
    img2.resize_to(width, hight);
    img3.slow_resize_to(width, hight);

    let mut bitmap = BitMap::new(width*3, hight);
    bitmap.paste(&img1, 0, 0).unwrap();
    bitmap.paste(&img2, width, 0).unwrap();
    bitmap.paste(&img3, width * 2, 0).unwrap();
    bitmap.simplify_and_save_as("images/image.bmp").unwrap();
}
