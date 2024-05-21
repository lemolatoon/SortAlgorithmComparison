use std::{cell::RefCell, rc::Rc};

use crate::counter::Counter;

pub struct Data<T: Clone + Eq + Ord> {
    value: T,
    counter: Rc<RefCell<Counter>>,
}

impl<T: Clone + Eq + Ord> Data<T> {
    pub fn new(value: T, counter: Rc<RefCell<Counter>>) -> Self {
        Self { value, counter }
    }
}

impl<T: Clone + Eq + Ord> Clone for Data<T> {
    fn clone(&self) -> Self {
        self.counter.borrow_mut().inclement_clone();
        Self {
            value: self.value.clone(),
            counter: Rc::clone(&self.counter),
        }
    }
}

impl<T: Clone + Eq + Ord> PartialEq for Data<T> {
    fn eq(&self, other: &Self) -> bool {
        self.counter.borrow_mut().inclement_compare();
        self.value == other.value
    }
}

impl<T: Clone + Eq + Ord> Eq for Data<T> {}

impl<T: Clone + Eq + Ord> PartialOrd for Data<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.counter.borrow_mut().inclement_compare();
        self.value.partial_cmp(&other.value)
    }
}

impl<T: Clone + Eq + Ord> Ord for Data<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.counter.borrow_mut().inclement_compare();
        self.value.cmp(&other.value)
    }
}
