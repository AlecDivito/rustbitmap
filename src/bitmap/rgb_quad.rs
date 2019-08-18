use super::file_header::FileHeader;
use super::info_header::InfoHeader;
use super::rgba::Rgba;

pub struct RgbQuad {
    data: Vec<Rgba>,
}

impl RgbQuad {
    pub fn stream(bit_stream: &[u8], file: &FileHeader, info: &InfoHeader) -> RgbQuad {
        let mut data = Vec::new();
        let offset = file.get_byte_size() + info.get_info_size();

        for index in 0..info.get_colors_used() {
            let i: usize = ((index * 4) + offset) as usize;
            data.push(Rgba::bgra(
                bit_stream[i],
                bit_stream[i + 1],
                bit_stream[i + 2],
                bit_stream[i + 3],
            ));
        }

        RgbQuad { data }
    }

    pub fn get_bytes_size(&self) -> u32 {
        4 * self.data.len() as u32
    }

    pub fn empty() -> RgbQuad {
        RgbQuad { data: Vec::new() }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for rgb in &self.data {
            bytes.push(rgb.get_blue());
            bytes.push(rgb.get_green());
            bytes.push(rgb.get_red());
            bytes.push(rgb.get_alpha());
        }
        bytes
    }

    ///
    /// Clone the colors
    ///
    pub fn clone_colors(&self) -> Vec<Rgba> {
        self.data.clone()
    }

    ///
    /// Get the number of colors stored in RgbQuad
    ///
    pub fn len(&self) -> usize {
        self.data.len()
    }
}

#[cfg(debug_assertions)]
impl std::fmt::Display for RgbQuad {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for c in &self.data {
            write!(f, "{}\n", c).unwrap();
        }
        write!(f, "")
    }
}

#[cfg(test)]
mod test {
    use super::RgbQuad;

    #[test]
    fn rgb_quad_byte_size() {
        let q = RgbQuad::empty();
        assert_eq!(q.get_bytes_size(), 0);
    }

    #[test]
    fn rgb_quad_clone_colors() {
        let q = RgbQuad::empty();
        assert_eq!(q.clone_colors().len(), 0);
    }

    #[test]
    fn rgb_quad_colors_length() {
        let q = RgbQuad::empty();
        assert_eq!(q.len(), 0);
    }
}
