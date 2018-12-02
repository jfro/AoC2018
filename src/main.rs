extern crate aoc;
extern crate clap;
use clap::{App, Arg};

fn main() {
    use aoc::days;
    let matches = App::new("AoC 2018 Runner")
        .version("0.1.0")
        .author("Jeremy Knope <me@jeremyknope.com>")
        .about("Manages running AoC 2018 puzzle solutions")
        .arg(
            Arg::with_name("day")
                .short("d")
                .long("day")
                .help("Sets which day puzzle solution to run")
                .default_value("1"),
        ).arg(
            Arg::with_name("part")
                .short("p")
                .long("part")
                .help("Specifies which part of puzzle to run for selected day")
                .default_value("1"),
        ).get_matches();

    let day = matches.value_of("day").unwrap();
    let part = matches.value_of("part").unwrap();

    match days::run(day, part) {
        Ok(result) => {println!("Answer: {}", result)}
        Err(e) => {
            println!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
