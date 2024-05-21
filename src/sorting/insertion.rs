use crate::data::ArrayWrapper;

pub fn insertion<T: std::fmt::Debug + Clone + Eq + Ord>(array: &mut ArrayWrapper<T>) {
    for i in 1..array.len() {
        for j in (0..i).rev() {
            if array.get(j) <= array.get(j + 1) {
                break;
            }
            array.swap(j, j + 1);
        }
    }
}
