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
