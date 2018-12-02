//! Solution for AoC 2018 Day 2
//! https://adventofcode.com/2018/day/2
//!

use std::collections::HashMap;
use crate::utils::lines_for_file;

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

#[derive(Debug, PartialEq)]
struct DifferenceResult {
    location: usize,
    count: usize,
}

fn difference_count(str1: &str, str2: &str) -> Option<DifferenceResult> {
    if str1.chars().count() != str2.chars().count() {
        return None;
    }
    let mut count = 0;
    let mut location = 0;
    for (i, item) in str1.chars().zip(str2.chars()).enumerate() {
        if item.0 != item.1 {
            location = i;
            count += 1;
        }
    }
    if count == 1 {
        return Some(DifferenceResult { location, count });
    }
    None
}

fn build_id_map() -> HashMap<String, BoxIDType> {
    let mut map: HashMap<String, BoxIDType> = HashMap::new();
    for line in lines_for_file(2, None) {
        let line = line.unwrap();
        let result = letter_count(&line);
        map.insert(line, result);
    }
    map
}

fn step1(map: HashMap<String, BoxIDType>) -> String {
    let three_count = map.iter().filter(|b| {*b.1 == BoxIDType::Triple || *b.1 == BoxIDType::Both}).count();
    let two_count = map.iter().filter(|b| {*b.1 == BoxIDType::Double || *b.1 == BoxIDType::Both}).count();
    format!("{}", three_count * two_count)
}

fn step2(map: HashMap<String, BoxIDType>) -> String {
    for outer in map.keys() {
        for inner in map.keys() {
            if outer == inner {
                continue;
            }
            if let Some(result) = difference_count(inner, outer) {
                let mut test = outer.clone();
                test.remove(result.location);
                return test;
            }
        }
    }
    String::from("")
}

pub fn run(step: u8) -> String {
    let map = build_id_map();
    if step == 2 {
        step2(map)
    }
    else {
        step1(map)
    }
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

    #[test]
    fn test_diff_count() {
        use super::*;
        assert_eq!(difference_count("abcdef", "abcdff"), Some(DifferenceResult { location: 4, count: 1}) );
        assert_eq!(difference_count("bcdef", "abcdff"), None);
        assert_eq!(difference_count("abcdec", "abcdff"), None);
    }

    #[test]
    fn test_step1() {
        use super::*;
        let map = build_id_map();
        let result = step1(map);
        assert_eq!(result, "5727");
    }

    #[test]
    fn test_step2() {
        use super::*;
        let map = build_id_map();
        let result = step2(map);
        assert_eq!(result, "uwfmdjxyxlbgnrotcfpvswaqh");
    }
}