use super::bit_depth::BitDepth;
use super::file_data::FileData;
use super::file_header::FileHeader;
use super::image::BitMap;
use super::info_header::InfoHeader;
use super::rgb_quad::RgbQuad;

pub struct File {
    file: FileHeader,
    info: InfoHeader,
    colors: RgbQuad,
    data: FileData,
}

impl File {
    ///
    /// Read in Bitmap file from file
    /// 
    pub fn read(filename: &str) -> Option<File> {
        let array = std::fs::read(filename).expect("Couldn't open file");
        let file = FileHeader::stream(&array);
        let info = InfoHeader::stream(&array);
        let colors = RgbQuad::stream(&array, &file, &info);
        let data = FileData::stream(&array, &file, &info, &colors)?;
        Some(File {
            file,
            info,
            colors,
            data,
        })
    }

    ///
    /// Create a bitmap file from a bitmap image
    /// 
    pub fn create(bitmap: &BitMap) -> File {
        // TODO: Figure out if we can simplify this
        let bit_depth = BitDepth::AllColors;
        let data = FileData::from_bitmap(bitmap, bit_depth);
        let colors = RgbQuad::empty();
        let info = InfoHeader::from_bitmap(bitmap, bit_depth);
        let file = FileHeader::new(
            data.get_bytes_size(),
            colors.get_bytes_size(),
            info.get_info_size(),
        );
        File {
            file,
            info,
            colors,
            data,
        }
    }

    // pub fn save_as(&mut self, filename: &str, simplify: bool)
    // {

    // }

    pub unsafe fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.append(&mut self.file.as_bytes());
        bytes.append(&mut self.info.as_bytes());
        bytes.append(&mut self.colors.as_bytes());
        bytes.append(&mut self.data.as_bytes());
        bytes
    }

    // fn convert_to(&mut self, bit_depth: BitDepth) -> Result<(), &'static str>
    // {
    //     // we need to change our data structures to mimic the chosen BitDepth
    //     // 1. Check if we need to add colors
    //     self.colors = match bit_depth {
    //         BitDepth::BW => RgbQuad::bw(),
    //         _ => RgbQuad::empty(),
    //     };
    //     self.data = match (self.info.get_bit_depth(), bit_depth) {
    //         (BitDepth::AllColors, BitDepth::BW) => self.data.convert_pixels_to_bw(),
    //         // BW => ALLCOLORS
    //         //
    //         _ => return Err("Converting not supported with types choosen"),
    //     };
    //     self.info.set_colors_used(self.colors.len() as u32);
    //     self.file.set_offset(&self.info);
    //     self.info.set_bit_depth(bit_depth);
    //     self.info.set_image_size(self.data.get_bytes_size());
    //     let size = self.file.get_off_bits() + self.data.get_bytes_size();
    //     self.file.set_file_size(size);
    //     Ok(())
    // }

    pub fn get_info_header(&self) -> &InfoHeader {
        &self.info
    }

    pub fn get_pixels(&self) -> &FileData {
        &self.data
    }
}

#[cfg(debug_assertions)]
impl std::fmt::Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "File Header: {}\nInfo Header:{}\nColors ({})\nBytes:({})",
            self.file,
            self.info,
            self.colors.len(),
            self.data.len()
        )
    }
}

#[cfg(test)]
mod test {
    use super::File;
    use super::BitMap;

    #[test]
    fn get_number_of_bytes_after_creating_file_from_bitmap() {
        let b = BitMap::new(2, 2);
        let file = File::create(&b);
        let pixel_bytes = 16;
        let info_header_bytes = 40;
        let color_bytes = 0;
        let header_bytes = 14;
        let bytes = pixel_bytes + info_header_bytes + color_bytes + header_bytes;
        assert_eq!(unsafe { file.to_bytes().len() } , bytes);
    }
}