use rustbitmap::{BitMap, Rgba};

fn main() {
    let red = Rgba::rgb(255, 0, 0);
    let blue = Rgba::rgb(0, 0, 255);
    let green = Rgba::rgb(0, 255, 0);
    let white = Rgba::rgb(255, 255, 255);
    let pixels = vec![red, blue, green, white];
    let bitmap = BitMap::create(2, 2, pixels).unwrap();

    let mut slow = bitmap.clone();
    slow.slow_resize_to(300, 300); // bicubic
    
    let mut medium = bitmap.clone();
    medium.resize_to(300, 300); // bilinear
    
    let mut fast = bitmap.clone();
    fast.fast_resize_to(300, 300); // nearest neighbor

    let mut bitmap = BitMap::new(900, 300);
    bitmap.paste(&slow, 600, 0).unwrap();
    bitmap.paste(&medium, 300, 0).unwrap();
    bitmap.paste(&fast, 0, 0).unwrap();
    bitmap.simplify_and_save_as("images/all.bmp").unwrap();
}
