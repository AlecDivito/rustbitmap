use std::ops::{Index, IndexMut};

use super::rgba::Rgba;
use super::file_header::FileHeader;
use super::info_header::InfoHeader;
use super::bit_depth::BitDepth;
use super::map::BitMap;

pub struct PixelData
{
    pixels: Vec<Rgba>,
    padding: u32,
    width: u32,
    height: u32,
    bit_depth: BitDepth,
}

impl PixelData
{

    pub fn from_bitmap(bitmap: &BitMap, bit_depth: BitDepth) -> PixelData
    {
        // TODO: Stop assuming that this is all colors
        PixelData {
            pixels: bitmap.get_pixels().clone(),
            padding: PixelData::get_row_buffer_size(bitmap.get_width(), bit_depth),
            width: bitmap.get_width(),
            height: bitmap.get_height(),
            bit_depth
        }
    }

    pub fn stream(
        bit_stream: &[u8],
        file: &FileHeader,
        info: &InfoHeader
    ) -> PixelData
    {
        let mut pixels: Vec<Rgba> = Vec::new();
        let offset = file.get_off_bits();
        let padding = PixelData::get_row_buffer_size(info.get_width(), info.get_bit_depth());
        let step = info.get_bit_depth().get_step_counter();
        let mut counter = 0;

        for _ in 0..info.get_height()
        {
            for _ in 0..info.get_width()
            {
                // TODO: check if the number of bytes needed exists
                //       If they don't, throw error
                let i = (offset + counter) as usize;
                let pixel = match info.get_bit_depth()
                {
                    BitDepth::AllColors =>
                        Rgba::bgr(bit_stream[i], bit_stream[i + 1], bit_stream[i + 2]),
                    BitDepth::AllColorsAndShades =>
                        Rgba::bgra(bit_stream[i], bit_stream[i + 1], bit_stream[i + 2], bit_stream[i + 3]),
                    _ => Rgba::black()
                };
                pixels.push(pixel);
                counter += step;
            }
            counter += padding;
        }
        PixelData {
            pixels,
            padding,
            width: info.get_width(),
            height: info.get_height(),
            bit_depth: info.get_bit_depth()
        }
    }

    // pub fn get_width(&self) -> u32
    // {
    //     self.width
    // }

    pub fn _get_height(&self) -> u32
    {
        self.height
    }

    pub fn as_bytes(&self) -> Vec<u8>
    {
        let mut bytes = Vec::new();
        let mut counter = self.width;
        for p in &self.pixels
        {
            // i need to watch the row count
            bytes.push(p.get_blue());
            bytes.push(p.get_green());
            bytes.push(p.get_red());
            if self.bit_depth == BitDepth::AllColorsAndShades
            {
                bytes.push(p.get_alpha())
            }
            // after row has been written, pad the bytes to a number divisible by 4
            counter = counter - 1;
            if counter == 0 {
                while bytes.len() % 4 != 0 {
                    bytes.push(0);
                }
                counter = self.width;
            }
        }
        bytes
    }

    pub fn len(&self) -> usize
    {
        self.pixels.len()
    }

    pub fn get_bytes_size(&self) -> u32
    {
        let used_bits = self.pixels.len() as u32 * self.bit_depth.get_step_counter();
        let padding = self.padding * self.height;
        used_bits + padding
    }

    pub fn as_rgba(&self) -> Vec<Rgba>
    {
        self.pixels.clone()
    }

    fn get_row_buffer_size(width: u32, bit_depth: BitDepth) -> u32
    {
        match bit_depth 
        {
            BitDepth::AllColors => match (width * 3) % 4 {
                1 => 3,
                2 => 2,
                3 => 1,
                _ => 0
            }
            _ => 0
        }
    }
}

impl Index<usize> for PixelData
{
    type Output = Rgba;
    fn index<'a>(&'a self, i: usize) -> &'a Rgba
    {
        &self.pixels[i]
    }
}

impl IndexMut<usize> for PixelData
{
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut Rgba {
        &mut self.pixels[i]
    }
}

impl std::fmt::Display for PixelData
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        for p in 0..self.pixels.len()// 0..std::cmp::min(5, self.pixels.len()) as usize
        {
            write!(f, "{}: {}\n", p, self.pixels[p as usize]).unwrap();
        }
        write!(f, "")
    }
}
