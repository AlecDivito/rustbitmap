use super::util;
use super::bit_depth::BitDepth;

use super::map::BitMap;

pub struct InfoHeader
{
    // specifies the size of the BitMapFileHeader structure, in bytes
    bi_size: u32,
    // specifies the width of the image, in pixels
    bi_width: u32,
    // specifies the height of the image, in pixels
    bi_height: u32,
    // specifies the number of planes of the target device, must be set to zero
    bi_planes: u16,
    // specifies the number of bits per pixel
    // possible values are as follows:
    //  - 1 (black / white)
    //  - 4 (16 colors)
    //  - 8 (256 colors)
    //  - 24 (16.7 million colors)
    bi_bit_depth: u16,
    // specifies the type of compression, usually set to zero (no compression)
    bi_compression: u32,
    // specifies the size of the image data, in bytes. If there is no
    // compression, it is valid to set this member to zero
    bi_size_image: u32,
    // specifies the the horizontal pixels per meter on the designated target
    /// device, usually set to zero.
    bi_x_pxls_per_meter: u32,
    // specifies the vertical pixels per meter on the designated target device,
    // usually set to zero
    bi_y_pxls_per_meter: u32,
    // specifies the number of colors used in the bitmap, if set to zero the
    // number of colors is calculated using the biBitDepth member.
    bi_clr_used: u32,
    // specifies the number of color that are 'important' for the bitmap, if set
    // to zero, all colors are important
    bi_clr_important: u32,
}

impl InfoHeader
{
    pub fn from_bitmap(bitmap: &BitMap, bit_depth: BitDepth) -> InfoHeader
    {
        InfoHeader {
            bi_size: 40,
            bi_width: bitmap.get_width(),
            bi_height: bitmap.get_height(),
            bi_bit_depth: bit_depth as u16,
            bi_planes: 1,
            bi_compression: 0,
            bi_size_image: 0,
            bi_x_pxls_per_meter: 0,
            bi_y_pxls_per_meter: 0,
            bi_clr_used: 0,
            bi_clr_important: 0,
        }
    }

    /**
     * Create a new Bit Map Info Header
     * 
     * @param {u32} size of BitMap::FileHeader in bytes
     * @param {u32} width of image in pixels
     * @param {u32} height of image in pixels
     * @param {u16} number of bits per pixel
     * @return {InfoHeader}
     */
    // pub fn new(bi_size: u32, bi_width: u32, bi_height: u32, bi_bit_depth: u16) -> InfoHeader
    // {
    //     InfoHeader {
    //         bi_size,
    //         bi_width,
    //         bi_height,
    //         bi_bit_depth,
    //         bi_planes: 0,
    //         bi_compression: 0,
    //         bi_size_image: 0,
    //         bi_x_pxls_per_meter: 0,
    //         bi_y_pxls_per_meter: 0,
    //         bi_clr_used: 0,
    //         bi_clr_important: 0,
    //     }
    // }

    pub fn stream(bit_stream: &[u8]) -> InfoHeader
    {
        // starts at 14
        let mut i: usize = 14;
        InfoHeader {
            bi_size: util::byte_slice_to_u32(bit_stream, & mut i),
            bi_width: util::byte_slice_to_u32(bit_stream, & mut i),
            bi_height: util::byte_slice_to_u32(bit_stream, & mut i),
            bi_planes: util::byte_slice_to_u16(bit_stream, & mut i),
            bi_bit_depth: util::byte_slice_to_u16(bit_stream, & mut i),
            bi_compression: util::byte_slice_to_u32(bit_stream, & mut i),
            bi_size_image: util::byte_slice_to_u32(bit_stream, & mut i),
            bi_x_pxls_per_meter: util::byte_slice_to_u32(bit_stream, & mut i),
            bi_y_pxls_per_meter: util::byte_slice_to_u32(bit_stream, & mut i),
            bi_clr_used: util::byte_slice_to_u32(bit_stream, & mut i),
            bi_clr_important: util::byte_slice_to_u32(bit_stream, & mut i),
        }
    }

    pub fn as_bytes(&self) -> Vec<u8>
    {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&util::byte_slice_from_u32(self.bi_size));
        bytes.extend_from_slice(&util::byte_slice_from_u32(self.bi_width));
        bytes.extend_from_slice(&util::byte_slice_from_u32(self.bi_height));
        bytes.extend_from_slice(&util::byte_slice_from_u16(self.bi_planes));
        bytes.extend_from_slice(&util::byte_slice_from_u16(self.bi_bit_depth));
        bytes.extend_from_slice(&util::byte_slice_from_u32(self.bi_compression));
        bytes.extend_from_slice(&util::byte_slice_from_u32(self.bi_size_image));
        bytes.extend_from_slice(&util::byte_slice_from_u32(self.bi_x_pxls_per_meter));
        bytes.extend_from_slice(&util::byte_slice_from_u32(self.bi_y_pxls_per_meter));
        bytes.extend_from_slice(&util::byte_slice_from_u32(self.bi_clr_used));
        bytes.extend_from_slice(&util::byte_slice_from_u32(self.bi_clr_important));
        bytes
    }

    pub fn get_info_size(&self) -> u32
    {
        self.bi_size
    }

    pub fn get_bit_depth(&self) -> BitDepth
    {
        match self.bi_bit_depth {
            1 => BitDepth::BW,
            4 => BitDepth::Color16Bit,
            8 => BitDepth::Color256Bit,
            24 => BitDepth::AllColors,
            32 => BitDepth::AllColorsAndShades,
            _ => BitDepth::UNKNOWN,
        }
    }

    // pub fn set_bit_depth(&mut self, bit: BitDepth)
    // {
    //     self.bi_bit_depth = bit as u16;
    // }

    // pub fn set_colors_used(&mut self, colors: u32)
    // {
    //     self.bi_clr_used = colors;
    // }

    pub fn get_width(&self) -> u32
    {
        self.bi_width
    }

    pub fn get_height(&self) -> u32
    {
        self.bi_height
    }

    pub fn get_colors_used(&self) -> u32
    {
        self.bi_clr_used
    }

    // pub fn set_image_size(&mut self, size: u32)
    // {
    //     self.bi_size_image = size;
    // }
}


impl std::fmt::Display for InfoHeader
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(f, "
        Header Size: {}, Width: {}, Height: {}, Bit Count: {}
        Planes: {}, compression: {}, image size: {}
        x pxls per meter: {}, y pxls per meter: {}
        clr_used: {} clr_important: {}",
            self.bi_size,
            self.bi_width,
            self.bi_height,
            self.bi_bit_depth,
            self.bi_planes,
            self.bi_compression,
            self.bi_size_image,
            self.bi_x_pxls_per_meter,
            self.bi_y_pxls_per_meter,
            self.bi_clr_used,
            self.bi_clr_important)
    }
}