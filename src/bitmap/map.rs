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

    pub fn get_filename(&self) -> &String
    {
        &self.filename
    }
}