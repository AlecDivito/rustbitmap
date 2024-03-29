use super::bit_data::BitData;
use super::bit_depth::BitDepth;
use super::image::BitMap;
use super::info_header::InfoHeader;
use super::pixel_data::PixelData;
use super::rgb_quad::RgbQuad;
use super::rgba::Rgba;

pub enum FileData {
    Bits(BitData),
    Pixels(PixelData),
}

impl FileData {
    ///
    /// Read in bytes from a from_slice and convert it into image data (pixels)
    ///
    pub fn from_slice(bit_stream: &[u8], info: &InfoHeader, colors: &RgbQuad) -> Option<FileData> {
        match info.get_bit_depth() {
            Some(b) => match b {
                BitDepth::Color2Bit | BitDepth::Color16Bit | BitDepth::Color256Bit => Some(
                    FileData::Bits(BitData::from_slice(bit_stream, info, b, colors)),
                ),
                BitDepth::AllColors | BitDepth::AllColorsAndShades => {
                    Some(FileData::Pixels(PixelData::from_slice(bit_stream, info, b)))
                }
            },
            None => None,
        }
    }

    ///
    /// Convert image data (pixels) from a bitmap into bits and bytes
    ///
    pub fn from_bitmap(bitmap: &BitMap, bit_depth: BitDepth) -> FileData {
        match bit_depth {
            BitDepth::Color2Bit | BitDepth::Color16Bit | BitDepth::Color256Bit => {
                FileData::Bits(BitData::from_bitmap(bitmap, bit_depth))
            }
            _ => FileData::Pixels(PixelData::from_bitmap(bitmap, bit_depth)),
        }
    }

    ///
    /// Get then length of the lists of colors
    ///
    pub fn len(&self) -> usize {
        match self {
            FileData::Bits(b) => b.len(),
            FileData::Pixels(p) => p.len(),
        }
    }

    ///
    /// get the size of the list of colors if they were to be convert
    ///
    pub fn get_bytes_size(&self) -> u32 {
        match self {
            FileData::Bits(b) => b.get_bytes_size(),
            FileData::Pixels(p) => p.get_bytes_size(),
        }
    }

    ///
    /// convert the list of colors to bytes
    ///
    pub fn as_bytes(&self) -> Vec<u8> {
        match self {
            FileData::Bits(b) => b.as_bytes(),
            FileData::Pixels(p) => p.as_bytes(),
        }
    }

    ///
    /// convert the list of colors to RGBA
    ///
    pub fn as_rgba(&self) -> Vec<Rgba> {
        match self {
            FileData::Bits(b) => b.as_rgba(),
            FileData::Pixels(p) => p.as_rgba(),
        }
    }
}

#[cfg(debug_assertions)]
impl std::fmt::Display for FileData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FileData::Bits(b) => write!(f, "{}", b),
            FileData::Pixels(p) => write!(f, "{}", p),
        }
    }
}
