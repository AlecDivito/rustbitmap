#[derive(PartialEq, Clone, Copy)]
pub enum BitDepth {
    BW = 1,
    Color16Bit = 4,
    Color256Bit = 8,
    AllColors = 24,
    AllColorsAndShades = 32,
}

impl BitDepth {
    /**
     * Get the number of bits or bytes to read when trying
     * to read in a file with a specified bit depth value
     *
     * @return {u32} amount of bits or bytes to skip
     */
    pub fn get_step_counter(&self) -> u32 {
        match self {
            Self::BW => 1,
            Self::Color16Bit => 4,
            Self::Color256Bit => 8,
            Self::AllColors => 3,
            Self::AllColorsAndShades => 4,
        }
    }
}
