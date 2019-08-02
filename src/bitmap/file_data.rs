use super::rgb_quad::{RgbQuad};
use super::file_header::FileHeader;
use super::info_header::InfoHeader;
use super::bit_depth::BitDepth;
use super::pixel_data::PixelData;
use super::bit_data::BitData;
use super::rgba::Rgba;
use super::map::BitMap;

pub trait ReadAndWrite<T> {
    fn stream(
        bit_stream: &[u8],
        file: &FileHeader,
        info: &InfoHeader,
        colors: &RgbQuad
    ) -> T;
    fn as_bytes(&self) -> Vec<u8>;
}

pub enum FileData
{
    Bits(BitData),
    Pixels(PixelData),
}

impl FileData
{
    pub fn from_bitmap(bitmap: &BitMap, bit_depth: BitDepth) -> FileData
    {
        FileData::Pixels(PixelData::from_bitmap(bitmap, bit_depth))
    }

    pub fn stream(
        bit_stream: &[u8],
        file: &FileHeader,
        info: &InfoHeader,
        colors: &RgbQuad
    ) -> Option<FileData>
    {
        match info.get_bit_depth()
        {
            BitDepth::BW | BitDepth::Color16Bit | BitDepth::Color256Bit =>
                Some(FileData::Bits(BitData::stream(bit_stream, file, info, colors))),
            BitDepth::AllColors | BitDepth::AllColorsAndShades =>
                Some(FileData::Pixels(PixelData::stream(bit_stream, file, info))),
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

    pub fn as_rgba(&self) -> Vec<Rgba>
    {
        match self
        {
            FileData::Bits(b) => b.as_rgba(),
            FileData::Pixels(p) => p.as_rgba(),
        }
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