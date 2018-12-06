use crate::utils::lines_for_file;
use std::collections::HashMap;
use regex::Regex;
use lazy_static::lazy_static;

#[derive(Debug, PartialEq)]
enum GuardStatus {
    BeginShift,
    Awake,
    Asleep,
    Unknown,
}

#[derive(Debug)]
struct Date {
    year: u16,
    month: u8,
    day: u8,
}

#[derive(Debug, PartialEq)]
struct Time {
    hour: u8,
    minute: u8,
}

#[derive(Debug)]
struct GuardEvent {
    date: Date,
    time: Time,
    id: u32,
    state: GuardStatus,
}

lazy_static! {
    static ref LINE_RE: Regex = Regex::new(r"(?x)
\[(?P<year>\d{4})  # the year
-
(?P<month>\d{2}) # the month
-
(?P<day>\d{2})   # the day
\s(?P<hour>\d{2}) # hour
:(?P<min>\d{2})\]   # min
\s(Guard\s\#(\d*))?
([\s\w]+)?
").unwrap();
}
// -----

fn generate_event(line: &str, guard_id: Option<u32>) -> Option<GuardEvent> {
//         let re = Regex::new(r"(?x)
// \[(?P<year>\d{4})  # the year
// -
// (?P<month>\d{2}) # the month
// -
// (?P<day>\d{2})   # the day
// \s(?P<hour>\d{2}) # hour
// :(?P<min>\d{2})\]   # min
// \s(Guard\s\#(\d*))?
// ([\s\w]+)?
// ").unwrap();
    let caps = LINE_RE.captures(line).unwrap();
    let date = Date { 
        year: caps["year"].parse::<u16>().unwrap(),
        month: caps["month"].parse::<u8>().unwrap(),
        day: caps["day"].parse::<u8>().unwrap(),
    };
    let time = Time {
        hour: caps["hour"].parse::<u8>().unwrap(),
        minute: caps["min"].parse::<u8>().unwrap(),
    };
    let mut final_guard_id = guard_id;
    let mut final_status = GuardStatus::Unknown;
    if let Some(guard) = caps.get(7) {
        let new_guard_id = guard.as_str().parse::<u32>().unwrap();
        final_guard_id = Some(new_guard_id);
        final_status = GuardStatus::BeginShift;
    }
    else if let Some(status) = caps.get(8) {
        final_status = match status.as_str() {
            "falls asleep" => GuardStatus::Asleep,
            "wakes up" => GuardStatus::Awake,
            _ => GuardStatus::Unknown
        };
    }
    Some(GuardEvent { date: date, time: time, id: final_guard_id.unwrap(), state: final_status })
}

fn generate_events(lines: Vec<String>) -> Vec<GuardEvent> {
    let mut events = Vec::new();

    let mut last_id: u32 = 0;
    for line in lines {
        if let Some(event) = generate_event(&line, Some(last_id)) {
            last_id = event.id;
            events.push(event);
        }
        else {
            panic!("Failed to generate event");
        }
    }

    events
}

// -----

fn part1() -> String {
    let mut line_strings: Vec<String> = Vec::new();
    for line in lines_for_file(4, Some("input.txt")) {
        if let Ok(line) = line {
            line_strings.push(line);
        }
    }
    line_strings.sort();
    let events = generate_events(line_strings);

    let mut guard_map: HashMap<u32, [u32; 60]> = HashMap::new();
    let mut sleep_start: Option<u8> = None;
    for event in events.iter() {
        // println!("Event: {:?}", event);
        if event.state == GuardStatus::Asleep {
            sleep_start = Some(event.time.minute);
        }
        else if event.state == GuardStatus::Awake {
            if let Some(sleep_start) = sleep_start  {
                let range = sleep_start..event.time.minute;
                let mins = guard_map.entry(event.id).or_insert([0; 60]);
                for min in range {
                    mins[min as usize] += 1;
                }
            }
            else {
                panic!("Awoke without a sleep: {:?}", event);
            }
            sleep_start = None;
        }

        // println!("Event: {:?}", event);
    }
    // for (id, mins) in guard_map.iter() {
    //     print!("Guard {}: ", id);
    //     for i in mins.iter() {
    //         print!("{} ", i);
    //     }
    //     println!("");
    // }
    let mut most_mins = 0;
    let mut found_id = 0;
    for (id, mins) in guard_map.iter() {
        let sum = mins.iter().sum();
        if sum > most_mins {
            most_mins = sum;
            found_id = *id;
        }
    }
    // println!("{} with {}", found_id, most_mins);
    let mut best_min = 0;
    let mut best_count = 0;
    for (min, &count) in guard_map[&found_id].iter().enumerate() {
        if count > best_count {
            best_min = min as u32;
            best_count = count;
        }
    }
    // println!("Best min: {}", best_min);
    format!("{}", best_min * found_id)
}

fn part2() -> String {
    String::from("unfinished")
}

pub fn run(part: u8) -> String {
    if part == 2 {
        part2()
    } else {
        part1()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
