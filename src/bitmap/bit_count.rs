#[derive(PartialEq, Clone, Copy)]
pub enum BitCount
{
    UNKNOWN = 0,
    BW = 1,
    Color16Bit = 4,
    Color256Bit = 8,
    AllColors = 24,
}
