use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Lines};

/// Simple wrapper around obtaining a line iterator for AoC input files
pub fn lines_for_file(day: u8, file: Option<&str>) -> Lines<BufReader<File>> {
    let filename = file.unwrap_or("input.txt");
    let day = format!("day{}", day);
    let f = File::open(format!("input/{}/{}", day, filename)).expect("file not found");
    let f = BufReader::new(f);
    f.lines()
}
