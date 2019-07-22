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
        // TODO: Figure out how to get the byte width and byte_padding
        // when the image size is NOT divisible by 4
        let bit_padding =  match pixels.get_width() % 8
        {
            0 => 0,
            _ => 8 - (pixels.get_width() % 8)
        };
        let byte_width = (pixels.get_width() + bit_padding) / 8;
        let byte_padding = match byte_width % 4
        {
            0 => 0,
            _ => 4 - (byte_width % 4)
        };
        pixels.convert_to_bw();
        let mut bytes = Vec::new();
        
        let mut byte: u8 = 0;
        let mut counter = 0;
        let mut shift = 0;
        for i in 0..pixels.len()
        {
            shift = (counter + 1) % 8;
            let step = if pixels[i].is_white() { 1 } else { 0 };
            byte = byte << 1;
            byte += step;
            counter += 1;
            if shift == 0 && i != 0
            {
                bytes.push(byte);
                byte = 0;
            }
            if i % pixels.get_width() as usize == 0 && i != 0
            {
                if bit_padding != 0
                {
                    // println!("{}:\t{}\t{:#b}", counter, bit_padding, byte);
                    byte = byte << bit_padding;
                    bytes.push(byte);
                    byte = 0;
                    counter += bit_padding;
                }
                // println!("{}", counter);
                for _ in 0..byte_padding
                {
                    bytes.push(0);
                }
            }
        }
        if shift != 0
        {
            byte = byte << (8-shift);
            bytes.push(byte);
        }
        for _ in 0..byte_padding
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
        for p in 0..std::cmp::min(22, self.data.len()) as usize
        {
            write!(f, "{}:\t{:#b}\n", p, self.data[p]).unwrap();
        }
        for p in ((self.data.len() - 5)..self.data.len()).rev()
        {
            write!(f, "{}:\t{:#b}\n", p, self.data[p]).unwrap();
        }
        write!(f, "")
    }
}