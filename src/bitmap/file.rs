use super::rgb_quad::RgbQuad;
use super::file_header::FileHeader;
use super::info_header::InfoHeader;
use super::pixel_data::PixelData;

pub struct File
{
    file: FileHeader,
    info: InfoHeader,
    colors: Vec<RgbQuad>,
    data: Vec<PixelData>
}

impl File
{
    pub fn read(filename: &str) -> File
    {
        let array = std::fs::read(filename).expect("Couldn't open file");
        let file = FileHeader::stream(&array);
        let info = InfoHeader::stream(&array);
        let colors = RgbQuad::stream(&array, &file, &info);
        let data = PixelData::stream(&array, &file, &info, &colors);
        File {
            file,
            info,
            colors,
            data
        }
    }

    pub fn save(&self, filename: &str) -> std::io::Result<()>
    {
        use std::io::Write;
        let mut bit_stream = unsafe { self.get_s_bytes() };
        let mut file = std::fs::File::create(filename)?;
        file.write_all(bit_stream.as_mut_slice())?;
        Ok(())
    }

    unsafe fn get_s_bytes(&self) -> Vec<u8>
    {
        let mut bytes = Vec::new();
        bytes.append(&mut self.file.as_bytes());
        bytes.append(&mut self.info.as_bytes());
        for c in &self.colors
        {
            bytes.append(&mut c.as_bytes());
        }
        let buffer_size = self.info.get_row_buffer_size(self.info.get_bit_count());
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

impl std::fmt::Display for File
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(f, "File Header: {}\nInfo Header:{}\n",
            self.file, self.info).unwrap();
        write!(f, "Colors ({}):\n", self.colors.len()).unwrap();
        for c in &self.colors
        {
            write!(f, "{}\n", c).unwrap();
        }
        write!(f, "Data ({}):\n", self.data.len()).unwrap();
        for i in 0..72//self.data.len()
        {
            write!(f, "{}:\t{}\n", i, self.data[i]).unwrap();
        }

        write!(f, "")
    }
}
