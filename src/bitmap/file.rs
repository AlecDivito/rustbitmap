use super::rgb_quad::RgbQuad;
use super::file_header::FileHeader;
use super::info_header::InfoHeader;
use super::bit_count::BitCount;
use super::file_data::FileData;

pub struct File
{
    file: FileHeader,
    info: InfoHeader,
    colors: RgbQuad,
    data: FileData
}

impl File
{
    pub fn read(filename: &str) -> Option<File>
    {
        let array = std::fs::read(filename).expect("Couldn't open file");
        let file = FileHeader::stream(&array);
        let info = InfoHeader::stream(&array);
        let colors = RgbQuad::stream(&array, &file, &info);
        let data = FileData::stream(&array, &file, &info, &colors)?;
        Some(File {
            file,
            info,
            colors,
            data
        })
    }

    pub fn save(&mut self, filename: &str, bit_count: BitCount) -> std::io::Result<()>
    {
        use std::io::Write;
        if self.info.get_bit_count() != bit_count
        {
            self.convert_to(bit_count).unwrap();
        }
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
        bytes.append(&mut self.colors.as_bytes());
        bytes.append(&mut self.data.as_bytes());
        bytes
    }

    fn convert_to(&mut self, bit_count: BitCount) -> Result<(), &'static str>
    {
        // we need to change our data structures to mimic the chosen BitCount
        // 1. Check if we need to add colors
        self.colors = match bit_count {
            BitCount::BW => RgbQuad::bw(),
            _ => RgbQuad::empty(),
        };
        self.data = match (self.info.get_bit_count(), bit_count) {
            (BitCount::AllColors, BitCount::BW) => self.data.convert_pixels_to_bw(),
            // BW => ALLCOLORS
            // 
            _ => return Err("Converting not supported with types choosen"),
        };
        self.info.set_colors_used(self.colors.len() as u32);
        self.file.set_offset(&self.info);
        self.info.set_bit_count(bit_count);
        self.info.set_image_size(self.data.get_bytes_size());
        let size = self.file.get_off_bits() + self.data.get_bytes_size();
        self.file.set_file_size(size);
        Ok(())
    }
}

impl std::fmt::Display for File
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(f, "File Header: {}\nInfo Header:{}\nColors ({}):\n{}Data ({}):\n{}",
            self.file, self.info, self.colors.len(), self.colors,
            self.data.len(), self.data)
    }
}
