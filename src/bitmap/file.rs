use super::bit_depth::BitDepth;
use super::file_data::FileData;
use super::file_header::FileHeader;
use super::image::BitMap;
use super::info_header::InfoHeader;
use super::rgb_quad::RgbQuad;
use super::rgba::Rgba;

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
    pub fn read(filename: &str) -> Result<File, String> {
        use std::io::ErrorKind;
        let byte_array = match std::fs::read(filename) {
            Err(why) => {
                return Err(String::from(match why.kind() {
                    ErrorKind::NotFound => format!("File {} was not found!", filename),
                    ErrorKind::PermissionDenied => format!(
                        "Could not read file {} because program lacks privilege!",
                        filename
                    ),
                    _ => format!("Couldn't read file {}!", filename),
                }))
            }
            Ok(bytes) => bytes,
        };
        if byte_array.len() == 0 {
            format!("File {} is empty!", filename);
        }

        let mut size_clamp = FileHeader::estimated_byte_size();

        // file header
        let file_header_bytes = if byte_array.len() > size_clamp {
            &byte_array[FileHeader::from_slice_range()]
        } else {
            return Err(String::from("Not enough data to parse bitmap header."));
        };
        let file = match FileHeader::from_slice(&file_header_bytes) {
            Err(why) => return Err(String::from(why)),
            Ok(f) => f,
        };

        // info header
        size_clamp = size_clamp + InfoHeader::estimated_byte_size();
        let file_info_bytes = if byte_array.len() > size_clamp {
            &byte_array[InfoHeader::from_slice_range()]
        } else {
            return Err(String::from("Not enough data to parse bitmap info header."));
        };
        let info = match InfoHeader::from_slice(&file_info_bytes) {
            Err(why) => return Err(String::from(why)),
            Ok(i) => i,
        };

        // colors
        let offset = (file.get_byte_size() + info.get_byte_size()) as usize;
        let color_byte_size = info.get_colors_used() as usize * RgbQuad::single_rgb_quad_size();
        size_clamp = size_clamp + color_byte_size;
        let color_bytes = if byte_array.len() > size_clamp {
            &byte_array[offset..size_clamp]
        } else {
            return Err(String::from("Not enough data to parse bitmap colors."));
        };
        let colors = match RgbQuad::from_slice(&color_bytes) {
            Err(why) => return Err(String::from(why)),
            Ok(i) => i,
        };

        // data
        let pixel_bytes = &byte_array[file.get_off_bits() as usize..byte_array.len()];
        let data = match FileData::from_slice(&pixel_bytes, &info, &colors) {
            Some(d) => d,
            None => return Err(String::from("Couldn't read in pixels from file")),
        };
        Ok(File {
            file,
            info,
            colors,
            data,
        })
    }

    ///
    /// Create a bitmap file from a bitmap image
    ///
    pub fn create(bitmap: &BitMap, bit_depth: BitDepth) -> File {
        // TODO: Figure out if we can simplify this
        let data = FileData::from_bitmap(bitmap, bit_depth);
        let colors = RgbQuad::from(bitmap, bit_depth);
        let info = InfoHeader::from(bitmap, bit_depth);
        let file = FileHeader::new(
            data.get_bytes_size(),
            colors.get_bytes_size(),
            info.get_byte_size(),
        );
        File {
            file,
            info,
            colors,
            data,
        }
    }

    pub fn calculate_file_size(&self) -> u32 {
        self.file.get_byte_size()
            + self.info.get_byte_size()
            + self.colors.get_bytes_size()
            + self.data.get_bytes_size()
    }

    pub unsafe fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.append(&mut self.file.as_bytes());
        bytes.append(&mut self.info.as_bytes());
        bytes.append(&mut self.colors.as_bytes());
        bytes.append(&mut self.data.as_bytes());
        bytes
    }

    pub fn get_width(&self) -> u32 {
        self.info.get_width()
    }

    pub fn get_height(&self) -> u32 {
        self.info.get_height()
    }

    pub fn get_bitmap_as_pixels(&self) -> Vec<Rgba> {
        self.data.as_rgba()
    }
}

#[cfg(debug_assertions)]
impl std::fmt::Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "File Header: {}\nInfo Header:{}\nColors ({})\n{}Bytes:({})\n",
            self.file,
            self.info,
            self.colors.len(),
            self.colors,
            self.data.len(),
            // self.data
        )
    }
}

#[cfg(test)]
mod test {
    use super::BitDepth;
    use super::BitMap;
    use super::File;
    use super::Rgba;

    #[test]
    fn check_files_height_and_width() {
        let b = BitMap::new(10, 10);
        let f = File::create(&b, BitDepth::AllColors);
        assert_eq!(f.get_width(), 10);
        assert_eq!(f.get_height(), 10);
        let colors = f.get_bitmap_as_pixels();
        for color in &colors {
            assert!(color == &Rgba::white());
        }
    }

    #[test]
    fn number_of_bytes_for_all_bit_depth() {
        let b = BitMap::new(2, 2);

        let file = File::create(&b, BitDepth::Color2Bit);
        assert_eq!(
            unsafe { file.to_bytes().len() },
            file.calculate_file_size() as usize
        );

        let file = File::create(&b, BitDepth::Color16Bit);
        assert_eq!(
            unsafe { file.to_bytes().len() },
            file.calculate_file_size() as usize
        );

        let file = File::create(&b, BitDepth::Color256Bit);
        assert_eq!(
            unsafe { file.to_bytes().len() },
            file.calculate_file_size() as usize
        );

        let file = File::create(&b, BitDepth::AllColors);
        assert_eq!(
            unsafe { file.to_bytes().len() },
            file.calculate_file_size() as usize
        );

        let file = File::create(&b, BitDepth::AllColorsAndShades);
        assert_eq!(
            unsafe { file.to_bytes().len() },
            file.calculate_file_size() as usize
        );
    }
}
