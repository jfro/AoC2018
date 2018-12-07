use crate::utils::lines_for_file;

fn replace_reactions(string: &mut String) -> bool {
    let mut iter = string.chars().enumerate().peekable();
    // let mut replace_indices: Vec<usize> = Vec::new();
    while let Some((i, c)) = iter.next() {
        if let Some((_, next_c)) = iter.peek() {
            let first = c as u8;
            let second = *next_c as u8;
            if first > second {
                if first - second == 32 {
                    // print!("{}{} ", c, next_c);
                    // replace_indices.push(i);
                    // iter.next();
                    let r = i..(i+2);
                    string.replace_range(r, "");
                    return true
                }
            }
            else if second > first {
                if second - first == 32 {
                    // print!("{}{} ", c, next_c);
                    // replace_indices.push(i);
                    // iter.next();
                    let r = i..(i+2);
                    string.replace_range(r, "");
                    return true
                }
            }
            
        }
    }
    // let will_replace = replace_indices.len() > 0;
    // // println!("Indices: {:?}", replace_indices);
    // for index in replace_indices {
    //     let r = index..(index+2);
    //     string.replace_range(r, "");
    //     println!("Result: {}", string);
    // }
    // will_replace
    false
}
fn process_string(mut string: String) -> String {
    // loop until no more replacements happen
    // println!("Start : {}", string);
    while replace_reactions(&mut string) {}
    string
}

fn part2() -> String {
    String::from("unfinished")
}

pub fn run(part: u8) -> String {
    if part == 2 {
        part2()
    } else {
        let line = lines_for_file(5, None).next().unwrap();
        format!("{}", process_string(line.unwrap()).len())
    }
}

pub fn bench(part: u8) {
    if part == 1 {
        let test = String::from("dabAcCaCBAcCcaDAdabAcCaCBAcCcaDAdabAcCaCBAcCcaDAdabAcCaCBAcCcaDA");
        let _result = process_string(test);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let test = String::from("dabAcCaCBAcCcaDA");
        let result = process_string(test);
        assert_eq!(result, "dabCBAcaDA");
    }
}