extern crate rustbitmap;

use rustbitmap::{ BitMap, Rgba };

fn main() {
   let mut bitmap = BitMap::new(24, 24);
   let cropped = bitmap.crop(0, 0, 10, 10).unwrap();

   // cropped is not a new bitmap image that is 10 by 10 of the original bitmap
   // image starting at (0, 0)

   // let's recolor our original image using `fill`. Fill works just like the
   // paint bucket tool in most drawing applications.
   bitmap.fill_region(5, 5, Rgba::black()).unwrap();

   // now the entire original image is black, let's paste back in our cropped
   // image in a new position
   bitmap.paste(&cropped, 10, 10).unwrap();
}