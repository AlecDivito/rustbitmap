pub fn byte_slice_to_u32(array: &[u8], index: &mut usize) -> u32 {
    let i: usize = *index;
    let a = [array[i], array[i + 1], array[i + 2], array[i + 3]];
    *index += 4;
    u32::from_le_bytes(a)
}

pub fn byte_slice_to_u16(array: &[u8], index: &mut usize) -> u16 {
    let i: usize = *index;
    let a = [array[i], array[i + 1]];
    *index += 2;
    u16::from_le_bytes(a)
}
