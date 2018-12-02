//! Solution for AoC 2018 Day 2
//! https://adventofcode.com/2018/day/2
//!

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum BoxIDType {
    None,
    Double,
    Triple,
    Both,
}

fn letter_count(id: &str) -> BoxIDType {
    let mut result = BoxIDType::None;
    let mut map: HashMap<char, u8> = HashMap::new();
    for c in id.chars() {
        let counter = map.entry(c).or_insert(0);
        *counter += 1;
    }
    for (_, count) in map.iter().filter(|v| {
        let (_, v) = v;
        **v == 2 || **v == 3
    }) {
        result = match (&result, *count) {
            (BoxIDType::None, 2) => BoxIDType::Double,
            (BoxIDType::None, 3) => BoxIDType::Triple,
            (BoxIDType::Double, 3) => BoxIDType::Both,
            (BoxIDType::Triple, 2) => BoxIDType::Both,
            _ => result
        }
    }
    result
}

pub fn run(_step: u8) {
    let f = File::open("input/day2/input.txt").expect("file not found");
    let f = BufReader::new(f);
    let mut map: HashMap<String, BoxIDType> = HashMap::new();
    for line in f.lines() {
        let line = line.unwrap();
        let result = letter_count(&line);
        map.insert(line, result);
    }
    let three_count = map.iter().filter(|b| {*b.1 == BoxIDType::Triple || *b.1 == BoxIDType::Both}).count();
    let two_count = map.iter().filter(|b| {*b.1 == BoxIDType::Double || *b.1 == BoxIDType::Both}).count();
    println!("Answer: {}", three_count * two_count);
}

mod tests {
    #[test]
    fn test_char_count() {
        use super::*;
        assert_eq!(letter_count("aaabcd"), BoxIDType::Triple);
        assert_eq!(letter_count("aabcd"), BoxIDType::Double);
        assert_eq!(letter_count("aaabbcd"), BoxIDType::Both);
        assert_eq!(letter_count("abcdefghijkkkk"), BoxIDType::None);
    }
}