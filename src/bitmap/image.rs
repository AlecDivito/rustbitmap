// use std::error::Error;
use super::file::File;
use super::rgba::Rgba;

pub struct BitMap {
    /// file read from
    filename: Option<String>,
    /// width of image
    width: u32,
    /// height of image
    height: u32,
    /// each pixel as an rgba
    pixels: Vec<Rgba>,
}

impl BitMap {
    ///
    /// Create a bitmap by reading in a .bmp file
    ///
    pub fn read(filename: &str) -> Option<BitMap> {
        let file = File::read(filename).unwrap();
        Some(BitMap {
            filename: Some(String::from(filename)),
            width: file.get_info_header().get_width(),
            height: file.get_info_header().get_height(),
            pixels: file.get_pixels().as_rgba(),
        })
    }

    ///
    /// Create a new bitmap image in memory
    ///
    /// Fill all the pixels as white
    ///
    pub fn new(width: u32, height: u32) -> BitMap {
        let white = Rgba::white();
        BitMap {
            filename: None,
            width,
            height,
            pixels: vec![white; (width * height) as usize],
        }
    }

    ///
    /// Create a new image from a list of pixels
    ///
    pub fn create(width: u32, height: u32, pixels: Vec<Rgba>) -> BitMap // Result<BitMap, &'static str>
    {
        // if width * height != pixels.len()
        // {
        //     return Err("The area must match the ")
        // }
        BitMap {
            filename: None,
            width,
            height,
            pixels,
        }
    }

    ///
    /// Save the image to its original location
    ///
    /// Fail if no original location is linked to the current bitmap
    ///
    pub fn save(&self) -> Result<(), &'static str> {
        match self.filename.as_ref() {
            Some(f) => match self.save_as(f) {
                Ok(_) => Ok(()),
                Err(_) => Err("Error saving file to disk."),
            },
            None => Err("Couldn't save image because you didn't create the bitmap from an image"),
        }
    }

    ///
    /// Save the image to a new location on disk
    ///
    pub fn save_as(&self, filename: &str) -> std::io::Result<()> {
        let file = File::create(self);
        use std::io::Write;
        let mut bit_stream = unsafe { file.to_bytes() };
        let mut file = std::fs::File::create(filename)?;
        file.write_all(bit_stream.as_mut_slice())?;
        Ok(())
    }

    /**
     * Analyze the currently recorded pixels and try and find the lowest bit
     * count possible to save the images at.
     *
     * The bit depth will be:
     * if there are at most 2 colors present, 2 bit
     * if there are at most 16 colors present, 4 bit
     * if there are at most 256 colors present, 8 bit
     * if there are more then 256 colors and all alphas are 100, 24 bit
     * if there are more then 256 colors and at least one alpha is not 100, 32 bit
     *
     */
    // pub fn simplify_and_save() -> std::io::Result<()>
    // {

    // }

    // pub fn simplify_and_save_as() -> std::io::Result<()>
    // {

    // }

    ///
    /// Get a reference to the collection of all the pixels inside of the image
    ///
    pub fn get_pixels(&self) -> &Vec<Rgba> {
        &self.pixels
    }

    ///
    /// Get the width of the image
    ///
    pub fn get_width(&self) -> u32 {
        self.width
    }

    ///
    /// Get the height of the image
    ///
    pub fn get_height(&self) -> u32 {
        self.height
    }

    ///
    /// Get the area of the image in pixels
    ///
    pub fn get_size(&self) -> u32 {
        self.width * self.height
    }

    ///
    /// Get a reference to the file name of the bitmap if it exists
    ///
    pub fn get_filename(&self) -> Option<&String> {
        match self.filename.as_ref() {
            Some(s) => Some(s),
            None => None,
        }
    }

    ///
    /// The data saved in pixels is upside down because of the way its saved
    /// in the data file. Use this method to get the index for a given x and
    /// y position.
    ///
    fn get_index(&self, x: u32, y: u32) -> usize {
        (((self.height - y - 1) * self.width) + x) as usize
    }
}

///
/// This block deals with creating a new bitmap from an existing one and manipulating
/// bit maps using other bitmaps
///
impl BitMap {
    ///
    /// Crop a given area of the current image
    ///
    /// @param {u32} starting x position
    /// @param {u32} starting y position
    /// @param {u32} ending x position
    /// @param {u32} ending y position
    ///
    /// @exception {&'static str} error message
    /// if the starting x and ending x or starting y and ending y is out of the
    /// image with height or width, throw an error
    ///
    pub fn crop(
        &self,
        from_x: u32,
        from_y: u32,
        to_x: u32,
        to_y: u32,
    ) -> Result<BitMap, &'static str> {
        if from_x > to_x {
            return Err("From x must be less then to x.");
        }
        let width = to_x - from_x;

        if from_y > to_y {
            return Err("From y must be less then to y.");
        }
        let height = to_y - from_y;

        let area = width * height;
        if area == 0 {
            return Ok(BitMap::new(0, 0));
        }

        if to_x > self.width || to_y > self.height {
            return Err("cropped image exceeds the bounds of the current image.");
        }

        let mut colors = vec![Rgba::white(); area as usize];
        for y in from_y..to_y {
            for x in from_x..to_x {
                let index = self.get_index(x, y);
                let colors_index = (((height - y - from_y - 1) * width) + x - from_x) as usize;
                colors[colors_index] = self.pixels[index];
            }
        }

        // colors.reverse();
        Ok(BitMap::create(width, height, colors))
    }

    ///
    /// Paste and entire bitmap into the current bitmap.
    ///
    pub fn paste(
        &mut self,
        bitmap: &BitMap,
        start_at_x: u32,
        start_at_y: u32,
    ) -> Result<(), &'static str> {
        if start_at_x > self.width || start_at_y > self.height {
            return Err("Starting position is outside of the image.");
        }

        let end_at_x = start_at_x + bitmap.get_width();
        let end_at_y = start_at_y + bitmap.get_height();
        if end_at_x > self.width || end_at_y > self.height {
            return Err("Bitmap being pasted doesn't fit inside image.");
        }

        for x in start_at_x..end_at_x {
            for y in start_at_y..end_at_y {
                let bitmap_index = bitmap.get_index(x - start_at_x, y - start_at_y);
                let self_index = self.get_index(x, y);
                self.pixels[self_index] = bitmap.get_pixels()[bitmap_index];
            }
        }

        Ok(())
    }
}

///
/// This implementation block deals with coloring the image
///
impl BitMap {
    ///
    /// Set the color of a pixel
    ///
    /// @param {u32} x position
    /// @param {u32} y position
    /// @param {Rgba} color to set pixel
    ///
    pub fn set_pixel(&mut self, x: u32, y: u32, color: Rgba) -> Result<(), &'static str> {
        if y > self.height || x > self.width {
            return Err("Pixel is not contained inside of the image.");
        }
        // images are saved upside down, so to get the pixel we flip it right side up
        let index = self.get_index(x, y);
        if index > (self.pixels.len() - 1) {
            return Err("Pixel is not contained inside of the image.");
        }

        self.pixels[index as usize] = color;
        Ok(())
    }

    ///
    /// Convert image from a colored image to gray
    ///
    pub fn color_to_gray(&mut self) {
        for c in &mut self.pixels {
            c.color_to_gray();
        }
    }

    ///
    /// Find all the pixels that are the same as the from color and convert them
    /// all to the "to" color.
    ///
    /// @param {Rgba} from color
    /// @param {Rgba} to color
    ///
    pub fn replace_all_color(&mut self, from: Rgba, to: Rgba) {
        for c in &mut self.pixels {
            if c == &from {
                c.recolor_to(&to);
            }
        }
    }

    ///
    /// Fill a region of an image with a color. The only colors that get changed
    /// are those that are the same as the pixel found at the given x and y value
    ///
    /// @param {u32} x position
    /// @param {u32} y position
    /// @param {Rgba} color to use to replace the other color
    ///
    pub fn fill_region(&mut self, x: u32, y: u32, color: Rgba) -> Result<(), &'static str> {
        if y > self.height || x > self.width {
            return Err("Pixel is not contained inside of the image.");
        }
        // images are saved upside down, so to get the pixel we flip it right side up
        let starting_index = self.get_index(x, y);
        if starting_index > (self.pixels.len() - 1) {
            return Err("Pixel is not contained inside of the image.");
        }

        let width = self.width as usize;
        let old_color = self.pixels[starting_index];
        let mut visited = Vec::new();
        let mut unvisited = vec![starting_index];
        while unvisited.len() > 0 {
            let index = unvisited.pop().unwrap();
            if old_color == self.pixels[index] {
                self.pixels[index] = color;
                visited.push(index);
            } else {
                continue;
            }

            // check the pixel above
            if index > width && !visited.contains(&(index - width)) {
                unvisited.push(index - width);
            }
            // check the bottom pixel
            let bottom_pixel = index + width;
            if bottom_pixel < self.pixels.len() && !visited.contains(&bottom_pixel) {
                unvisited.push(bottom_pixel);
            }

            let index_in_image = index < self.pixels.len() && index > 0;
            // check the pixel to the right
            let right_pixel = index + 1;
            if index_in_image && index - 1 % width != 0 && !visited.contains(&right_pixel) {
                unvisited.push(right_pixel);
            }
            // check the pixel to the left
            if index_in_image && index % width != 0 && !visited.contains(&(index - 1)) {
                unvisited.push(index - 1);
            }
        }

        Ok(())
    }
}

///
/// This implementation block is only meant for resizing images using one of the
/// 3 (so far only 2 implemented) algorithms (nearest neighbor, bilinear, bicubic)
///
impl BitMap {
    ///
    /// Resize the current image by using nearest neighbor algorithm. Scale image
    /// to image size * the factor
    ///
    /// @param {f32} scaling factor to apply to image
    ///
    pub fn fast_resize_by(&mut self, factor: f32) {
        let width = (factor * (self.width as f32)).round();
        let height = (factor * (self.height as f32)).round();
        self.fast_resize(width as u32, height as u32);
    }

    ///
    /// Resize the current image by using nearest neighbor algorithm. Scale image
    /// to specified width and height
    ///
    /// @param {u32} new image width
    /// @param {u32} new image height
    ///
    pub fn fast_resize_to(&mut self, width: u32, height: u32) {
        self.fast_resize(width, height);
    }

    ///
    /// Resize the image by using a nearest neighbor algorithm
    ///
    /// @param {u32} new image width
    /// @param {u32} new image height
    ///
    fn fast_resize(&mut self, width: u32, height: u32) {
        // image 1 (currently loaded image)
        // image 2 (new image to be produced)
        let new_area = width * height;
        let mut i2: Vec<Rgba> = vec![Rgba::black(); new_area as usize];

        let cy = (height as f32) / (self.height as f32); // Scale in y
        let cx = (width as f32) / (self.width as f32); // Scale in x

        // write new image
        for y in 0..height {
            for x in 0..width {
                // Calculate position in input image
                // then just pick the nearest neighbor to (v, w)
                let v = ((x as f32) / cx).floor() as u32; // x, w
                let w = ((y as f32) / cy).floor() as u32; // y, h
                let i1_index = ((w * self.width) + v) as usize;
                let i2_index = ((y * width) + x) as usize;
                i2[i2_index] = self.pixels[i1_index];
            }
        }

        self.width = width;
        self.height = height;
        self.pixels = i2;
    }

    ///
    /// Resize the current image by using bilinear interpolation algorithm. Scale
    /// image to image size * the factor
    ///
    /// @param {f32} scaling factor to apply to image
    ///
    pub fn resize_by(&mut self, factor: f32) {
        let width = (factor * (self.width as f32)).round();
        let height = (factor * (self.height as f32)).round();
        self.resize(width as u32, height as u32);
    }

    ///
    /// Resize the current image by using bilinear interpolation algorithm. Scale
    /// image to specified width and height
    ///
    /// @param {u32} new image width
    /// @param {u32} new image height
    ///
    pub fn resize_to(&mut self, width: u32, height: u32) {
        self.resize(width, height);
    }

    ///
    /// Resize the image by using a Bilinear interpolation algorithm
    ///
    /// @param {u32} new image width
    /// @param {u32} new image height
    ///
    fn resize(&mut self, width: u32, height: u32) {
        // image 1 (currently loaded image)
        // image 2 (new image to be produced)
        let new_area = width * height;
        let mut i2: Vec<Rgba> = vec![Rgba::black(); new_area as usize];

        let step_x = std::cmp::max(self.width - 1, 1) as f32 / std::cmp::max(width - 1, 1) as f32;
        let step_y = std::cmp::max(self.height - 1, 1) as f32 / std::cmp::max(height - 1, 1) as f32;

        // write new image
        for y in 0..height {
            for x in 0..width {
                // Calculate position in input image
                // then just pick the nearest neighbor to (v, w)
                let v = (x as f32) * step_x; // x of our next point
                let w = (y as f32) * step_y; // y of our next point
                let diff_x = v - v.floor();
                let diff_x1 = 1.0 - diff_x;
                let diff_y = w - w.floor();
                let diff_y1 = 1.0 - diff_y;

                let index = ((y * width) + x) as usize;

                let index_1 = ((w.floor() * self.width as f32) + v.floor()) as usize;
                let mut index_2 = index_1 + 1;
                let mut index_3 = index_1 + self.width as usize;
                let mut index_4 = index_3 + 1;
                if index_2 >= self.get_size() as usize {
                    index_2 = index_1;
                }
                if index_3 >= self.get_size() as usize {
                    index_3 = index_1;
                }
                if index_4 >= self.get_size() as usize {
                    index_4 = index_2;
                }

                let top = Rgba::blur(
                    &self.pixels[index_2],
                    diff_x,
                    &self.pixels[index_1],
                    diff_x1,
                )
                .unwrap();
                let bottom = Rgba::blur(
                    &self.pixels[index_4],
                    diff_x,
                    &self.pixels[index_3],
                    diff_x1,
                )
                .unwrap();
                let color = Rgba::blur(&bottom, diff_y, &top, diff_y1).unwrap();
                i2[index] = color;
            }
        }

        self.width = width;
        self.height = height;
        self.pixels = i2;
    }
}

impl std::cmp::PartialEq for BitMap {
    fn eq(&self, other: &Self) -> bool {
        if self.pixels.len() != other.pixels.len()
            || self.width != other.width
            || self.height != other.height
        {
            return false;
        }

        for i in 0..self.pixels.len() {
            if self.pixels[i] != other.pixels[i] {
                return false;
            }
        }

        true
    }
}

impl std::fmt::Display for BitMap {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.filename.is_some() {
            write!(f, "filename: {} ", self.filename.as_ref().unwrap()).unwrap()
        }
        write!(
            f,
            "width: {}\t height: {}\t pixels: {}\n",
            self.width,
            self.height,
            self.pixels.len()
        )
        .unwrap();
        for c in &self.pixels {
            write!(f, "{}\n", c).unwrap();
        }
        write!(f, "\n")
    }
}
