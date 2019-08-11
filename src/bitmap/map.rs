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

    pub fn resize_by(&mut self, factor: f32)
    {
        let width = (factor *  (self.width as f32)).round();
        let height = (factor * (self.height as f32)).round();
        self.resize(width as u32, height as u32);
    }

    pub fn resize_to(&mut self, width: u32, height: u32)
    {
        self.resize(width, height);
    }

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

    pub fn get_filename(&self) -> &String
    {
        &self.filename
    }

    /**
     * Resize the image by using a nearest neighbor algorithm
     * 
     * @param {u32} new image width
     * @param {u32} new image height
     */
    fn resize(&mut self, width: u32, height: u32)
    {
        // image 1 (currently loaded image)
        let m1 = self.width as i32;
        let n1 = self.height as i32;

        // image 2 (new image to be produced)
        let m2 = width;
        let n2 = height;
        let new_area = m2 * n2;
        let mut i2: Vec<Rgba> = vec![Rgba::black(); new_area as usize];

        let cy = (n2 as f32) / (n1 as f32); // Scale in x
        let cx = (m2 as f32) / (m1 as f32); // Scale in y
    
        // write new image
        for y in 0..n2
        {
            for x in 0..m2
            {
                // Calculate position in input image
                // then just pick the nearest neighbor to (v, w)
                let v = ((x as f32) / cx).floor() as i32; // x, w
                let w = ((y as f32) / cy).floor() as i32; // y, h
                let i1_index = ((w * m1) + v) as usize;
                let i2_index = ((y * m2) + x) as usize;
                i2[i2_index] = self.pixels[i1_index];
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
        write!(f, "width: {}, height: {}, pixels: {}, file: {}\n",
            self.width, self.height, self.pixels.len(), self.filename).unwrap();
        for c in &self.pixels
        {
            write!(f, "{}\n", c).unwrap();
        }
        write!(f, "\n")
    }
}