#[path = "../util.rs"]
mod util;

use std::collections::HashMap;

pub fn part_1() {
    let input: String = util::get_input_part(6, 1);

    let mut grid: Vec<Vec<bool>> = Vec::new();
    let mut r: i32 = 0;
    let mut c: i32 = 0;
    let mut found = false;

    for line in input.lines() {
        if !found {
            c = 0;
        }
        let mut row: Vec<bool> = Vec::new();
        for ch in line.chars() {
            if ch == '#' {
                row.push(true);
            } else {
                row.push(false);
            }

            if ch == '^' {
                found = true;
            }

            if !found {
                c += 1;
            }
        }
        grid.push(row);

        if !found {
            r += 1;
        }
    }

    let mut visited: HashMap<(i32, i32), bool> = HashMap::new();

    let directions = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut dir = 0;

    while r < grid.len() as i32 && c < grid[0].len() as i32 && r >= 0 && c >= 0 {
        visited.insert((r, c), true);

        let next_r = r + directions[dir].0;
        let next_c = c + directions[dir].1;

        if next_r < 0 || next_r >= grid.len() as i32 || next_c < 0 || next_c >= grid[0].len() as i32
        {
            break;
        }

        if grid[next_r as usize][next_c as usize] {
            dir = (dir + 1) % 4;
            continue;
        }

        r = next_r;
        c = next_c;
    }

    println!("{}", visited.len());
}

pub fn part_2() {
    let input: String = util::get_input_part(6, 1);

    let mut grid: Vec<Vec<bool>> = Vec::new();
    let mut start_r: i32 = 0;
    let mut start_c: i32 = 0;
    let mut found = false;

    for line in input.lines() {
        if !found {
            start_c = 0;
        }
        let mut row: Vec<bool> = Vec::new();
        for ch in line.chars() {
            if ch == '#' {
                row.push(true);
            } else {
                row.push(false);
            }

            if ch == '^' {
                found = true;
            }

            if !found {
                start_c += 1;
            }
        }
        grid.push(row);

        if !found {
            start_r += 1;
        }
    }

    let mut candidates = 0;
    for rr in 0..grid.len() {
        for cc in 0..grid[0].len() {
            if grid[rr][cc] {
                continue;
            }
            let mut r = start_r;
            let mut c = start_c;
            grid[rr][cc] = true;

            let mut visited: HashMap<(i32, i32), Vec<usize>> = HashMap::new();

            let directions = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
            let mut dir = 0;

            while r < grid.len() as i32 && c < grid[0].len() as i32 && r >= 0 && c >= 0 {
                if visited.get(&(r, c)).is_some_and(|v| v.contains(&dir)) {
                    candidates += 1;
                    break;
                }

                if !visited.contains_key(&(r, c)) {
                    visited.insert((r, c), Vec::new());
                }
                visited.get_mut(&(r, c)).unwrap().push(dir);

                let next_r = r + directions[dir].0;
                let next_c = c + directions[dir].1;

                if next_r < 0
                    || next_r >= grid.len() as i32
                    || next_c < 0
                    || next_c >= grid[0].len() as i32
                {
                    break;
                }

                if grid[next_r as usize][next_c as usize] {
                    dir = (dir + 1) % 4;
                    continue;
                }

                r = next_r;
                c = next_c;
            }

            grid[rr][cc] = false;
        }
    }

    println!("{}", candidates);
}
