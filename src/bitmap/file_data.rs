use super::rgb_quad::{RgbQuad};
use super::file_header::FileHeader;
use super::info_header::InfoHeader;
use super::bit_count::BitCount;
use super::pixel_data::PixelData;
use super::bit_data::BitData;

pub trait ReadAndWrite<T> {
    fn stream(
        bit_stream: &[u8],
        file: &FileHeader,
        info: &InfoHeader,
        colors: &RgbQuad
    ) -> T;
    fn as_bytes(&self) -> Vec<u8>;
}

pub trait Convert<T>
{
    fn convert(&self) -> Vec<T>;
}

pub enum FileData
{
    Bits(BitData),
    Pixels(PixelData),
}

impl FileData
{
    pub fn stream(
        bit_stream: &[u8],
        file: &FileHeader,
        info: &InfoHeader,
        colors: &RgbQuad
    ) -> Option<FileData>
    {
        match info.get_bit_count()
        {
            BitCount::BW =>
                Some(FileData::Bits(BitData::stream(bit_stream, file, info, colors))),
            BitCount::AllColors =>
                Some(FileData::Pixels(PixelData::stream(bit_stream, file, info, colors))),
            _ => None,
        }
    }

    pub fn len(&self) -> usize
    {
        match self
        {
            FileData::Bits(b) => b.len(),
            FileData::Pixels(p) => p.len(),
        }
    }

    pub fn get_bytes_size(&self) -> u32
    {
        match self
        {
            FileData::Bits(b) => b.get_bytes_size(),
            FileData::Pixels(p) => p.get_bytes_size(),
        }
    }

    pub fn as_bytes(&self) -> Vec<u8>
    {
        match self
        {
            FileData::Bits(b) => b.as_bytes(),
            FileData::Pixels(p) => p.as_bytes(),
        }   
    }

    pub fn convert_pixels_to_bw(&mut self) -> FileData
    {
        FileData::Bits(BitData::convert(self.pixels()))
    }

    fn pixels(&mut self) -> &mut PixelData
    {
        if let FileData::Pixels(p) = self { p } else { panic!("Not Pixels") }
    }

    fn _bits(&mut self) -> &mut BitData
    {
        if let FileData::Bits(b) = self { b } else { panic!("Not Bytes") }
    }
}

impl std::fmt::Display for FileData
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        match self
        {
            FileData::Bits(b) => write!(f, "{}", b),
            FileData::Pixels(p) => write!(f, "{}", p),
        }
    }
}