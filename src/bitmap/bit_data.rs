use super::rgb_quad::RgbQuad;
use super::file_header::FileHeader;
use super::info_header::InfoHeader;
use super::pixel_data::PixelData;

pub struct BitData
{
    // stores colors key on each 6 bits of 8 available bits 
    data: Vec<u8>,
}

impl BitData
{
    pub fn convert(pixels: &mut PixelData) -> BitData
    {
        let byte_width = pixels.get_width() / 8;
        let padding = 4 - (byte_width % 4);
        pixels.convert_to_bw();
        let mut bytes = Vec::new();
        
        let mut byte: u8 = 0;
        let mut counter = 0;
        let mut shift = 0;
        for i in 0..pixels.len()
        {
            shift = (i + 1) % 8;
            let step = if pixels[i].is_white() { 1 } else { 0 };
            byte = byte << 1;
            byte += step;

            if shift == 0
            {
                bytes.push(byte);
                byte = 0;

                counter += 1;
                if counter % byte_width == 0
                {
                    for _ in 0..padding
                    {
                        bytes.push(0);
                    }
                    counter = 0;
                }
            }
        }
        for _ in 0..(8-shift)
        {
            byte = byte << 1;
        }
        for _ in 0..padding
        {
            bytes.push(0);
        }

        BitData { data: bytes }
    }

    pub fn stream(
        bit_stream: &[u8],
        file: &FileHeader,
        _info: &InfoHeader,
        _colors: &RgbQuad
    ) -> BitData
    {
        let offset = file.get_off_bits() as usize;
        // for byte in offset..bits
        let mut bytes = Vec::new();
        for index in offset..bit_stream.len()
        {
            bytes.push(bit_stream[index]);
        }
        BitData { data: bytes }
    }

    pub fn as_bytes(&self) -> Vec<u8>
    {
        self.data.clone()
    }

    pub fn len(&self) -> usize
    {
        self.data.len()
    }

    pub fn get_bytes_size(&self) -> u32
    {
        self.data.len() as u32
    }
}

impl std::fmt::Display for BitData
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        for p in 0..5// &self.data
        {
            write!(f, "{}:\t{:#b}\n", p, self.data[p]).unwrap();
        }
        write!(f, "")
    }
}