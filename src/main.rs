use std::fmt;
use std::fs;
use std::fs::File;
use std::io::Write;

struct BitMapFileHeader
{
    // Must always be set to 'BM' to declare that this is a .bmp-file
    bf_type: [char; 2],
    // specifies the size of the file in bytes
    bf_size: u32,
    bf_reserved1: u16,
    bf_reserved2: u16,
    // Specifies the offset from the beginning of the file to the bitmap data.
    bf_off_bits: u32
}

impl BitMapFileHeader
{
    /**
     * Create a new Bit Map File Header
     * 
     * @param {u32} size of file in bytes
     * @param {u32} offset from beginning of file to bitmap data
     * @return {BitMapFileHeader}
     */
    pub fn new(size: u32, offset: u32) -> BitMapFileHeader
    {
        BitMapFileHeader {
            bf_type: ['B', 'M'],
            bf_size: size,
            bf_reserved1: 0,
            bf_reserved2: 0,
            bf_off_bits: offset,
        }
    }

    /**
     * Create a new Bit Map File Header from stream of bytes
     * 
     * @param {&[u8; 14]} 14 byte long slice
     * @return {BitMapFileHeader}
     */
    pub fn stream(bit_stream: &[u8]) -> BitMapFileHeader
    {
        let bf_type = [bit_stream[0] as char, bit_stream[1] as char];
        let mut i = 2;
        BitMapFileHeader {
            bf_type,
            bf_size: byte_slice_to_u32(bit_stream, &mut i),
            bf_reserved1: byte_slice_to_u16(bit_stream, &mut i),
            bf_reserved2: byte_slice_to_u16(bit_stream, &mut i),
            bf_off_bits: byte_slice_to_u32(bit_stream, &mut i)
        }
    }

    pub fn as_bytes(&self) -> Vec<u8>
    {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&[self.bf_type[0] as u8, self.bf_type[1] as u8]);
        bytes.extend_from_slice(&byte_slice_from_u32(self.bf_size));
        bytes.extend_from_slice(&byte_slice_from_u16(self.bf_reserved1));
        bytes.extend_from_slice(&byte_slice_from_u16(self.bf_reserved2));
        bytes.extend_from_slice(&byte_slice_from_u32(self.bf_off_bits));
        bytes
    }

    pub fn get_file_header_byte_size(&self) -> usize
    {
        14
    }

    pub fn get_size(&self) -> u32
    {
        self.bf_size
    }

    pub fn get_off_bits(&self) -> u32
    {
        self.bf_off_bits
    }

}

impl fmt::Display for BitMapFileHeader
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "Type: {}{}, Size: {}, res1: {}, res2: {}, offset: {}",
            self.bf_type[0], self.bf_type[1], self.bf_size, self.bf_reserved1,
            self.bf_reserved2, self.bf_off_bits)
    }
}


struct BitMapInfoHeader
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
    bi_bit_count: u16,
    // specifies the type of compression, usually set to zero (no compression)
    bi_compression: u32,
    // specifies the size of the image data, in bytes. If there is no
    // compression, it is valid to set this member to zero
    bi_size_image: u32,
    // specifies the the horizontal pixels per meter on the designated target
    /// device, usually set to zero.
    bi_x_pels_per_meter: u32,
    // specifies the vertical pixels per meter on the designated target device,
    // usually set to zero
    bi_y_pels_per_meter: u32,
    // specifies the number of colors used in the bitmap, if set to zero the
    // number of colors is calculated using the biBitCount member.
    bi_clr_used: u32,
    // specifies the number of color that are 'important' for the bitmap, if set
    // to zero, all colors are important
    bi_clr_important: u32,
}

impl BitMapInfoHeader
{
    /**
     * Create a new Bit Map Info Header
     * 
     * @param {u32} size of BitMapFileHeader in bytes
     * @param {u32} width of image in pixels
     * @param {u32} height of image in pixels
     * @param {u16} number of bits per pixel
     * @return {BitMapInfoHeader}
     */
    pub fn new(bi_size: u32, bi_width: u32, bi_height: u32, bi_bit_count: u16) -> BitMapInfoHeader
    {
        BitMapInfoHeader {
            bi_size,
            bi_width,
            bi_height,
            bi_bit_count,
            bi_planes: 0,
            bi_compression: 0,
            bi_size_image: 0,
            bi_x_pels_per_meter: 0,
            bi_y_pels_per_meter: 0,
            bi_clr_used: 0,
            bi_clr_important: 0,
        }
    }

    pub fn stream(bit_stream: &[u8]) -> BitMapInfoHeader
    {
        // starts at 14
        let mut i: usize = 14;
        BitMapInfoHeader {
            bi_size: byte_slice_to_u32(bit_stream, & mut i),
            bi_width: byte_slice_to_u32(bit_stream, & mut i),
            bi_height: byte_slice_to_u32(bit_stream, & mut i),
            bi_planes: byte_slice_to_u16(bit_stream, & mut i),
            bi_bit_count: byte_slice_to_u16(bit_stream, & mut i),
            bi_compression: byte_slice_to_u32(bit_stream, & mut i),
            bi_size_image: byte_slice_to_u32(bit_stream, & mut i),
            bi_x_pels_per_meter: byte_slice_to_u32(bit_stream, & mut i),
            bi_y_pels_per_meter: byte_slice_to_u32(bit_stream, & mut i),
            bi_clr_used: byte_slice_to_u32(bit_stream, & mut i),
            bi_clr_important: byte_slice_to_u32(bit_stream, & mut i),
        }
    }

    pub fn as_bytes(&self) -> Vec<u8>
    {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&byte_slice_from_u32(self.bi_size));
        bytes.extend_from_slice(&byte_slice_from_u32(self.bi_width));
        bytes.extend_from_slice(&byte_slice_from_u32(self.bi_height));
        bytes.extend_from_slice(&byte_slice_from_u16(self.bi_planes));
        bytes.extend_from_slice(&byte_slice_from_u16(self.bi_bit_count));
        bytes.extend_from_slice(&byte_slice_from_u32(self.bi_compression));
        bytes.extend_from_slice(&byte_slice_from_u32(self.bi_size_image));
        bytes.extend_from_slice(&byte_slice_from_u32(self.bi_x_pels_per_meter));
        bytes.extend_from_slice(&byte_slice_from_u32(self.bi_y_pels_per_meter));
        bytes.extend_from_slice(&byte_slice_from_u32(self.bi_clr_used));
        bytes.extend_from_slice(&byte_slice_from_u32(self.bi_clr_important));
        bytes
    }

    pub fn get_row_buffer_size(&self) -> usize
    {
        match (self.bi_width * 3) % 4 {
            1 => 3,
            2 => 2,
            3 => 1,
            _ => 0
        }
    }

    pub fn get_info_size(&self) -> u32
    {
        self.bi_size
    }

    pub fn get_bit_count(&self) -> u16
    {
        self.bi_bit_count
    }

    pub fn get_width(&self) -> u32
    {
        self.bi_width
    }

    pub fn get_height(&self) -> u32
    {
        self.bi_height
    }
}

fn byte_slice_from_u32(value: u32) -> [u8; 4]
{
    unsafe { std::mem::transmute::<u32, [u8; 4]>(value.to_le()) }
}

fn byte_slice_from_u16(value: u16) -> [u8; 2]
{
    unsafe { std::mem::transmute::<u16, [u8; 2]>(value.to_le()) }
}

fn byte_slice_to_u32(array: &[u8], index: & mut usize) -> u32
{
    let i: usize = *index;
    let a = [array[i], array[i + 1], array[i + 2], array[i + 3]];
    *index += 4;
    let sum = unsafe {
        std::mem::transmute::<[u8; 4], u32>(a).to_le()
    };
    sum
}

fn byte_slice_to_u16(array: &[u8], index: & mut usize) -> u16
{
    let i: usize = *index;
    let a = [array[i], array[i + 1]];
    *index += 2;
    let sum = unsafe {
        std::mem::transmute::<[u8; 2], u16>(a).to_le()
    };
    sum
}

unsafe fn as_u8_slice<T: Sized>(p: &T) -> &[u8]
{
    ::std::slice::from_raw_parts(
        (p as * const T) as * const u8, 
        ::std::mem::size_of::<T>()
    )
}

impl fmt::Display for BitMapInfoHeader
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "
        Header Size: {}, Width: {}, Height: {}, Bit Count: {}
        Planes: {}, compression: {}, image size: {}
        x pxls per meter: {}, y pxls per meter: {}
        clr_used: {} clr_important: {}",
            self.bi_size,
            self.bi_width,
            self.bi_height,
            self.bi_bit_count,
            self.bi_planes,
            self.bi_compression,
            self.bi_size_image,
            self.bi_x_pels_per_meter,
            self.bi_y_pels_per_meter,
            self.bi_clr_used,
            self.bi_clr_important)
    }
}

struct RgbQuad
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
        file: & BitMapFileHeader,
        info: & BitMapInfoHeader
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
}

impl fmt::Display for RgbQuad
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "Red: {}, Green: {}, Blue: {}, Reserved: {}",
            self.red,
            self.green,
            self.blue,
            self.reserved)
    }
}

struct PixelData
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

    pub fn stream(
        bit_stream: &[u8],
        file: &BitMapFileHeader,
        info: &BitMapInfoHeader
    ) -> Vec<PixelData>
    {
        let mut array: Vec<PixelData> = Vec::new();
        let offset = file.get_off_bits() as usize;
        let row_buffer = info.get_row_buffer_size();
        let mut counter = 0;

        for _ in 0..info.get_height()
        {
            for _ in 0..info.get_width()
            {
                let i = offset + counter;
                let data = match info.get_bit_count()
                {
                    // TODO: implement parsing  
                    1 => PixelData::new(0, 0, 0),
                    4 => PixelData::new(0, 0, 0),
                    8 => PixelData::new(0, 0, 0),
                    24 => PixelData::new(bit_stream[i], bit_stream[i + 1], bit_stream[i + 2]),
                    _ => PixelData::new(0, 0, 0),
                };
                array.push(data);
                counter += 3;
            }
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

impl fmt::Display for PixelData
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "Red: {}, Green: {}, Blue: {}",
            self.red,
            self.green,
            self.blue)
    }
}

struct Bmp
{
    file: BitMapFileHeader,
    info: BitMapInfoHeader,
    colors: Vec<RgbQuad>,
    data: Vec<PixelData>
}

impl Bmp
{
    pub fn read(filename: &str) -> Bmp
    {
        let array = fs::read(filename).expect("Couldn't open file");
        let file = BitMapFileHeader::stream(&array);
        println!("{}", file);
        let info = BitMapInfoHeader::stream(&array);
        println!("{}", info);
        let colors = RgbQuad::stream(&array, &file, &info);
        let data = PixelData::stream(&array, &file, &info);
        Bmp {
            file,
            info,
            colors,
            data
        }
    }

    pub fn save(&self, filename: &str) -> std::io::Result<()>
    {
        let mut bit_stream = unsafe { self.get_bit_map_as_bytes() };
        let mut file = File::create(filename)?;
        file.write_all(bit_stream.as_mut_slice())?;
        Ok(())
    }

    unsafe fn get_bit_map_as_bytes(&self) -> Vec<u8>
    {
        let mut bytes = Vec::new();
        bytes.append(&mut self.file.as_bytes());
        bytes.append(&mut self.info.as_bytes());
        for c in &self.colors
        {
            bytes.append(&mut c.as_bytes());
        }
        let buffer_size = self.info.get_row_buffer_size();
        for y in 0..self.info.get_height()
        {
            for x in 0..self.info.get_width()
            {
                let index = (x + (y * self.info.get_height())) as usize;
                let pixel = &self.data[index];
                bytes.append(&mut pixel.as_bytes());
            }
            for _ in 0..buffer_size
            {
                bytes.push(u8::min_value());
            }
        }
        bytes
    }
}

impl fmt::Display for Bmp
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "File Header: {}\nInfo Header:{}\n",
            self.file, self.info).unwrap();
        write!(f, "Colors ({}):\n", self.colors.len()).unwrap();
        for c in &self.colors
        {
            write!(f, "{}\n", c).unwrap();
        }
        write!(f, "Data ({}):\n", self.data.len()).unwrap();
        for i in 0..self.data.len()
        {
            write!(f, "{}:\t{}\n", i, self.data[i]).unwrap();
        }

        write!(f, "")
    }
}

fn main() {
    let b1 = Bmp::read("/home/divitoa/Program/RUST/bmp-writer/pixel_data_test.bmp");
    // println!("{}", b1);
    b1.save("/home/divitoa/Program/RUST/bmp-writer/save.bmp");
    let b2 = Bmp::read("/home/divitoa/Program/RUST/bmp-writer/save.bmp");
    // println!("{}", b2);
    // let b2 = Bmp::read("/home/divitoa/Program/RUST/bmp-writer/bw_example.bmp");
}
