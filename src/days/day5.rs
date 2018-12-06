
fn part1() -> String {
    let test = "dabAcCaCBAcCcaDA";
    let mut iter = test.chars().peekable();
    while let Some(c) = iter.next() {
        if let Some(next_c) = iter.peek() {
            let first = c as u8;
            let second = *next_c as u8;
            if first > second {
                if first - second == 32 {
                    print!("{}{} ", c, next_c);
                }
            }
            else if second > first {
                if second - first == 32 {
                    print!("{}{} ", c, next_c);
                }
            }
            
        }
        else {
            println!("The End");
        }
    }
    String::from("unfinished")
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
    // use super::*;
}