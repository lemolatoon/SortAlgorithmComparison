use std::cell::Cell;

#[derive(Debug, Clone)]
pub struct Counter {
    compare_counter: Cell<usize>,
    swap_counter: Cell<usize>,
}

impl Default for Counter {
    fn default() -> Self {
        Self::new()
    }
}

impl Counter {
    pub fn new() -> Self {
        Self {
            compare_counter: Cell::new(0),
            swap_counter: Cell::new(0),
        }
    }

    pub fn inclement_compare(&self) {
        self.compare_counter.set(self.compare_counter.get() + 1);
    }

    pub fn inclement_swap(&self) {
        self.swap_counter.set(self.swap_counter.get() + 1);
    }

    #[must_use]
    pub fn compare_counter(&self) -> usize {
        self.compare_counter.get()
    }

    #[must_use]
    pub fn swap_counter(&self) -> usize {
        self.swap_counter.get()
    }

    pub fn reset_counter(&self) {
        self.compare_counter.set(0);
        self.swap_counter.set(0);
    }
}
