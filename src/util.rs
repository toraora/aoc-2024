use std::collections::HashMap;
use std::fs;

pub type Grid = HashMap<(i32, i32), char>;

pub fn _get_input(day: u32) -> String {
    let path = format!("src/day_{}/in", day);
    fs::read_to_string(path).expect("Failed to read file")
}

pub fn get_input_part(day: u32, part: u32) -> String {
    let path = format!("src/day_{}/in_{}", day, part);
    fs::read_to_string(path).expect("Failed to read file")
}

pub fn parse_grid(input: &str) -> Grid {
    let mut grid: Grid = Grid::new();
    for (r, line) in input.lines().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            grid.insert((r as i32, c as i32), ch);
        }
    }

    return grid;
}
