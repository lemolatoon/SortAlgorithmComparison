pub mod counter;
pub mod data;
pub mod sorting;

use glob::glob;
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
        "{} sec\nswap_counter: {}\ncompare_counter: {}",
        end.as_secs_f64(),
        array.counter().swap_counter(),
        array.counter().compare_counter()
    );
}

fn main() {
    let mut array = Vec::<u16>::new();
    let pattern = "2*.csv";
    for entry in glob(pattern).unwrap() {
        let path = match entry {
            Ok(path) => path,
            Err(e) => {
                eprintln!("Failed to read: {:?}", e);
                continue;
            }
        };
        let reader = BufReader::new(File::open(path).unwrap());
        let lines = reader
            .lines()
            .skip_while(|l| !l.as_ref().unwrap().starts_with('2'))
            .collect::<Result<Vec<String>, _>>()
            .unwrap();
        let content = lines.join("\n");
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(content.as_bytes());

        for r in rdr.records() {
            let r = r.unwrap();
            let Ok(value) = r.get(1).unwrap().parse::<f32>() else {
                continue;
            };
            let value = value * 10.0;
            if value > u16::MAX as f32 {
                panic!("value too big for u16: {}", value);
            }
            array.push((value * 10.0) as u16);
        }
    }

    for i in [100, 1000, 10000, 100000] {
        let array: Vec<u16> = array.clone().into_iter().take(i).collect();
        println!("\n==========Data length: {}==========\n", array.len());

        let array = ArrayWrapper::new(array);
        array.counter().reset_counter();
        {
            println!("Radix Sort!");
            let mut array = array.clone();
            sort_evaluator(&mut array, sorting::radix2);
        }
        {
            println!("Quick Sort!");
            let mut array = array.clone();
            sort_evaluator(&mut array, sorting::quick);
        }
        {
            println!("Heap Sort!");
            let mut array = array.clone();
            sort_evaluator(&mut array, sorting::heap);
        }
        {
            println!("Shell Sort!");
            let mut array = array.clone();
            sort_evaluator(&mut array, sorting::shell);
        }
        {
            println!("Insertion Sort!");
            let mut array = array.clone();
            sort_evaluator(&mut array, sorting::insertion);
        }
        {
            println!("Bubble Sort!");
            let mut array = array.clone();
            sort_evaluator(&mut array, sorting::bubble);
        }
    }
    println!("\n==========Data length: {}==========\n", array.len());

    let array = ArrayWrapper::new(array);
    array.counter().reset_counter();
    {
        println!("Radix Sort!");
        let mut array = array.clone();
        sort_evaluator(&mut array, sorting::radix2);
    }
    {
        println!("Quick Sort!");
        let mut array = array.clone();
        sort_evaluator(&mut array, sorting::quick);
    }
    {
        println!("Heap Sort!");
        let mut array = array.clone();
        sort_evaluator(&mut array, sorting::heap);
    }
    {
        println!("Shell Sort!");
        let mut array = array.clone();
        sort_evaluator(&mut array, sorting::shell);
    }
    {
        println!("Insertion Sort!");
        let mut array = array.clone();
        sort_evaluator(&mut array, sorting::insertion);
    }
    {
        println!("Bubble Sort!");
        let mut array = array.clone();
        sort_evaluator(&mut array, sorting::bubble);
    }
}
