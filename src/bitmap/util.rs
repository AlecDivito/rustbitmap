pub fn byte_slice_from_u32(value: u32) -> [u8; 4] {
    unsafe { std::mem::transmute::<u32, [u8; 4]>(value.to_le()) }
}

pub fn byte_slice_from_u16(value: u16) -> [u8; 2] {
    unsafe { std::mem::transmute::<u16, [u8; 2]>(value.to_le()) }
}

pub fn byte_slice_to_u32(array: &[u8], index: &mut usize) -> u32 {
    let i: usize = *index;
    let a = [array[i], array[i + 1], array[i + 2], array[i + 3]];
    *index += 4;
    let sum = unsafe { std::mem::transmute::<[u8; 4], u32>(a).to_le() };
    sum
}

pub fn byte_slice_to_u16(array: &[u8], index: &mut usize) -> u16 {
    let i: usize = *index;
    let a = [array[i], array[i + 1]];
    *index += 2;
    let sum = unsafe { std::mem::transmute::<[u8; 2], u16>(a).to_le() };
    sum
}
