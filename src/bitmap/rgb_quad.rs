use super::file_header::FileHeader;
use super::info_header::InfoHeader;

pub struct RgbQuad
{
    // blue part of color
    blue: u8,
    // green part of color
    green: u8,
    // red part of color
    red: u8,
    // must always be set to zero
    reserved: u8,
}

impl RgbQuad
{
    /**
     * Create a new rgb color
     * 
     * @param {u8} blue
     * @param {u8} green
     * @param {u8} red
     * @return {RgbQuad}
     */
    pub fn new(blue: u8, green: u8, red: u8) -> RgbQuad
    {
        RgbQuad {
            blue,
            green,
            red,
            reserved: 0
        }
    }

    pub fn stream(
        bit_stream: &[u8],
        file: & FileHeader,
        info: & InfoHeader
    ) -> Vec<RgbQuad>
    {
        let mut array: Vec<RgbQuad> = Vec::new();
        let mut i: usize = file.get_file_header_byte_size() + info.get_info_size() as usize;

        while i < file.get_off_bits() as usize
        {
            array.push(RgbQuad::new(bit_stream[i], bit_stream[i+1], bit_stream[i+2]));
            i += 4;
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

    pub fn get_red(&self) -> u8
    {
        self.red
    }

    pub fn get_green(&self) -> u8
    {
        self.green
    }

    pub fn get_blue(&self) -> u8
    {
        self.blue
    }

}

impl std::fmt::Display for RgbQuad
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(f, "Red: {}, Green: {}, Blue: {}, Reserved: {}",
            self.red,
            self.green,
            self.blue,
            self.reserved)
    }
}