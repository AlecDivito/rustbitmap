use super::rgba::Rgba;

#[derive(PartialEq, Clone, Copy)]
pub enum BitDepth {
    BW = 1,
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
    /// @return {u32} amount of bits or bytes to skip
    ///
    pub fn get_step_counter(&self) -> u32 {
        match self {
            Self::BW => 1,
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
    pub fn get_suggested_bit_depth(colors: &[Rgba]) -> BitDepth {
        let mut contains_transparents = false;
        let mut unique_colors = Vec::new();
        for c in colors {
            if c.is_transparent() {
                contains_transparents = true;
            }
            if !unique_colors.contains(c) {
                unique_colors.push(c.clone());
            }
            // TODO: Magic number???
            if unique_colors.len() > 256 {
                break;
            }
        }
        match unique_colors.len() {
            0..=2 => BitDepth::BW,
            3..=16 => BitDepth::Color16Bit,
            17..=256 => BitDepth::Color256Bit,
            _ => match contains_transparents {
                true => BitDepth::AllColorsAndShades,
                false => BitDepth::AllColors
            }
        }
    }
}

#[cfg(debug_assertions)]
impl std::fmt::Display for BitDepth {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::BW => write!(f, "BitDepth: BW\n"),
            Self::Color16Bit => write!(f, "BitDepth: Color16Bit\n"),
            Self::Color256Bit => write!(f, "BitDepth: Color256Bit\n"),
            Self::AllColors => write!(f, "BitDepth: AllColors\n"),
            Self::AllColorsAndShades => write!(f, "BitDepth: AllColorsAndShades\n"),
        }
    }
}