#[derive(PartialEq, Clone, Copy)]
pub enum BitDepth
{
    UNKNOWN = 0,
    BW = 1,
    Color16Bit = 4,
    Color256Bit = 8,
    AllColors = 24,
    AllColorsAndShades = 32,
}

impl BitDepth
{
    /**
     * Get the number of bits or bytes to read when trying
     * to read in a file with a specified bit depth value
     * 
     * @return {u32} amount of bits or bytes to skip
     */
    pub fn get_step_counter(&self) -> u32
    {
        match *self
        {
            BitDepth::UNKNOWN => 0,
            BitDepth::BW => 1,
            BitDepth::Color16Bit => 4,
            BitDepth::Color256Bit => 8,
            BitDepth::AllColors => 3,
            BitDepth::AllColorsAndShades => 4,
        }
    }
}