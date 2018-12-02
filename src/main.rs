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
            Arg::with_name("step")
                .short("s")
                .long("step")
                .help("Specifies step to run for selected day puzzle")
                .default_value("1"),
        ).get_matches();

    let day = matches.value_of("day").unwrap();
    let step = matches.value_of("step").unwrap();

    match days::run(day, step) {
        Ok(_) => {}
        Err(e) => {
            println!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
