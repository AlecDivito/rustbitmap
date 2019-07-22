use super::file_header::FileHeader;
use super::info_header::InfoHeader;

pub struct Rgb
{
    // blue part of color
    blue: u8,
    // green part of color
    green: u8,
    // red part of color
    red: u8,
    // must always be set to zero
    alpha: u8,
}

impl Rgb
{
    /**
     * Create a new rgb color
     * 
     * @param {u8} blue
     * @param {u8} green
     * @param {u8} red
     * @return {RgbQuad}
     */
    pub fn new(blue: u8, green: u8, red: u8) -> Rgb
    {
        Rgb {
            blue,
            green,
            red,
            alpha: 0
        }
    }

    pub fn get_blue(&self) -> u8
    {
        self.blue
    }

    pub fn get_green(&self) -> u8
    {
        self.green
    }

    pub fn get_red(&self) -> u8
    {
        self.red
    }

    pub fn get_alpha(&self) -> u8
    {
        self.alpha
    }
}

impl std::fmt::Display for Rgb
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(f, "Red: {}, Green: {}, Blue: {}, Alpha: {}",
            self.red,
            self.green,
            self.blue,
            self.alpha)
    }
}

pub struct RgbQuad
{
    data: Vec<Rgb>,
}

impl RgbQuad
{


    pub fn stream(
        bit_stream: &[u8],
        file: & FileHeader,
        info: & InfoHeader
    ) -> RgbQuad
    {
        let mut data = Vec::new();
        let offset = file.get_byte_size() + info.get_info_size();

        for index in 0..info.get_colors_used()
        {
            let i: usize = ((index * 4) + offset) as usize;
            data.push(Rgb::new(bit_stream[i], bit_stream[i+1], bit_stream[i+2]));
        }

        RgbQuad { data }
    }

    pub fn get_byte_size() -> u32
    {
        4
    }

    pub fn bw() -> RgbQuad
    {
        let mut data = Vec::with_capacity(2);
        data.push(Rgb::new(0, 0, 0));
        data.push(Rgb::new(255, 255, 255));
        RgbQuad { data }
    }

    pub fn empty() -> RgbQuad
    {
        RgbQuad { data: Vec::new() }
    }

    pub fn as_bytes(&self) -> Vec<u8>
    {
        let mut bytes = Vec::new();
        for rgb in &self.data
        {
            bytes.push(rgb.get_blue());
            bytes.push(rgb.get_green());
            bytes.push(rgb.get_red());
            bytes.push(rgb.get_alpha());
        }
        bytes
    }

    pub fn len(&self) -> usize
    {
        self.data.len()
    }
}

impl std::fmt::Display for RgbQuad
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        for c in &self.data
        {
            write!(f, "{}\n", c).unwrap();
        }
        write!(f, "")
    }
}
