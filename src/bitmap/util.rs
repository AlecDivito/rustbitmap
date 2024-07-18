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

pub fn calculate_crop(max_x: u32, max_y: u32, x: u32, y: u32, crop_factor: f32) -> (u32, u32, u32, u32) {
    let width = max_x;
    let height = max_y;

    // number of pixels to move
    let x_distance = (width as f32 * crop_factor / 2.0).round() as u32;
    let y_distance = (height as f32 * crop_factor / 2.0).round() as u32;

    enum Op {
        Max,
        Min
    }

    let compute_correct_crop = |op: Op, coord: i32, distance: i32| {
        match op {
            Op::Max => {
                let calc = coord + distance;
                if calc > width as i32 {
                    (width  as i32, calc - width as i32)
                } else {
                    (calc, 0)
                }
            },
            Op::Min => {
                let calc = coord - distance;
                if calc < 0 {
                    (0, calc.abs())
                } else {
                    (calc, 0)
                }
            },
        }
    };

    
    let (mut from_x, from_x_remaining) = compute_correct_crop(Op::Min, x as i32, x_distance as i32);
    let (mut from_y, from_y_remaining) = compute_correct_crop(Op::Min, y as i32, y_distance as i32);
    let (mut to_x, to_x_remaining)  = compute_correct_crop(Op::Max, x as i32, x_distance as i32);
    let (mut to_y, to_y_remaining)  = compute_correct_crop(Op::Max, y as i32, y_distance as i32);

    from_x -= to_x_remaining;
    from_y -= to_y_remaining;
    to_x += from_x_remaining;
    to_y += from_y_remaining;

    (from_x as u32, from_y as u32, to_x as u32, to_y as u32)
}

#[cfg(test)]
mod test {
    use crate::bitmap::util::calculate_crop;


    #[test]
    pub fn calculate_crop_from_0_0() {
        let (x1, y1, x2, y2) = calculate_crop(100, 100, 0, 0, 0.5);
        assert_eq!(x1, 0);
        assert_eq!(y1, 0);
        assert_eq!(x2, 50);
        assert_eq!(y2, 50);
    }

    #[test]
    pub fn calculate_crop_from_100_100() {
        let (x1, y1, x2, y2) = calculate_crop(100, 100, 100, 100, 0.5);
        assert_eq!(x1, 50);
        assert_eq!(y1, 50);
        assert_eq!(x2, 100);
        assert_eq!(y2, 100);
    }

    #[test]
    pub fn calculate_crop_from_50_50() {
        let (x1, y1, x2, y2) = calculate_crop(100, 100, 50, 50, 0.5);
        assert_eq!(x1, 25);
        assert_eq!(y1, 25);
        assert_eq!(x2, 75);
        assert_eq!(y2, 75);
    }
}
