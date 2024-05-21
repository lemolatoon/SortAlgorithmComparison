pub struct Counter {
    clone_counter: usize,
    compare_counter: usize,
}

impl Counter {
    pub fn new() -> Self {
        Self {
            clone_counter: 0,
            compare_counter: 0,
        }
    }
    pub fn inclement_clone(&mut self) {
        self.clone_counter += 1;
    }

    pub fn inclement_compare(&mut self) {
        self.compare_counter += 1;
    }

    #[must_use]
    pub fn clone_counter(&self) -> usize {
        self.clone_counter
    }

    #[must_use]
    pub fn compare_counter(&self) -> usize {
        self.compare_counter
    }

    pub fn reset_counter(&mut self) {
        self.clone_counter = 0;
        self.compare_counter = 0;
    }
}
