use rustbitmap::BitMap;

fn main() {
    // load bitmap from file
    let bitmap = BitMap::read("/mnt/c/Users/divit/Downloads/building.bmp").unwrap();
    let resize_factor = 0.3;

    let mut fast = bitmap.clone();
    println!("Pixel Count: {}", fast.get_pixels().len());
    fast.fast_resize_by(resize_factor).unwrap();
    println!("Fast Resize bytes: {}", fast.get_estimated_file_size_in_bytes());
    fast.simplify_and_save_as("boy-in-water-fast.bmp").unwrap();

    let mut medium = bitmap.clone();
    println!("Pixel Count: {}", medium.get_pixels().len());
    medium.resize_by(resize_factor).unwrap();
    println!("Medium Resize bytes: {}", medium.get_estimated_file_size_in_bytes());
    medium.simplify_and_save_as("boy-in-water-medium.bmp").unwrap();

    let mut slow = bitmap.clone();
    println!("Pixel Count: {}", slow.get_pixels().len());
    slow.slow_resize_by(resize_factor).unwrap();
    println!("Slow Resize bytes: {}", slow.get_estimated_file_size_in_bytes());
    slow.simplify_and_save_as("boy-in-water-slow.bmp").unwrap();
}
