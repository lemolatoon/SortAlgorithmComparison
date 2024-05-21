use crate::data::ArrayWrapper;

pub fn quick<T: Clone + Eq + Ord>(array: &mut ArrayWrapper<T>) {
    quick_inner(array, 0, array.len() - 1);
}

fn quick_inner<T: Clone + Eq + Ord>(array: &mut ArrayWrapper<T>, left: usize, right: usize) {
    if left >= right {
        return;
    }

    let pivot = (left + right) / 2;
    let mut i = left;
    let mut j = right;
    loop {
        while array.get(i) < array.get(pivot) {
            i += 1;
        }

        while array.get(pivot) < array.get(j) {
            j -= 1;
        }

        if i <= j {
            array.swap(i, j);
            i += 1;
            j -= 1;
        }

        if i > j {
            break;
        }
    }

    quick_inner(array, left, j);
    quick_inner(array, i, right);
}
