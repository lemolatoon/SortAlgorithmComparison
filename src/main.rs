pub mod counter;
pub mod data;
pub mod sorting;

use std::fs::File;
use std::io::{BufRead, BufReader};

use data::ArrayWrapper;

fn sort_evaluator<T: Clone + Ord + Eq>(
    array: &mut ArrayWrapper<T>,
    f: impl FnOnce(&mut ArrayWrapper<T>),
) {
    let start = std::time::Instant::now();
    f(array);
    let end = start.elapsed();
    println!(
        "{}.{:025} sec\nswap_counter: {},\tcompare_counter: {}",
        end.as_secs(),
        end.subsec_nanos() / 1_000_000,
        array.counter().swap_counter(),
        array.counter().compare_counter()
    );
}

fn main() {
    let reader = BufReader::new(File::open("./data-utf8.csv").unwrap());
    let lines = reader
        .lines()
        .skip_while(|l| !l.as_ref().unwrap().starts_with('2'))
        .collect::<Result<Vec<String>, _>>()
        .unwrap();
    let mut array = Vec::<u16>::with_capacity(lines.len());
    let content = lines.join("\n");
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(content.as_bytes());

    for r in rdr.records() {
        let r = r.unwrap();
        let Ok(value) = r.get(1).unwrap().parse::<f32>() else {
            continue;
        };
        array.push((value * 10.0) as u16);
    }

    println!("Data length: {}", array.len());

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
        sort_evaluator(&mut array, sorting::radix2);
    }
    {
        println!("Insertion Sort!");
        let mut array = array.clone();
        sort_evaluator(&mut array, sorting::insertion);
    }
}
