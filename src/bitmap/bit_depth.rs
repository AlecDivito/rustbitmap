use super::image::BitMap;

#[derive(PartialEq, Clone, Copy)]
pub enum BitDepth {
    Color2Bit = 1,
    Color16Bit = 4,
    Color256Bit = 8,
    AllColors = 24,
    AllColorsAndShades = 32,
}

impl BitDepth {
    ///
    /// Get the number of bits or bytes to read when trying
    /// to read in a file with a specified bit depth value
    ///
    /// For the ColorNBit, we return the total length of how much each color
    /// takes up in """bits"""
    ///
    /// For AllColors*, we return the total length of how much each color takes
    /// up in """bytes"""
    ///
    pub fn get_step_counter(&self) -> u32 {
        match self {
            Self::Color2Bit => 1,
            Self::Color16Bit => 4,
            Self::Color256Bit => 8,
            Self::AllColors => 3,
            Self::AllColorsAndShades => 4,
        }
    }

    ///
    /// Get a suggested bit depth depending on the colors contained inside of
    /// a array of colors
    ///
    pub fn get_suggested_bit_depth(bitmap: &BitMap) -> BitDepth {
        let unique_colors = bitmap.get_all_unique_colors().len();
        let contains_transparents = bitmap.is_image_transparent();
        match unique_colors {
            0..=2 => BitDepth::Color2Bit,
            3..=16 => BitDepth::Color16Bit,
            17..=256 => BitDepth::Color256Bit,
            _ => match contains_transparents {
                true => BitDepth::AllColorsAndShades,
                false => BitDepth::AllColors,
            },
        }
    }
}

#[cfg(debug_assertions)]
impl std::fmt::Display for BitDepth {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Color2Bit => write!(f, "BitDepth: Color2Bit\n"),
            Self::Color16Bit => write!(f, "BitDepth: Color16Bit\n"),
            Self::Color256Bit => write!(f, "BitDepth: Color256Bit\n"),
            Self::AllColors => write!(f, "BitDepth: AllColors\n"),
            Self::AllColorsAndShades => write!(f, "BitDepth: AllColorsAndShades\n"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::BitDepth;
    use super::BitMap;
    use crate::bitmap::rgba::Rgba;

    #[test]
    fn get_correct_step_counter() {
        assert_eq!(BitDepth::Color2Bit.get_step_counter(), 1);
        assert_eq!(BitDepth::Color16Bit.get_step_counter(), 4);
        assert_eq!(BitDepth::Color256Bit.get_step_counter(), 8);
        assert_eq!(BitDepth::AllColors.get_step_counter(), 3);
        assert_eq!(BitDepth::AllColorsAndShades.get_step_counter(), 4);
    }

    #[test]
    fn get_correct_suggested_bit_depth() {
        let mut bitmap = BitMap::new(256, 1);
        assert!(BitDepth::Color2Bit == BitDepth::get_suggested_bit_depth(&bitmap));

        for x in 0..2 {
            bitmap
                .set_pixel(x, 0, Rgba::rgb(x as u8, x as u8, x as u8))
                .unwrap();
        }
        assert!(BitDepth::Color16Bit == BitDepth::get_suggested_bit_depth(&bitmap));

        for x in 0..15 {
            bitmap
                .set_pixel(x, 0, Rgba::rgb(x as u8, x as u8, x as u8))
                .unwrap();
        }
        assert!(BitDepth::Color16Bit == BitDepth::get_suggested_bit_depth(&bitmap));
        bitmap.set_pixel(15, 0, Rgba::rgb(16, 16, 16)).unwrap();
        assert!(BitDepth::Color256Bit == BitDepth::get_suggested_bit_depth(&bitmap));

        for x in 0..bitmap.get_width() {
            bitmap
                .set_pixel(x, 0, Rgba::rgb(x as u8, x as u8, x as u8))
                .unwrap();
        }
        assert!(BitDepth::Color256Bit == BitDepth::get_suggested_bit_depth(&bitmap));

        bitmap.set_pixel(15, 0, Rgba::rgba(16, 16, 16, 0)).unwrap();
        assert!(BitDepth::Color256Bit == BitDepth::get_suggested_bit_depth(&bitmap));
    }
}
