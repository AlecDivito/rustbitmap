struct BitMapFileHeader
{
    // Must always be set to 'BM' to declare that this is a .bmp-file
    bfType: [char; 2],
    // specifies the size of the file in bytes
    bfSize: u32,
    bfReserved1: u16,
    bfReserved2: u16,
    // Specifies the offset from the beginning of the file to the bitmap data.
    bfOffBits: u32
}

impl BitMapFileHeader
{
    /**
     * Create a new Bit Map File Header
     * 
     * @param {u32} size of file in bytes
     * @param {u32} offset from beginning of file to bitmap data
     */
    pub fn new(size: u32, offset: u32) -> BitMapFileHeader
    {
        BitMapFileHeader {
            bfType: ['B', 'M'],
            bfSize: size,
            bfOffBits: offset,
            bfReserved1: 0,
            bfReserved2: 0
        }
    }
}

struct BitMapInfoHeader
{
    // specifies the size of the BitMapFileHeader structure, in bytes
    biSize: u32,
    // specifies the width of the image, in pixels
    biWidth: u32,
    // specifies the height of the image, in pixels
    biHeight: u32,
    // specifies the number of planes of the target device, must be set to zero
    biPlanes: u16,
    // specifies the number of bits per pixel
    biBitCount: u16,
    // specifies the type of compression, usually set to zero (no compression)
    biCompression: u32,
    // specifies the size of the image data, in bytes. If there is no
    // compression, it is valid to set this member to zero
    biSizeImage: u32,
    // specifies the the horizontal pixels per meter on the designated target
    /// device, usually set to zero.
    biXPelsPerMeter: u32,
    // specifies the vertical pixels per meter on the designated target device,
    // usually set to zero
    biYPelsPerMeter: u32,
    // specifies the number of colors used in the bitmap, if set to zero the
    // number of colors is calculated using the biBitCount member.
    biClrUsed: u32,
    // specifies the number of color that are 'important' for the bitmap, if set
    // to zero, all colors are important
    biClrImportant: u32,
}

// struct Bmp
// {
//     bmfh: BitMapFileHeader,
//     bmih: str,
//     rgbquad: Colors[],

// }

fn main() {
    println!("Hello, world!");
}
