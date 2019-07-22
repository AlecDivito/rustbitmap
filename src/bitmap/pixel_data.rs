use std::ops::{Index, IndexMut};

use super::rgb_quad::{Rgb, RgbQuad};
use super::file_header::FileHeader;
use super::info_header::InfoHeader;

pub struct Pixel
{
    red: u8,
    green: u8,
    blue: u8,
}

impl Pixel
{
    pub fn new(blue: u8, green: u8, red: u8) -> Pixel
    {
        Pixel {red,green,blue}
    }

    pub fn _copy(color: &Rgb) -> Pixel
    {
        Pixel {
            red: color.get_red(),
            green: color.get_green(),
            blue: color.get_blue(),
        }
    }

    pub fn is_white(&self) -> bool
    {
        self.red == 255 && self.green == 255 && self.blue == 255
    }

    pub fn set_red(&mut self, red: u8)
    {
        self.red = red;
    }

    pub fn set_green(&mut self, green: u8)
    {
        self.green = green;
    }

    pub fn set_blue(&mut self, blue: u8)
    {
        self.blue = blue;
    }

}

impl std::fmt::Display for Pixel
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(f, "Red: {}, Green: {}, Blue: {}",
            self.red,
            self.green,
            self.blue)
    }
}

pub struct PixelData
{
    pixels: Vec<Pixel>,
    padding: u32,
    width: u32,
    height: u32,
}

impl PixelData
{

    pub fn stream(
        bit_stream: &[u8],
        file: &FileHeader,
        info: &InfoHeader,
        _colors: &RgbQuad
    ) -> PixelData
    {
        let mut pixels: Vec<Pixel> = Vec::new();
        let offset = file.get_off_bits();
        let padding = info.get_row_buffer_size(info.get_bit_count());
        let step = 3; // TODO: why is step 3 (I know why but documentation)
        let mut counter = 0;

        for _ in 0..info.get_height()
        {
            for _ in 0..info.get_width()
            {
                let i = (offset + counter) as usize;
                let p = Pixel::new(bit_stream[i], bit_stream[i + 1], bit_stream[i + 2]);
                pixels.push(p);
                counter += step;
            }
            counter += padding;
        }
        PixelData {
            pixels,
            padding,
            width: info.get_width(),
            height: info.get_height()
        }
    }

    pub fn convert_to_bw(&mut self)
    {
        // update all colors to ether black or white
        for c in &mut self.pixels
        {
            if !c.is_white()
            {
                c.set_blue(0);
                c.set_red(0);
                c.set_green(0);
            }
        }
    }

    pub fn get_width(&self) -> u32
    {
        self.width
    }

    pub fn _get_height(&self) -> u32
    {
        self.height
    }

    pub fn as_bytes(&self) -> Vec<u8>
    {
        let mut bytes = Vec::new();
        for p in &self.pixels
        {
            bytes.push(p.blue);
            bytes.push(p.green);
            bytes.push(p.red);
        }
        bytes
    }

    pub fn len(&self) -> usize
    {
        self.pixels.len()
    }

    pub fn get_bytes_size(&self) -> u32
    {
        let used_bits = self.pixels.len() as u32 * 3;
        let padding = self.padding * self.height;
        used_bits + padding
    }
}

impl Index<usize> for PixelData
{
    type Output = Pixel;
    fn index<'a>(&'a self, i: usize) -> &'a Pixel
    {
        &self.pixels[i]
    }
}

impl IndexMut<usize> for PixelData
{
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut Pixel {
        &mut self.pixels[i]
    }
}

impl std::fmt::Display for PixelData
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        for p in 0..5//&self.pixels
        {
            write!(f, "{}: {}\n", p, self.pixels[p]).unwrap();
        }
        write!(f, "")
    }
}
