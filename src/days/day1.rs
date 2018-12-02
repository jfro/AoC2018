//! Solution for AoC 2018 Day 1
//! https://adventofcode.com/2018/day/1
//!

use std::collections::HashMap;
use ::utils::lines_for_file;

fn step1() -> String {
    let mut freq = 0;
    for line in lines_for_file(1, None) {
        let line = line.unwrap();
        let my_int = line.parse::<i32>().unwrap();
        freq += my_int;
    }
    format!("{}", freq)
}

fn step2(frequency: i32, previous: &mut HashMap<i32, i32>) -> (i32, Option<i32>) {
    let mut frequency = frequency;
    for line in lines_for_file(1, None) {
        let line = line.unwrap();
        let my_int = line.parse::<i32>().unwrap();
        frequency += my_int;
        if previous.contains_key(&frequency) {
            return (frequency, Some(frequency));
        } else {
            previous.insert(frequency, 1);
        }
    }
    (frequency, None)
}

pub fn run(step: u8) -> String {
    if step == 1 {
        return step1();
    } else {
        let mut mymap: HashMap<i32, i32> = HashMap::new();
        let mut prev_freq = 0;
        let (f, _) = step2(prev_freq, &mut mymap);
        prev_freq = f;
        loop {
            let (f, dupe) = step2(prev_freq, &mut mymap);
            prev_freq = f;
            if let Some(d) = dupe {
                return format!("{}", d);
            }
        }
    }
}

mod tests {
    #[test]
    fn test_step1() {
        use super::*;
        assert_eq!(run(1), "416");
    }

    #[test]
    fn test_step2() {
        use super::*;
        assert_eq!(run(2), "56752");
    }
}