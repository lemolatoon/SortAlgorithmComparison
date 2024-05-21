use crate::counter::Counter;

#[derive(Clone)]
pub struct ArrayWrapper<T: Clone + Eq + Ord> {
    array: Vec<T>,
    counter: Counter,
}

#[derive(Debug)]
pub struct ValueWrapper<'a, T: Clone + Eq + Ord> {
    v: &'a T,
    counter: &'a Counter,
}

impl<'a, T: Clone + Eq + Ord> PartialEq for ValueWrapper<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.counter.inclement_compare();
        self.v == other.v
    }
}

impl<'a, T: Clone + Eq + Ord> Eq for ValueWrapper<'a, T> {}
impl<'a, T: Clone + Eq + Ord> PartialOrd for ValueWrapper<'a, T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<'a, T: Clone + Eq + Ord> Ord for ValueWrapper<'a, T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.counter.inclement_compare();
        self.v.cmp(other.v)
    }
}

impl<T: Clone + Eq + Ord> ArrayWrapper<T> {
    pub fn new(array: Vec<T>) -> Self {
        Self {
            array,
            counter: Counter::new(),
        }
    }
    pub fn swap(&mut self, i: usize, j: usize) {
        self.counter.inclement_swap();
        self.array.swap(i, j);
    }

    pub fn len(&self) -> usize {
        self.array.len()
    }

    pub fn is_empty(&self) -> bool {
        self.array.is_empty()
    }

    pub fn counter(&self) -> &Counter {
        &self.counter
    }

    pub fn get(&self, i: usize) -> ValueWrapper<T> {
        ValueWrapper {
            v: &self.array[i],
            counter: &self.counter,
        }
    }
}
