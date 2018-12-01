extern crate aoc;
extern crate clap;
use clap::{Arg, App};

fn main() {
    use aoc::days;
    let matches = App::new("AoC 2018 Runner")
                        .version("1.0")
                        .author("Jeremy Knope <me@jeremyknope.com>")
                        .about("Manages running AoC 2018 puzzle solutions")
                        .arg(Arg::with_name("day")
                            .short("d")
                            // .long("day")
                            .help("Sets which day puzzle solution to run")
                            .default_value("1"))
                        .arg(Arg::with_name("step")
                            .short("s")
                            // .long("step")
                            .help("Specifies step to run for selected day puzzle")
                            .default_value("1"))
                            .get_matches();

    let day = matches.value_of("day").unwrap().parse::<u8>();
    let step = matches.value_of("step").unwrap().parse::<u8>();

    if let Err(_) = day {
        println!("Invalid day value: {}", matches.value_of("day").unwrap());
        std::process::exit(1);
    }
    if let Err(_) = step {
        println!("Invalid step value: {}", matches.value_of("step").unwrap());
        std::process::exit(1);
    }

    let day = day.unwrap();
    let step = step.unwrap();

    println!("Running day: {} Step: {}", day, step);
    match day {
        1 => {
            days::day1::run(step);
        },
        _ => {
            println!("Unimplemented day: {}", day);
            std::process::exit(1);
        }
    }
}
