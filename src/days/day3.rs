use crate::utils::lines_for_file;

#[derive(Debug, PartialEq)]
struct Claim {
    id: usize,
    left: usize,
    top: usize,
    width: usize,
    height: usize,
}

impl Claim {
    fn right(&self) -> usize {
        self.left + self.width
    }

    fn bottom(&self) -> usize {
        self.top + self.height
    }

    // fn intersects(&self, other: &Claim) -> bool {
    //     if other.left >= self.right() || self.left >= other.right() {
    //         return false;
    //     }
    //     if other.bottom() <= self.top || self.bottom() <= other.top {
    //         return false;
    //     }
    //     return true;
    // }
}

fn generate_claim(string: &str) -> Option<Claim> {
    // #1 @ 1,3: 4x4 (left, top: widthxheight)
    // this seems crappy...
    let at_index = string.find('@')?;
    let comma_index = string.find(',')?;
    let colon_index = string.find(':')?;
    let x_index = string.find('x')?;

    let id = (&string[1..at_index-1]).parse::<usize>().unwrap();
    let left = (&string[at_index+2..comma_index]).parse::<usize>().unwrap();
    let top = (&string[comma_index+1..colon_index]).parse::<usize>().unwrap();
    let width = (&string[colon_index+2..x_index]).parse::<usize>().unwrap();
    let height = (&string[x_index+1..string.len()]).parse::<usize>().unwrap();
    Some(Claim {
        id, left, top, width, height
    })
}

fn part1() -> String {
    let mut claims: Vec<Claim> = Vec::new();
    for line in lines_for_file(3, Some("input.txt")) {
        let line = line.unwrap();
        if let Some(claim) = generate_claim(&line) {
            claims.push(claim);
        }
        else {
            println!("Failed to generate claim: {}", line);
        }
    }

    let mut grid: Vec<Vec<i32>> = Vec::new();

    for claim in claims {

            // pad before claim if not there yet
            for _ in grid.len()..claim.top {
                grid.push(Vec::new());
            }
            for row in claim.top..claim.bottom() {
                // add row if missing
                if row >= grid.len() {
                    grid.push(Vec::new());
                }
                // pad x
                for _ in grid[row].len()..claim.left {
                    grid[row].push(0);
                }
                for col in claim.left..claim.right() {
                    if col >= grid[row].len() {
                        grid[row].push(0);
                    }
                    grid[row][col] += 1;
                }
            }

    }
    // println!("{:?}", grid);
    // for y in grid.iter() {
    //     for x in y.iter() {
    //         print!("{}", x);
    //     }
    //     println!("");
    // }

    let row_check = grid.iter().map(|row| {
        row.iter().filter(|count| **count > 1).count() as u32
    }).fold(0, |acc, x| acc + x);

    format!("{}", row_check)
}

fn part2() -> String {
    String::from("Unfinished")
}

pub fn run(part: u8) -> String {
    if part == 2 {
        part2()
    }
    else {
        part1()
    }
}

mod tests {
    #[test]
    fn test_generate_claim() {
        use super::*;    
        assert_eq!(Claim { id: 1, left: 1, top: 3, width: 4, height: 4 }, generate_claim("#1 @ 1,3: 4x4").unwrap());
        assert_eq!(Claim { id: 234, left: 3, top: 1235, width: 234, height: 1245 }, generate_claim("#234 @ 3,1235: 234x1245").unwrap());
    }

    #[test]
    fn test_intersects() {
        use super::*;
        let c1 = Claim { id: 1, left: 4, top: 4, width: 4, height: 4 };
        let c2 = Claim { id: 2, left: 1, top: 1, width: 4, height: 4 };
        assert!(c1.intersects(&c2));

        let c1 = Claim { id: 1, left: 4, top: 4, width: 4, height: 4 };
        let c2 = Claim { id: 2, left: 1, top: 0, width: 4, height: 4 };
        assert!(!c1.intersects(&c2));

        let c1 = Claim { id: 1, left: 4, top: 4, width: 4, height: 4 };
        let c2 = Claim { id: 2, left: 8, top: 0, width: 4, height: 4 };
        assert!(!c1.intersects(&c2));

        let c1 = Claim { id: 1, left: 4, top: 4, width: 4, height: 4 };
        let c2 = Claim { id: 2, left: 7, top: 1, width: 4, height: 4 };
        assert!(c1.intersects(&c2));
    }
}
