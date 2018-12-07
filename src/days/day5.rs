use crate::utils::lines_for_file;

fn replace_reactions(string: &mut String) -> bool {
    let mut iter = string.chars().enumerate().peekable();
    let mut replace_indices: Vec<usize> = Vec::new();
    while let Some((i, c)) = iter.next() {
        if let Some((_, next_c)) = iter.peek() {
            let first = c as u8;
            let second = *next_c as u8;
            if first > second {
                if first - second == 32 {
                    // print!("{}{} ", c, next_c);
                    replace_indices.push(i);
                    iter.next();
                    // let r = i..(i+2);
                    // string.replace_range(r, "");
                    // return true
                }
            }
            else if second > first {
                if second - first == 32 {
                    // print!("{}{} ", c, next_c);
                    replace_indices.push(i);
                    iter.next();
                    // let r = i..(i+2);
                    // string.replace_range(r, "");
                    // return true
                }
            }
            
        }
    }
    let mut offset = 0;
    let will_replace = replace_indices.len() > 0;
    // println!("Indices: {:?}", replace_indices);
    for index in replace_indices {
        // if offset > index {
        //     panic!("Offset too big: {} > {}", offset, index);
        // }
        let adjusted_index = index - offset;
        let r = adjusted_index..(adjusted_index+2);
        string.replace_range(r, "");
        offset = offset + 2;
        // println!("Result: {}", string);
    }
    will_replace
    // false
}
fn process_string(mut string: String) -> String {
    // loop until no more replacements happen
    // println!("Start : {}", string);
    while replace_reactions(&mut string) {}
    string
}

fn part2() -> String {
    let line = lines_for_file(5, None).next().unwrap().unwrap();
    let alphabet = ('a' as u8)..('z' as u8);
    let result = alphabet.map(|c| {
        let result = line.replace(c as char, "").replace((c - 32) as char, "");
        process_string(result).len()
    }).min().unwrap();
    format!("{}", result)
}

pub fn run(part: u8) -> String {
    if part == 2 {
        part2()
    } else {
        let line = lines_for_file(5, None).next().unwrap().unwrap();
        format!("{}", process_string(line).len())
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

    #[test]
    fn test_part1() {
        assert_eq!(run(1), "10762");
    }
}