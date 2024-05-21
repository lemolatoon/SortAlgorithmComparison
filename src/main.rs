pub mod counter;
pub mod data;

use data::Data;
use rand;
use rand::distributions::Uniform;
use rand::prelude::*;
use std::{cell::RefCell, rc::Rc};

use crate::counter::Counter;

fn main() {
    let counter = Rc::new(RefCell::new(Counter::new()));

    let n = 500;
    let mut array = Vec::with_capacity(n);
    let seed = 42;
    let uniform = Uniform::try_from(0..100000).unwrap();
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    for _ in 0..n {
        let value = uniform.sample(&mut rng);
        let data = Data::new(value, Rc::clone(&counter));
        array.push(data);
    }
    array.sort();
    println!(
        "clone_counter: {}, compare_counter: {}",
        counter.borrow().clone_counter(),
        counter.borrow().compare_counter(),
    );
}
