pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;

/// Runs a solution for given day & part
pub fn run(day_s: &str, part_s: &str) -> Result<String, String> {
    let day = day_s.parse::<u8>();
    let part = part_s.parse::<u8>();

    if day.is_err() {
        return Err(format!("Invalid day value: {}", day_s));
    }
    if part.is_err() {
        return Err(format!("Invalid part value: {}", part_s));
    }

    let day = day.unwrap();
    let part = part.unwrap();

    println!("Running day: {} Part: {}", day, part);
    match day {
        1 => {
            Ok(day1::run(part))
        }
        2 => {
            Ok(day2::run(part))
        }
        3 => {
            Ok(day3::run(part))
        }
        4 => {
            Ok(day4::run(part))
        }
        _ => Err(String::from("Unimplemented day")),
    }
}
