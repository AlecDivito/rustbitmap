use super::util;

pub struct FileHeader {
    /// Must always be set to 'BM' to declare that this is a .bmp-file
    bitmap_type: [char; 2],
    /// specifies the size of the file in bytes
    size: u32,
    reserved1: u16,
    reserved2: u16,
    /// Specifies the offset from the beginning of the file to the bitmap data.
    off_bits: u32,
}

impl FileHeader {
    ///
    /// Create a new Bit Map File Header
    ///
    /// @param {u32} size of file in bytes
    /// @param {u32} offset from beginning of file to bitmap data
    /// @return {FileHeader}
    ///
    pub fn new(data_size: u32, color_size: u32, info_size: u32) -> FileHeader {
        let size = data_size + color_size + info_size + 14;
        let off_bits = info_size + 14;
        FileHeader {
            bitmap_type: ['B', 'M'],
            size,
            reserved1: 0,
            reserved2: 0,
            off_bits,
        }
    }

    ///
    /// Create a new Bit Map File Header from stream of bytes
    ///
    /// @param {&[u8; 14]} 14 byte long slice
    /// @return {FileHeader}
    ///
    pub fn stream(bit_stream: &[u8]) -> FileHeader {
        let bitmap_type = [bit_stream[0] as char, bit_stream[1] as char];
        let mut i = 2;
        FileHeader {
            bitmap_type,
            size: util::byte_slice_to_u32(bit_stream, &mut i),
            reserved1: util::byte_slice_to_u16(bit_stream, &mut i),
            reserved2: util::byte_slice_to_u16(bit_stream, &mut i),
            off_bits: util::byte_slice_to_u32(bit_stream, &mut i),
        }
    }

    ///
    /// Get FileHeader as a array of bytes
    ///
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&[self.bitmap_type[0] as u8, self.bitmap_type[1] as u8]);
        bytes.extend_from_slice(&util::byte_slice_from_u32(self.size));
        bytes.extend_from_slice(&util::byte_slice_from_u16(self.reserved1));
        bytes.extend_from_slice(&util::byte_slice_from_u16(self.reserved2));
        bytes.extend_from_slice(&util::byte_slice_from_u32(self.off_bits));
        bytes
    }

    pub fn get_byte_size(&self) -> u32 {
        14
    }

    pub fn _get_size(&self) -> u32 {
        self.size
    }

    pub fn get_off_bits(&self) -> u32 {
        self.off_bits
    }
}

impl std::fmt::Display for FileHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Type: {}{}, Size: {}, res1: {}, res2: {}, offset: {}",
            self.bitmap_type[0],
            self.bitmap_type[1],
            self.size,
            self.reserved1,
            self.reserved2,
            self.off_bits
        )
    }
}
