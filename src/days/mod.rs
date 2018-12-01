mod day1;
mod day2;

/// Runs a solution for given day & step
pub fn run(day_s: &str, step_s: &str) -> Result<(), String> {
    let day = day_s.parse::<u8>();
    let step = step_s.parse::<u8>();

    if day.is_err() {
        return Err(format!("Invalid day value: {}", day_s));
    }
    if step.is_err() {
        return Err(format!("Invalid step value: {}", step_s));
    }

    let day = day.unwrap();
    let step = step.unwrap();

    println!("Running day: {} Step: {}", day, step);
    match day {
        1 => {
            day1::run(step);
            Ok(())
        }
        2 => {
            day2::run(step);
            Ok(())
        }
        _ => Err(String::from("Unimplemented day")),
    }
}
