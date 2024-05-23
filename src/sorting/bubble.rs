use crate::data::ArrayWrapper;

pub fn bubble<T: std::fmt::Debug + Clone + Eq + Ord>(array: &mut ArrayWrapper<T>) {
    let n = array.len();
    for i in 0..n {
        for j in ((i + 1)..n).rev() {
            if array.get(j - 1) > array.get(j) {
                array.swap(j - 1, j);
            }
        }
    }
}
