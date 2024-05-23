use crate::data::ArrayWrapper;

pub fn radix(array: &mut ArrayWrapper<u8>) {
    let mut counts: [u8; 256] = [0; 256];

    for i in 0..array.len() {
        counts[array.get(i).into_raw::<u8>() as usize] += 1;
    }

    let mut inserting = 0;
    for (i, v) in counts.into_iter().enumerate() {
        for _ in 0..v {
            array.set(inserting, i as u8);
            inserting += 1;
        }
    }
}

pub fn radix2(array: &mut ArrayWrapper<u16>) {
    let mut count: [usize; 256] = [0; 256];
    let mut output: Vec<u16> = vec![0; array.len()];

    for &num in array.iter() {
        count[(num & 0xFF) as usize] += 1;
    }
    for i in 1..256 {
        count[i] += count[i - 1];
    }
    for &num in array.iter().rev() {
        count[(num & 0xFF) as usize] -= 1;
        output[count[(num & 0xFF) as usize]] = num;
    }
    array.swap_with_slice(&mut output);

    count = [0; 256];

    for &num in array.iter() {
        count[(num >> 8 & 0xFF) as usize] += 1;
    }
    for i in 1..256 {
        count[i] += count[i - 1];
    }
    for &num in array.iter().rev() {
        count[(num >> 8 & 0xFF) as usize] -= 1;
        output[count[(num >> 8 & 0xFF) as usize]] = num;
    }
    array.swap_with_slice(&mut output);
}
