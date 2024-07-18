# rustbitmap

[![Build Status](https://travis-ci.org/AlecDivito/rustbitmap.svg?branch=master)](https://travis-ci.org/AlecDivito/rustbitmap)
[![codecov](https://codecov.io/gh/AlecDivito/rustbitmap/branch/master/graph/badge.svg)](https://codecov.io/gh/AlecDivito/rustbitmap)
<br>
A rust library that can read, write and edit bitmap files.

# Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
rustbitmap = "0.2.0"
```

# Getting start

## Reading Files and Creating bitmaps

rust-bitmap makes it really easy to read in bitmaps and edit them. To load a
bitmap from a file just pass in the string to the file. It's also possible to
create plain white in memory bitmaps as well.

```rust
use rustbitmap::BitMap;
use rustbitmap::Rgba;

fn main() {
    // load bitmap from file
    let bitmap = BitMap::read("example.bmp").unwrap();

    // create a new bitmap that is 24 pixels by 24 pixels
    let bitmap = BitMap::new(24, 24);

    // create 2 by 2 bitmap that is colored all black
    let pixels: Vec<Rgba> = vec![Rgba::black(), Rgba::white(), Rgba::white(), Rgba::black()];
    let bitmap = BitMap::create(2, 2, pixels).unwrap();
}
```

## Saving

To save a bitmap is very easy as well. There are 2 options to saving a bit map
which is to try and save a simplified version or to save a default 24 bit color
version.

```rust
use rustbitmap::BitMap;

fn main() {
   let bitmap = BitMap::new(24, 24);

   // the bitmap will be saved as a 24 bit image
   bitmap.save_as("24_bit_white_square.bmp").unwrap();
   
   // because the image is just a white square, it will be saved as a 1 bit image
   bitmap.simplify_and_save_as("1_bit_white_square.bmp").unwrap();
}
```

## Resizing

You can also edit the size of our bitmaps really quickly and easily by using
a range of different resizing tools such as nearest neighbor, bilinear interpolation
and bicubic interpolation. Using the different resizing tools we can create really
cool gradients.

```rust
use rustbitmap::{ BitMap, Rgba };

fn main() {
   let red = Rgba::rgb(255, 0, 0);
   let blue = Rgba::rgb(0, 0, 255);
   let green = Rgba::rgb(0, 255, 0);
   let white = Rgba::rgb(255, 255, 255);
   let pixels = vec![red, blue, green, white];
   let mut bitmap = Bitmap::create(2, 2, pixels).unwrap();
   bitmap.resize_by(100.0);
   bitmap.save_as("gradient.bmp").unwrap();
}
```

## Editing

If you want to crop an image or paste one bitmap into another it's really easy:

```rust
fn main() {
   let mut bitmap = BitMap::new(24, 24).unwrap();
   let cropped = bitmap.crop(0, 0, 10, 10).unwrap();

   // cropped is not a new bitmap image that is 10 by 10 of the original bitmap
   // image starting at (0, 0)

   // let's recolor our original image using `fill`. Fill works just like the
   // paint bucket tool in most drawing applications.
   bitmap.fill_region(5, 5, Rgba::black()).unwrap();

   // now the entire original image is black, let's paste back in our cropped
   // image in a new position
   bitmap.paste
}
```

## License

Licensed under [MIT license](LICENSE) or http://opensource.org/licenses/MIT

## Contribution

If you come by this project and want to contribute just post an issue, explaining
what feature you would like to add or bug you ran into.
