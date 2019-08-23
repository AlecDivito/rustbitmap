use super::bit_depth::BitDepth;
use super::image::BitMap;
use super::rgba::Rgba;

pub struct RgbQuad {
    data: Vec<Rgba>,
}

///
/// Used for constants
///
impl RgbQuad {
    pub fn single_rgb_quad_size() -> usize {
        4
    }
}

///
/// Core implementation
///
impl RgbQuad {
    ///
    /// From a from_slice of bytes, read in a list of colors used to render the
    /// bitmap image
    ///
    pub fn from_slice(bit_stream: &[u8]) -> Result<RgbQuad, &'static str> {
        if bit_stream.len() == 0 {
            return Ok(RgbQuad::empty());
        }
        let mut data = Vec::new();
        if bit_stream.len() % 4 != 0 {
            return Err("Not enough data to parse Rgb quad colors");
        }
        let colors_used = bit_stream.len() / 4;
        for index in 0..colors_used {
            let i: usize = index * 4;
            data.push(Rgba::bgra(
                bit_stream[i],
                bit_stream[i + 1],
                bit_stream[i + 2],
                bit_stream[i + 3],
            ));
        }

        Ok(RgbQuad { data })
    }

    ///
    /// From a bitmap, create a list of unique colors that are used to create
    /// the bitmap
    ///
    pub fn from(bitmap: &BitMap, bit_depth: BitDepth) -> RgbQuad {
        match bit_depth {
            BitDepth::Color2Bit | BitDepth::Color16Bit | BitDepth::Color256Bit => RgbQuad {
                data: bitmap.get_all_unique_colors(),
            },
            _ => RgbQuad::empty(),
        }
    }

    ///
    /// Create a empty rgb quad
    ///
    fn empty() -> RgbQuad {
        RgbQuad { data: Vec::new() }
    }

    pub fn get_bytes_size(&self) -> u32 {
        4 * self.data.len() as u32
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
    use super::BitDepth;
    use super::BitMap;
    use super::RgbQuad;
    use super::Rgba;

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

    #[test]
    fn crating_a_rgb_quad_from_bitmap() {
        let mut b = BitMap::new(2, 2);

        let quad = RgbQuad::from(&b, BitDepth::Color2Bit);
        assert_eq!(quad.as_bytes().len(), quad.get_bytes_size() as usize);

        b.set_pixel(0, 0, Rgba::rgb(255, 0, 0)).unwrap();
        b.set_pixel(1, 0, Rgba::rgb(0, 0, 255)).unwrap();
        b.set_pixel(0, 1, Rgba::black()).unwrap();
        let quad = RgbQuad::from(&b, BitDepth::Color16Bit);
        assert_eq!(quad.as_bytes().len(), quad.get_bytes_size() as usize);

        b.resize_by(20.0).unwrap();
        let quad = RgbQuad::from(&b, BitDepth::AllColors);
        assert_eq!(quad.as_bytes().len(), quad.get_bytes_size() as usize);

        let quad = RgbQuad::from(&b, BitDepth::AllColorsAndShades);
        assert_eq!(quad.as_bytes().len(), quad.get_bytes_size() as usize);

        b.color_to_gray();
        let quad = RgbQuad::from(&b, BitDepth::Color256Bit);
        assert_eq!(quad.as_bytes().len(), quad.get_bytes_size() as usize);
    }
}
