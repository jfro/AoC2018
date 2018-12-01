use std::fs::File;
use std::io::{BufReader};
use std::io::prelude::*;
use std::collections::HashMap;

#[allow(dead_code)]
fn step1() {
    let f = File::open("input.txt").expect("file not found");
    let f = BufReader::new(f);
    let mut freq = 0;
    for line in f.lines() {
        let line = line.unwrap();
        let my_int = line.parse::<i32>().unwrap();
        freq += my_int;
    }
    println!("Frequency: {}", freq);
}

fn step2(frequency: i32, previous: &mut HashMap<i32, i32>) -> (i32, Option<i32>) {
    let f = File::open("input.txt").expect("file not found");
    let f = BufReader::new(f);
    let mut frequency = frequency;
    for line in f.lines() {
        let line = line.unwrap();
        let my_int = line.parse::<i32>().unwrap();
        frequency += my_int;
        if previous.contains_key(&frequency) {
            return (frequency, Some(frequency));
        }
        else {
            previous.insert(frequency, 1);
        }
    }
    (frequency, None)
}

fn main() {
    // step1();
    let mut mymap: HashMap<i32, i32> = HashMap::new();
    let mut prev_freq = 0;
    let (f, _) = step2(prev_freq, &mut mymap);
    prev_freq = f;
    loop {
        let (f, dupe) = step2(prev_freq, &mut mymap);
        prev_freq = f;
        if let Some(d) = dupe {
            println!("Dupe: {}", d);
            return;
        }
    }
}
