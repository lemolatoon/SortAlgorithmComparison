pub mod counter;
pub mod data;
pub mod sorting;

use data::ArrayWrapper;
use rand::distributions::Uniform;
use rand::prelude::*;

fn main() {
    let n = 500;
    let mut array = Vec::with_capacity(n);
    let seed = 42;
    let uniform = Uniform::from(0..100000);
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    for _ in 0..n {
        let value = uniform.sample(&mut rng);
        array.push(value);
    }
    let array = ArrayWrapper::new(array);
    array.counter().reset_counter();
    {
        let mut array = array.clone();
        sorting::quick(&mut array);
        println!(
            "quick sort: swap_counter: {}, compare_counter: {}",
            array.counter().swap_counter(),
            array.counter().compare_counter()
        );
    }
}
