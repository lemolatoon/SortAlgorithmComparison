pub mod counter;
pub mod data;
pub mod sorting;

use data::ArrayWrapper;
use rand::distributions::Uniform;
use rand::prelude::*;

fn sort_evaluator<T: Clone + Ord + Eq>(
    array: &mut ArrayWrapper<T>,
    f: impl FnOnce(&mut ArrayWrapper<T>),
) {
    let start = std::time::Instant::now();
    f(array);
    let end = start.elapsed();
    println!(
        "{}.{:020} sec\nswap_counter: {},\tcompare_counter: {}",
        end.as_secs(),
        end.subsec_nanos() / 1_000_000,
        array.counter().swap_counter(),
        array.counter().compare_counter()
    );
}

fn main() {
    let n = 5000000;
    let mut array = Vec::with_capacity(n);
    let seed = 42;
    let uniform = Uniform::from(u8::MIN..=u8::MAX);
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    for _ in 0..n {
        let value = uniform.sample(&mut rng);
        array.push(value);
    }
    let array = ArrayWrapper::new(array);
    array.counter().reset_counter();
    {
        println!("Quick Sort!");
        let mut array = array.clone();
        sort_evaluator(&mut array, sorting::quick);
    }
    {
        println!("Shell Sort!");
        let mut array = array.clone();
        sort_evaluator(&mut array, sorting::shell);
    }
    {
        println!("Radix Sort!");
        let mut array = array.clone();
        sort_evaluator(&mut array, sorting::radix);
    }
    {
        println!("Insertion Sort!");
        let mut array = array.clone();
        sort_evaluator(&mut array, sorting::insertion);
    }
}
