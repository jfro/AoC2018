use crate::utils::lines_for_file;
use std::collections::HashSet;

#[derive(Debug, PartialEq)]
struct Claim {
    id: usize,
    left: usize,
    top: usize,
    width: usize,
    height: usize,
}
struct RegisteredClaims {
    claims: Vec<usize>,
    count: usize,
}

impl RegisteredClaims {
    fn new() -> RegisteredClaims {
        RegisteredClaims {
            claims: Vec::new(),
            count: 0,
        }
    }
}

impl Claim {
    fn right(&self) -> usize {
        self.left + self.width
    }

    fn bottom(&self) -> usize {
        self.top + self.height
    }
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

fn generate_claims() -> Vec<Claim> {
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
    claims
}

fn generate_grid(claims: &Vec<Claim>) -> (Vec<Vec<RegisteredClaims>>, HashSet<usize>) {
    let mut invalid_claims: HashSet<usize> = HashSet::new();
    let mut grid: Vec<Vec<RegisteredClaims>> = Vec::new();

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
                grid[row].push(RegisteredClaims::new());
            }
            for col in claim.left..claim.right() {
                if col >= grid[row].len() {
                    grid[row].push(RegisteredClaims::new());
                }
                grid[row][col].claims.push(claim.id);
                grid[row][col].count += 1;
                if grid[row][col].count > 1 {
                    // add claims that overlap to our list of invalid ones for part 2
                    for claim in &grid[row][col].claims {
                        invalid_claims.insert(*claim);
                    }
                }
            }
        }
    }
    (grid, invalid_claims)
}

fn part1(grid: Vec<Vec<RegisteredClaims>>) -> String {
    let total_overlap = grid.iter().map(|row| {
        row.iter().filter(|rc| rc.count > 1).count() as u32
    }).fold(0, |acc, x| acc + x);

    format!("{}", total_overlap)
}

fn part2(claims: Vec<Claim>, invalid_claims: HashSet<usize>) -> String {
    let results: Vec<usize> = claims.iter().filter(|c| {
        !invalid_claims.contains(&c.id)
    }).map(|c| c.id).collect();
    // we'd crash if this failed... but it shouldn't for this puzzle :)
    format!("{}", results[0])
}

pub fn run(part: u8) -> String {
    let claims = generate_claims();
    let (grid, invalid_claims) = generate_grid(&claims);
    if part == 2 {
        part2(claims, invalid_claims)
    }
    else {
        part1(grid)
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
    fn test_parts() {
        use super::*; // docs claim i can put this in mod level, but it produces warnings there
        let claims = generate_claims();
        let (grid, invalid_claims) = generate_grid(&claims);
        assert_eq!(part1(grid), "110389");
        assert_eq!(part2(claims, invalid_claims), "552");
    }
}
