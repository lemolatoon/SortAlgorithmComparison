use crate::data::ArrayWrapper;

pub fn shell<T: std::fmt::Debug + Clone + Eq + Ord>(array: &mut ArrayWrapper<T>) {
    let mut h = 1;
    let n = array.len();

    while h < n {
        h = 3 * h + 1;
    }

    while h > 1 {
        h /= 3;
        for i in h..n {
            for j in (0..=(i - h)).rev().step_by(h) {
                if array.get(j) > array.get(j + h) {
                    array.swap(j, j + h);
                } else {
                    break;
                }
            }
        }
    }
}
