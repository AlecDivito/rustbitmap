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
        BitMap {
            filename: String::default(),
            width,
            height,
            pixels: Vec::with_capacity((width * height) as usize)
        }
    }

    pub fn new_colored(width: u32, height: u32, color: Rgba) -> BitMap
    {
        let pixels = vec![color; (width * height) as usize];
        BitMap {
            filename: String::default(),
            width,
            height,
            pixels
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

    pub fn save(&self) -> std::io::Result<()>
    {
        self.save_as(&self.filename)
    }

    // pub fn simplify_and_save() -> std::io::Result<()>
    // {

    // }

    pub fn save_as(&self, filename: &str) -> std::io::Result<()>
    {
        let file = File::create(self);
        use std::io::Write;
        let mut bit_stream = unsafe { file.to_bytes() };
        let mut file = std::fs::File::create(filename)?;
        file.write_all(bit_stream.as_mut_slice())?;
        Ok(())
    }

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

                // print!("{} => ({} {} {} {}): ", index, index_1, index_2, index_3, index_4);

                let top = Rgba::blur(&self.pixels[index_2], diff_x,
                    &self.pixels[index_1], diff_x1);
                let bottom = Rgba::blur(&self.pixels[index_4], diff_x,
                    &self.pixels[index_3], diff_x1);
                let color = Rgba::blur(&bottom, diff_y, &top, diff_y1);
                // println!("{}", color);
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