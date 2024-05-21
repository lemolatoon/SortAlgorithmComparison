use crate::counter::Counter;

#[derive(Clone)]
pub struct ArrayWrapper<T: Clone + Eq + Ord> {
    array: Vec<T>,
    counter: Counter,
}

impl<T> std::fmt::Debug for ArrayWrapper<T>
where
    T: std::fmt::Debug + Clone + Eq + Ord,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ArrayWrapper")
            .field("array", &self.array)
            .finish()
    }
}

#[derive(Debug)]
pub struct ValueRef<'a, T: Clone + Eq + Ord> {
    v: &'a T,
    counter: &'a Counter,
}

impl<'a, T: Clone + Eq + Ord> PartialEq for ValueRef<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.counter.inclement_compare();
        self.v == other.v
    }
}

#[derive(Debug)]
pub struct OwnedValue<T: Clone + Eq + Ord> {
    v: T,
}

impl<T: Clone + Eq + Ord> OwnedValue<T> {
    pub fn to_ref<'a>(&'a self, counter: &'a Counter) -> ValueRef<'a, T> {
        ValueRef {
            v: &self.v,
            counter,
        }
    }
}

impl<'a, T: Clone + Eq + Ord> ValueRef<'a, T> {
    pub fn clone_value(&self) -> OwnedValue<T> {
        let v = self.v.clone();
        OwnedValue { v }
    }
}

impl<'a, T: Clone + Eq + Ord> Eq for ValueRef<'a, T> {}
impl<'a, T: Clone + Eq + Ord> PartialOrd for ValueRef<'a, T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<'a, T: Clone + Eq + Ord> Ord for ValueRef<'a, T> {
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
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.array.iter()
    }
    pub fn into_inner(self) -> (Vec<T>, Counter) {
        let Self { array, counter } = self;

        (array, counter)
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

    pub fn get(&self, i: usize) -> ValueRef<T> {
        ValueRef {
            v: &self.array[i],
            counter: &self.counter,
        }
    }
}
