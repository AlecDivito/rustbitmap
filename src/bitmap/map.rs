// use std::error::Error;
use super::file::File;
use super::rgba::Rgba;

pub struct BitMap
{
    width: u32,
    height: u32,
    pixels: Vec<Rgba>
}

impl BitMap
{
    pub fn read(filename: &str) -> Option<BitMap>
    {
        let file = File::read(filename).unwrap();
        Some(file.to_bitmap())
    }

    pub fn new(width: u32, height: u32) -> BitMap
    {
        BitMap {
            width,
            height,
            pixels: Vec::with_capacity((width * height) as usize)
        }
    }

    pub fn new_colored(width: u32, height: u32, color: Rgba) -> BitMap
    {
        let pixels = vec![color; (width * height) as usize];
        BitMap {
            width,
            height,
            pixels
        }
    }

    // pub fn save() -> std::io::Result<()>
    // {
    //     use std::io::Write;

    //     let mut bit_stream = unsafe { self.get_s_bytes() };
    //     let mut file = std::fs::File::create(filename)?;
    //     file.write_all(bit_stream.as_mut_slice())?;
    //     Ok(())
    // }

    // pub fn simplify_and_save() -> std::io::Result<()>
    // {

    // }

    // pub fn save_as() -> std::io::Result<()>
    // {

    // }

    // pub fn simplify_and_save_as() -> std::io::Result<()>
    // {

    // }
}