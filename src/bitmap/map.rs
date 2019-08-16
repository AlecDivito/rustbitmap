// use std::error::Error;
use super::file::File;
use super::rgba::Rgba;

pub struct BitMap
{
    filename: String,
    width: u32,
    height: u32,
    pixels: Vec<Rgba>
}

impl BitMap
{
    pub fn read(filename: &str) -> Option<BitMap>
    {
        let file = File::read(filename).unwrap();
        Some(BitMap {
            filename: String::from(filename),
            width: file.get_info_header().get_width(),
            height: file.get_info_header().get_height(),
            pixels: file.get_pixels().as_rgba()
        })
    }

    pub fn new(width: u32, height: u32) -> BitMap
    {
        let white = Rgba::white();
        BitMap {
            filename: String::default(),
            width,
            height,
            pixels: vec![white; (width * height) as usize]
        }
    }

    pub fn create(width: u32, height: u32, pixels: Vec<Rgba>) -> BitMap
    {
        BitMap {
            filename: String::default(),
            width,
            height,
            pixels
        }
    }

    /**
     * Save the image to its original location
     * 
     * Fail if no original location is linked to the current bitmap
     */
    pub fn save(&self) -> std::io::Result<()>
    {
        self.save_as(&self.filename)
    }

    /**
     * Save the image to a new location on disk
     */
    pub fn save_as(&self, filename: &str) -> std::io::Result<()>
    {
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
     * The bitcount will be:
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

    pub fn get_pixels(&self) -> &Vec<Rgba>
    {
        &self.pixels
    }

    pub fn get_width(&self) -> u32
    {
        self.width
    }

    pub fn get_height(&self) -> u32
    {
        self.height
    }

    pub fn get_size(&self) -> u32
    {
        self.width * self.height
    }

    pub fn get_filename(&self) -> &String
    {
        &self.filename
    }

    fn get_index(&self, x: u32, y: u32) -> usize
    {
        (((self.height - y - 1) * self.width) + x) as usize
    }
}

/**
 * This block deals with creating a new bitmap from an existing one and manipulating
 * bit maps using other bitmaps
 */
impl BitMap {
    pub fn crop(&self, from_x: u32, from_y: u32, to_x: u32, to_y: u32) -> Result<BitMap, &'static str>
    {
        if from_x > to_x
        {
            return Err("From x must be less then to x.");
        }
        let width = to_x - from_x;

        if from_y > to_y
        {
            return Err("From y must be less then to y.");
        }
        let height = to_y - from_y;

        let area = width * height;
        if area == 0
        {
            return Ok(BitMap::new(0, 0));
        }

        if to_x > self.width || to_y > self.height
        {
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

    pub fn paste(&mut self, bitmap: &BitMap, start_at_x: u32, start_at_y: u32) -> Result<(), &'static str>
    {
        if start_at_x > self.width || start_at_y > self.height
        {
            return Err("Starting position is outside of the image.");
        }

        let end_at_x = start_at_x + bitmap.get_width();
        let end_at_y = start_at_y + bitmap.get_height(); 
        if end_at_x > self.width || end_at_y > self.height
        {
            return Err("Bitmap being pasted doesn't fit inside image.");
        }

        for x in start_at_x..end_at_x
        {
            for y in start_at_y..end_at_y
            {
                let bitmap_index = bitmap.get_index(x - start_at_x, y - start_at_y);
                let self_index = self.get_index(x, y);
                self.pixels[self_index] = bitmap.get_pixels()[bitmap_index];
            }
        }

        Ok(())
    }
}

/**
 * This implementation block deals with coloring the image
 */
impl BitMap {

    pub fn set_pixel(&mut self, x: u32, y: u32, color: Rgba) -> Result<(), &'static str>
    {
        if y > self.height || x > self.width
        {
            return Err("Pixel is not contained inside of the image.");
        }
        // images are saved upside down, so to get the pixel we flip it right side up
        let index = self.get_index(x, y);
        if index > (self.pixels.len() - 1)
        {
            return Err("Pixel is not contained inside of the image.");
        }

        self.pixels[index as usize] = color;
        Ok(())
    }

    /**
     * Convert image from a colored image to gray
     */
    pub fn color_to_gray(&mut self)
    {
        for c in &mut self.pixels
        {
            c.color_to_gray();
        }
    }
}

/**
 * This implementation block is only meant for resizing images using one of the 
 * 3 (so far only 2 implemented) algorithms (nearest neighbor, bilinear, bicubic)
 */
impl BitMap {

    /**
     * Resize the current image by using nearest neighbor algorithm. Scale image
     * to image size * the factor
     * 
     * @param {f32} scaling factor to apply to image
     */
    pub fn fast_resize_by(&mut self, factor: f32)
    {
        let width = (factor *  (self.width as f32)).round();
        let height = (factor * (self.height as f32)).round();
        self.fast_resize(width as u32, height as u32);
    }

    /**
     * Resize the current image by using nearest neighbor algorithm. Scale image
     * to specified width and height
     * 
     * @param {u32} new image width
     * @param {u32} new image height
     */
    pub fn fast_resize_to(&mut self, width: u32, height: u32)
    {
        self.fast_resize(width, height);
    }

    /**
     * Resize the image by using a nearest neighbor algorithm
     * 
     * @param {u32} new image width
     * @param {u32} new image height
     */
    fn fast_resize(&mut self, width: u32, height: u32)
    {
        // image 1 (currently loaded image)
        // image 2 (new image to be produced)
        let new_area = width * height;
        let mut i2: Vec<Rgba> = vec![Rgba::black(); new_area as usize];

        let cy = (height as f32) / (self.height as f32); // Scale in y
        let cx = (width as f32) / (self.width as f32); // Scale in x
    
        // write new image
        for y in 0..height
        {
            for x in 0..width
            {
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

    /**
     * Resize the current image by using bilinear interpolation algorithm. Scale
     * image to image size * the factor
     * 
     * @param {f32} scaling factor to apply to image
     */
    pub fn resize_by(&mut self, factor: f32)
    {
        let width = (factor *  (self.width as f32)).round();
        let height = (factor * (self.height as f32)).round();
        self.resize(width as u32, height as u32);
    }

    /**
     * Resize the current image by using bilinear interpolation algorithm. Scale
     * image to specified width and height
     * 
     * @param {u32} new image width
     * @param {u32} new image height
     */
    pub fn resize_to(&mut self, width: u32, height: u32)
    {
        self.resize(width, height);
    }

    /**
     * Resize the image by using a Bilinear interpolation algorithm
     * 
     * @param {u32} new image width
     * @param {u32} new image height
     */
    fn resize(&mut self, width: u32, height: u32)
    {
        // image 1 (currently loaded image)
        // image 2 (new image to be produced)
        let new_area = width * height;
        let mut i2: Vec<Rgba> = vec![Rgba::black(); new_area as usize];

        let step_x = std::cmp::max(self.width - 1, 1) as f32 /  std::cmp::max(width - 1, 1) as f32;
        let step_y = std::cmp::max(self.height - 1, 1) as f32 / std::cmp::max(height - 1, 1) as f32;

        // write new image
        for y in 0..height
        {
            for x in 0..width
            {
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
                if index_2 >= self.get_size() as usize
                {
                    index_2 = index_1;
                }
                if index_3 >= self.get_size() as usize
                {
                    index_3 = index_1;
                }
                if index_4 >= self.get_size() as usize
                {
                    index_4 = index_2;
                }

                let top = Rgba::blur(&self.pixels[index_2], diff_x,
                    &self.pixels[index_1], diff_x1);
                let bottom = Rgba::blur(&self.pixels[index_4], diff_x,
                    &self.pixels[index_3], diff_x1);
                let color = Rgba::blur(&bottom, diff_y, &top, diff_y1);
                i2[index] = color;
            }
        }

        self.width = width;
        self.height = height;
        self.pixels = i2;
    }
}

impl std::fmt::Display for BitMap
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(f, "width: {}\t height: {}\t pixels: {}\t file: {}\n",
            self.width, self.height, self.pixels.len(), self.filename).unwrap();
        for c in &self.pixels
        {
            write!(f, "{}\n", c).unwrap();
        }
        write!(f, "\n")
    }
}