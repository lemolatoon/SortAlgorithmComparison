use crate::data::ArrayWrapper;

pub fn construct_heap<T: std::fmt::Debug + Clone + Eq + Ord>(
    array: &mut ArrayWrapper<T>,
    mut l: usize,
    r: usize,
) {
    let v = array.get(l).clone_value();
    loop {
        let l_left_child = 2 * l + 1;
        if l_left_child > r {
            break;
        }

        let l_right_child = l_left_child + 1;
        let larger_child =
            if l_left_child != r && array.get(l_right_child) > array.get(l_left_child) {
                l_right_child
            } else {
                l_left_child
            };
        if v.to_ref(array.counter()) >= array.get(larger_child) {
            break;
        }
        array.set(l, array.get(larger_child).into_raw());
        l = larger_child;
    }
    array.set(l, v.to_ref(array.counter()).into_raw());
}

pub fn heap<T: std::fmt::Debug + Clone + Eq + Ord>(array: &mut ArrayWrapper<T>) {
    let n = array.len();
    for i in (0..n).rev() {
        construct_heap(array, i, n - 1);
    }

    for i in (1..n).rev() {
        array.swap(0, i);
        construct_heap(array, 0, i - 1);
    }
}
