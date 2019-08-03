use super::file_header::FileHeader;
use super::info_header::InfoHeader;
use super::bit_depth::BitDepth;
use super::rgb_quad::RgbQuad;
use super::rgba::Rgba;

pub struct BitData
{
    // stores colors key on each 6 bits of 8 available bits 
    width: u32,
    // height: u32,
    colors: Vec<Rgba>,
    bytes: Vec<u8>,
    bit_depth: BitDepth,
}

impl BitData
{
    // pub fn convert(pixels: &mut PixelData) -> BitData
    // {
    //     // TODO: Figure out how to get the byte width and byte_padding
    //     // when the image size is NOT divisible by 4
    //     let bit_padding =  match pixels.get_width() % 8
    //     {
    //         0 => 0,
    //         _ => 8 - (pixels.get_width() % 8)
    //     };
    //     let byte_width = (pixels.get_width() + bit_padding) / 8;
    //     let byte_padding = match byte_width % 4
    //     {
    //         0 => 0,
    //         _ => 4 - (byte_width % 4)
    //     };
    //     pixels.convert_to_bw();
    //     let mut bytes = Vec::new();
        
    //     let mut byte: u8 = 0;
    //     let mut counter = 0;
    //     let mut shift = 0;
    //     for i in 0..pixels.len()
    //     {
    //         counter += 1;
    //         shift = counter % 8;
    //         let step = if pixels[i].is_white() { 1 } else { 0 };
    //         byte = byte << 1;
    //         byte += step;

    //         if shift == 0 && i != 0 && pixels.get_width() >= 8
    //         {
    //             bytes.push(byte);
    //             byte = 0;
    //         }
    //         if counter % pixels.get_width() == 0 && i != 0
    //         {
    //             if bit_padding != 0
    //             {
    //                 byte = byte << bit_padding;
    //                 bytes.push(byte);
    //                 byte = 0;
    //                 counter = 0;
    //             }

    //             for _ in 0..byte_padding
    //             {
    //                 bytes.push(0);
    //             }
    //         }
    //     }
    //     if shift != 0
    //     {
    //         byte = byte << (8-shift);
    //         bytes.push(byte);
    //     }
    //     for _ in 0..byte_padding
    //     {
    //         bytes.push(0);
    //     }

    //     BitData { bytes: bytes }
    // }

    pub fn stream(
        bit_stream: &[u8],
        file: &FileHeader,
        info: &InfoHeader,
        colors: &RgbQuad
    ) -> BitData
    {
        let offset = file.get_off_bits() as usize;
        // for byte in offset..bits
        let mut bytes = Vec::new();
        for index in offset..bit_stream.len()
        {
            bytes.push(bit_stream[index]);
        }
        BitData {
            width: info.get_width(),
            // height: info.get_height(),
            bit_depth: info.get_bit_depth(),
            colors: colors.clone_colors(),
            bytes
        }
    }

    pub fn as_bytes(&self) -> Vec<u8>
    {
        self.bytes.clone()
    }

    pub fn as_rgba(&self) -> Vec<Rgba>
    {
        let mut pixels = Vec::new();
        let step = self.bit_depth.get_step_counter();
        // figure out how much padding is on each row
        // this is needed because for each row of a bmp image needs to finish
        // with a width of bytes that is divisible by 4. Here we are figuring out
        // how much bit padding and byte padding we need.
        // TODO: move to it's own class
        let bit_width = self.width * self.bit_depth.get_step_counter();
        let bit_padding = match bit_width % 8
        {
            0 => 0,
            _ => 8 - (self.width % 8)
        };
        let byte_width = (bit_width + bit_padding) / 8;
        let byte_padding = match byte_width % 4
        {
            0 => 0,
            _ => 4 - (byte_width % 4)
        };

        // loop through all the bytes
        let mut byte_padding_counter = 0;
        let mut start_reading_again = false;
        for byte in &self.bytes
        {
            if byte_padding_counter > 0
            {
                byte_padding_counter = byte_padding_counter - 1;
                continue;
            }
            // loop through the bits of the byte
            for byte_indexes in (0..(8 / step)).rev()
            {
                // so this works on every time after the pixels length
                // reaches 72. We need a way to tell it that it is a new line
                // (at lease once)
                if pixels.len() as u32 % self.width == 0
                    && pixels.len() != 0
                    && !start_reading_again
                {
                    break;
                }
                // bits could be:
                // (1) -> 0 1 2 3 4 5 6 7
                // (4) -> 0 1
                // (8) -> 0
                let starting_bit = byte_indexes * step;
                let ending_bit = starting_bit + step;
                let mut index: usize = 0;
                for bit_index in (starting_bit..ending_bit).rev()
                {
                    index = index << 1;
                    let next_bit = (byte >> bit_index) & 1;
                    index = index + next_bit as usize;
                }
                pixels.push(self.colors[index]);
            }
            if start_reading_again
            {
                start_reading_again = false;
            }
            if (pixels.len() as u32) % self.width == 0
                && pixels.len() != 0
                && start_reading_again == false
            {
                start_reading_again = true;
                byte_padding_counter = byte_padding;
            }
        }
        pixels
    }

    pub fn len(&self) -> usize
    {
        self.bytes.len()
    }

    pub fn get_bytes_size(&self) -> u32
    {
        self.bytes.len() as u32
    }

    // pub fn get_bit_padding(&self) -> u32
    // {
    //     match self.width * self.bit_depth.get_step_counter() % 8
    //     {
    //         0 => 0,
    //         _ => 8 - (self.width % 8)
    //     }
    // }
}

impl std::fmt::Display for BitData
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        for p in 0..std::cmp::min(22, self.bytes.len()) as usize
        {
            write!(f, "{}:\t{:#b}\n", p, self.bytes[p]).unwrap();
        }
        // for p in ((self.bytes.len() - 5)..self.bytes.len()).rev()
        // {
        //     write!(f, "{}:\t{:#b}\n", p, self.bytes[p]).unwrap();
        // }
        write!(f, "")
    }
}