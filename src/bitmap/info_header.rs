use super::bit_depth::BitDepth;
use super::util;

use super::image::BitMap;

pub struct InfoHeader {
    /// specifies the size of the BitMapFileHeader structure, in bytes
    size: u32,
    /// specifies the width of the image, in pixels
    width: u32,
    /// specifies the height of the image, in pixels
    height: u32,
    /// specifies the number of planes of the target device, must be set to zero
    planes: u16,
    /// specifies the number of bits per pixel
    /// possible values are as follows:
    ///  - 1 (black / white)
    ///  - 4 (16 colors)
    ///  - 8 (256 colors)
    ///  - 24 (16.7 million colors)
    bit_depth: u16,
    /// specifies the type of compression, usually set to zero (no compression)
    compression: u32,
    /// specifies the size of the image data, in bytes. If there is no
    /// compression, it is valid to set this member to zero
    size_image: u32,
    /// specifies the the horizontal pixels per meter on the designated target
    /// device, usually set to zero.
    x_pixels_per_meter: u32,
    /// specifies the vertical pixels per meter on the designated target device,
    /// usually set to zero
    y_pixels_per_meter: u32,
    /// specifies the number of colors used in the bitmap, if set to zero the
    /// number of colors is calculated using the biBitDepth member.
    colors_used: u32,
    /// specifies the number of color that are 'important' for the bitmap, if set
    /// to zero, all colors are important
    colors_important: u32,
}

impl InfoHeader {
    ///
    /// Create a header based on a bitmap.
    ///
    pub fn from(bitmap: &BitMap, bit_depth: BitDepth) -> InfoHeader {
        let colors_used = match bit_depth {
            BitDepth::Color2Bit | BitDepth::Color16Bit | BitDepth::Color256Bit => {
                bitmap.get_all_unique_colors().len()
            }
            _ => 0,
        } as u32;
        InfoHeader {
            size: 40,
            width: bitmap.get_width(),
            height: bitmap.get_height(),
            bit_depth: bit_depth as u16,
            planes: 1,
            compression: 0,
            size_image: 0,
            x_pixels_per_meter: 0,
            y_pixels_per_meter: 0,
            colors_used,
            colors_important: 0,
        }
    }

    ///
    /// Read in header based on stream of bytes
    ///
    /// Bytes should be in correct order
    ///
    /// 1. size   as a u32
    /// 2. width  as a u32
    /// 3. height as a u32
    /// 4. planes as a u16
    /// 5. bit_depth as a u16
    /// 6. compression as a u32
    /// 7. size_image as a u32
    /// 8. x_pixels_per_meter as a u32
    /// 9. y_pixels_per_meter as a u32
    /// 10. colors_used as a u32
    /// 11. colors_important as a u32
    ///
    pub fn stream(bit_stream: &[u8]) -> InfoHeader {
        // starts at 14
        let mut i: usize = 14;
        InfoHeader {
            size: util::byte_slice_to_u32(bit_stream, &mut i),
            width: util::byte_slice_to_u32(bit_stream, &mut i),
            height: util::byte_slice_to_u32(bit_stream, &mut i),
            planes: util::byte_slice_to_u16(bit_stream, &mut i),
            bit_depth: util::byte_slice_to_u16(bit_stream, &mut i),
            compression: util::byte_slice_to_u32(bit_stream, &mut i),
            size_image: util::byte_slice_to_u32(bit_stream, &mut i),
            x_pixels_per_meter: util::byte_slice_to_u32(bit_stream, &mut i),
            y_pixels_per_meter: util::byte_slice_to_u32(bit_stream, &mut i),
            colors_used: util::byte_slice_to_u32(bit_stream, &mut i),
            colors_important: util::byte_slice_to_u32(bit_stream, &mut i),
        }
    }

    ///
    /// Convert struct back into bytes
    ///
    /// We need to do this manually because byte count matters (although I guess
    /// we could edit the header and up the size) because when rust converts
    /// the structure to bytes all the elements become the size of the biggest
    /// element
    ///
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&util::byte_slice_from_u32(self.size));
        bytes.extend_from_slice(&util::byte_slice_from_u32(self.width));
        bytes.extend_from_slice(&util::byte_slice_from_u32(self.height));
        bytes.extend_from_slice(&util::byte_slice_from_u16(self.planes));
        bytes.extend_from_slice(&util::byte_slice_from_u16(self.bit_depth));
        bytes.extend_from_slice(&util::byte_slice_from_u32(self.compression));
        bytes.extend_from_slice(&util::byte_slice_from_u32(self.size_image));
        bytes.extend_from_slice(&util::byte_slice_from_u32(self.x_pixels_per_meter));
        bytes.extend_from_slice(&util::byte_slice_from_u32(self.y_pixels_per_meter));
        bytes.extend_from_slice(&util::byte_slice_from_u32(self.colors_used));
        bytes.extend_from_slice(&util::byte_slice_from_u32(self.colors_important));
        bytes
    }

    ///
    /// Get the size of the information header in bytes
    ///
    pub fn get_byte_size(&self) -> u32 {
        self.size
    }

    ///
    /// Get the bit depth of the bitmap
    ///
    pub fn get_bit_depth(&self) -> Option<BitDepth> {
        match self.bit_depth {
            1 => Some(BitDepth::Color2Bit),
            4 => Some(BitDepth::Color16Bit),
            8 => Some(BitDepth::Color256Bit),
            24 => Some(BitDepth::AllColors),
            32 => Some(BitDepth::AllColorsAndShades),
            _ => None,
        }
    }

    ///
    /// Get the width of the image
    ///
    pub fn get_width(&self) -> u32 {
        self.width
    }

    ///
    /// Get the height of the image
    ///
    pub fn get_height(&self) -> u32 {
        self.height
    }

    ///
    /// Get the number of colors used to create the image
    ///
    pub fn get_colors_used(&self) -> u32 {
        self.colors_used
    }
}

#[cfg(debug_assertions)]
impl std::fmt::Display for InfoHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "
        Header Size: {}, Width: {}, Height: {}, Bit Count: {}
        Planes: {}, compression: {}, image size: {}
        x pxls per meter: {}, y pxls per meter: {}
        colors_used: {} colors_important: {}",
            self.size,
            self.width,
            self.height,
            self.bit_depth,
            self.planes,
            self.compression,
            self.size_image,
            self.x_pixels_per_meter,
            self.y_pixels_per_meter,
            self.colors_used,
            self.colors_important
        )
    }
}

#[cfg(test)]
mod test {
    use super::BitDepth;
    use super::BitMap;
    use super::InfoHeader;

    #[test]
    fn get_info_size_in_bytes_after_bitmap_conversion() {
        let b = BitMap::new(10, 10);
        let data = InfoHeader::from(&b, BitDepth::AllColors);
        assert_eq!(data.get_byte_size(), 40);
    }

    #[test]
    fn get_width_and_height_after_bitmap_conversion() {
        let b = BitMap::new(10, 10);
        let data = InfoHeader::from(&b, BitDepth::AllColors);
        assert_eq!(data.get_width(), 10);
        assert_eq!(data.get_height(), 10);
    }

    #[test]
    fn get_colors_used_after_bitmap_conversion_24_bit() {
        let b = BitMap::new(10, 10);
        let data = InfoHeader::from(&b, BitDepth::AllColors);
        assert_eq!(data.get_colors_used(), 0);
    }
}
