use super::rgb_quad::RgbQuad;
use super::file_header::FileHeader;
use super::info_header::InfoHeader;
use super::bit_count::BitCount;

pub struct PixelData
{
    red: u8,
    green: u8,
    blue: u8,
}

impl PixelData
{
    pub fn new(blue: u8, green: u8, red: u8) -> PixelData
    {
        PixelData {red,green,blue}
    }

    pub fn copy(color: &RgbQuad) -> PixelData
    {
        PixelData {
            red: color.get_red(),
            green: color.get_green(),
            blue: color.get_blue(),
        }
    }

    pub fn stream(
        bit_stream: &[u8],
        file: &FileHeader,
        info: &InfoHeader,
        colors: &Vec<RgbQuad>
    ) -> Vec<PixelData>
    {
        let mut array: Vec<PixelData> = Vec::new();
        let offset = file.get_off_bits() as usize;
        let row_buffer = info.get_row_buffer_size(info.get_bit_count());
        let mut counter = 0;

        let step = match info.get_bit_count()
        {
            BitCount::BW => 1,
            BitCount::Color16Bit => 3,
            BitCount::Color256Bit => 3,
            BitCount::AllColors => 3,
            BitCount::UNKNOWN => 3,
        };

        for _ in 0..info.get_height()
        {
            for _ in 0..info.get_width()
            {
                let i = offset + counter;
                // if i >= bit_stream.len() { continue; }
                match info.get_bit_count()
                {
                    // TODO: implement parsing  
                    BitCount::BW => {
                        for shift in 0..6
                        {
                            let index: usize = if bit_stream[i] >> shift == 0 { 0 } else { 1 };
                            if counter < 9
                            {
                                print!("{} ", index);
                            }
                            array.push(PixelData::copy(&colors[index]));
                        }
                        if counter < 9
                        {
                            println!();
                        }                    },
                    BitCount::Color16Bit => {
                       array.push( PixelData::new(0, 0, 0));
                    },
                    BitCount::Color256Bit => {
                        array.push(PixelData::new(0, 0, 0));
                    },
                    BitCount::AllColors => {
                        array.push(PixelData::new(bit_stream[i], bit_stream[i + 1], bit_stream[i + 2]));
                    },
                    _ => array.push(PixelData::new(0, 0, 0))
                };
                counter += step;
            }
            println!("{}", counter);
            counter += row_buffer;
        }
        array
    }

    pub fn as_bytes(&self) -> Vec<u8>
    {
        let mut bytes = Vec::with_capacity(3);
        bytes.push(self.blue);
        bytes.push(self.green);
        bytes.push(self.red);
        bytes
    }
}

impl std::fmt::Display for PixelData
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(f, "Red: {}, Green: {}, Blue: {}",
            self.red,
            self.green,
            self.blue)
    }
}