#[path = "../util.rs"]
mod util;

use std::collections::{HashMap, VecDeque};

const DIRECTIONS: [[i32; 2]; 4] = [[1, 0], [0, 1], [-1, 0], [0, -1]];

fn num_reachable_from(grid: &HashMap<(i32, i32), u32>, r: i32, c: i32) -> i32 {
    let mut reachable: HashMap<(i32, i32), bool> = HashMap::new();
    r1(&mut reachable, grid, 0, r, c);
    return reachable.len() as i32;
}

fn rating(grid: &HashMap<(i32, i32), u32>, r: i32, c: i32) -> i32 {
    let mut reachable: HashMap<(i32, i32), bool> = HashMap::new();
    return r1(&mut reachable, grid, 0, r, c);
}

fn r1(
    reachable: &mut HashMap<(i32, i32), bool>,
    grid: &HashMap<(i32, i32), u32>,
    cur: u32,
    r: i32,
    c: i32,
) -> i32 {
    if !grid.get(&(r, c)).is_some_and(|&x| x == cur) {
        return 0;
    }

    if cur == 9 {
        reachable.insert((r, c), true);
        return 1;
    }

    return DIRECTIONS
        .map(|[d_r, d_c]| r1(reachable, grid, cur + 1, r + d_r, c + d_c))
        .iter()
        .sum();
}

pub fn part_1() {
    let input: String = util::get_input_part(10, 1);

    let mut grid: HashMap<(i32, i32), u32> = HashMap::new();
    let mut trailheads: Vec<(i32, i32)> = Vec::new();
    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            grid.insert((row as i32, col as i32), ch.to_digit(10).unwrap());
            if ch == '0' {
                trailheads.push((row as i32, col as i32));
            }
        }
    }

    let mut count = 0;
    for &(t_r, t_c) in trailheads.iter() {
        count += num_reachable_from(&grid, t_r, t_c);
    }

    println!("{}", count);
}

pub fn part_2() {
    let input: String = util::get_input_part(10, 1);

    let mut grid: HashMap<(i32, i32), u32> = HashMap::new();
    let mut trailheads: Vec<(i32, i32)> = Vec::new();
    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            grid.insert((row as i32, col as i32), ch.to_digit(10).unwrap());
            if ch == '0' {
                trailheads.push((row as i32, col as i32));
            }
        }
    }

    let mut count = 0;
    for &(t_r, t_c) in trailheads.iter() {
        count += rating(&grid, t_r, t_c);
    }

    println!("{}", count);
}
