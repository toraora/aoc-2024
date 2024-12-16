#[path = "../util.rs"]
mod util;

use std::collections::HashMap;

pub fn part_1() {
    let input: String = util::get_input_part(8, 1);

    let mut grid: HashMap<(i32, i32), char> = HashMap::new();
    let mut map: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    for (r, line) in input.lines().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            grid.insert((r as i32, c as i32), ch);

            if ch != '.' {
                if !map.contains_key(&ch) {
                    map.insert(ch, Vec::new());
                }
                map.get_mut(&ch).unwrap().push((r as i32, c as i32));
            }
        }
    }

    let mut antinodes: HashMap<(i32, i32), bool> = HashMap::new();
    for (ch, positions) in map.iter() {
        for i in 0..positions.len() {
            for j in 0..positions.len() {
                if i == j {
                    continue;
                }
                let p1 = positions[i];
                let p2 = positions[j];

                let pos = (p1.0 + 2 * (p2.0 - p1.0), p1.1 + 2 * (p2.1 - p1.1));
                if grid.get(&pos).is_some() {
                    antinodes.insert(pos, true);
                }
            }
        }
    }

    println!("{}", antinodes.len());
}

pub fn part_2() {
    let input: String = util::get_input_part(8, 1);

    let mut grid: HashMap<(i32, i32), char> = HashMap::new();
    let mut map: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    for (r, line) in input.lines().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            grid.insert((r as i32, c as i32), ch);

            if ch != '.' {
                if !map.contains_key(&ch) {
                    map.insert(ch, Vec::new());
                }
                map.get_mut(&ch).unwrap().push((r as i32, c as i32));
            }
        }
    }

    let mut antinodes: HashMap<(i32, i32), bool> = HashMap::new();
    for (ch, positions) in map.iter() {
        for i in 0..positions.len() {
            for j in 0..positions.len() {
                if i == j {
                    continue;
                }
                let p1 = positions[i];
                let p2 = positions[j];

                for k in -50..50 {
                    let pos = (p1.0 + k * (p2.0 - p1.0), p1.1 + k * (p2.1 - p1.1));
                    if grid.get(&pos).is_some() {
                        antinodes.insert(pos, true);
                    }
                }
            }
        }
    }

    println!("{}", antinodes.len());
}
