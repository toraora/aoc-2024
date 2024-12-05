#[path = "../util.rs"]
mod util;

use std::collections::HashMap;

pub fn part_1() {
    let input: String = util::get_input_part(4, 1);

    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        grid.push(row);
    }

    let mut count = 0;
    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            if grid[r][c] == 'X' {
                for dr in -1..=1 {
                    for dc in -1..=1 {
                        if dr == 0 && dc == 0 {
                            continue;
                        }

                        for d in 0..4 {
                            let rr = r as i32 + dr * d;
                            let cc = c as i32 + dc * d;
                            if (rr < 0 || rr >= grid.len() as i32)
                                || (cc < 0 || cc >= grid[r].len() as i32)
                            {
                                break;
                            }

                            if grid[rr as usize][cc as usize] == ['X', 'M', 'A', 'S'][d as usize] {
                                if d == 3 {
                                    count += 1;
                                }
                            } else {
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    println!("{}", count);
}

pub fn part_2() {
    let input: String = util::get_input_part(4, 1);

    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        grid.push(row);
    }

    let mut count = 0;
    for r in 1..(grid.len() - 1) {
        for c in 1..(grid[r].len() - 1) {
            if grid[r][c] == 'A' {
                let Ms = (if grid[r - 1][c - 1] == 'M' { 1 } else { 0 })
                    + (if grid[r - 1][c + 1] == 'M' { 1 } else { 0 })
                    + (if grid[r + 1][c - 1] == 'M' { 1 } else { 0 })
                    + (if grid[r + 1][c + 1] == 'M' { 1 } else { 0 });
                let As = (if grid[r - 1][c - 1] == 'S' { 1 } else { 0 })
                    + (if grid[r - 1][c + 1] == 'S' { 1 } else { 0 })
                    + (if grid[r + 1][c - 1] == 'S' { 1 } else { 0 })
                    + (if grid[r + 1][c + 1] == 'S' { 1 } else { 0 });
                if Ms == 2 && As == 2 && grid[r - 1][c - 1] != grid[r + 1][c + 1] {
                    count += 1;
                }
            }
        }
    }

    println!("{}", count);
}
