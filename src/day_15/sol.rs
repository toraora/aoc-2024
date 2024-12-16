#[path = "../util.rs"]
mod util;

use std::collections::HashMap;

pub fn part_1() {
    let input: String = util::get_input_part(15, 1);

    let mut grid_mode = true;
    let mut grid: HashMap<(i32, i32), char> = HashMap::new();
    let mut cur: (i32, i32) = (0, 0);
    let mut directions: Vec<char> = Vec::new();

    for (r, line) in input.lines().enumerate() {
        if line.len() == 0 {
            grid_mode = false;
            continue;
        }
        if grid_mode {
            for (c, ch) in line.chars().enumerate() {
                if ch == '@' {
                    cur = (r as i32, c as i32);
                    grid.insert((r as i32, c as i32), '.');
                } else {
                    grid.insert((r as i32, c as i32), ch);
                }
            }
        } else {
            for ch in line.chars() {
                directions.push(ch);
            }
        }
    }

    for d in directions.iter() {
        let (d_r, d_c): (i32, i32) = match d {
            '^' => (-1, 0),
            'v' => (1, 0),
            '<' => (0, -1),
            '>' => (0, 1),
            _ => (0, 0),
        };

        let (mut cur_r, mut cur_c) = cur;
        let mut boxes_to_push: Vec<(i32, i32)> = Vec::new();
        let mut can_push_boxes = false;
        loop {
            cur_r = cur_r + d_r;
            cur_c = cur_c + d_c;

            let cur_cell = grid.get(&(cur_r, cur_c));

            if cur_cell.is_some_and(|&c| c == 'O') {
                boxes_to_push.push((cur_r, cur_c));
            } else if cur_cell.is_some_and(|&c| c == '.') {
                can_push_boxes = true;
                break;
            } else {
                break;
            }
        }

        if can_push_boxes {
            for (b_r, b_c) in boxes_to_push.iter() {
                grid.insert((b_r + d_r, b_c + d_c), 'O');
            }
            if boxes_to_push.len() != 0 {
                grid.insert(boxes_to_push[0], '.');
            }
            cur = (cur.0 + d_r, cur.1 + d_c);
        }
    }

    let mut s = 0;
    for (&(r, c), &ch) in grid.iter() {
        if ch == 'O' {
            s += 100 * r + c
        }
    }

    println!("{}", s);
}

pub fn part_2() {
    let input: String = util::get_input_part(15, 1);

    let mut grid_mode = true;
    let mut grid: HashMap<(i32, i32), char> = HashMap::new();
    let mut cur: (i32, i32) = (0, 0);
    let mut directions: Vec<char> = Vec::new();

    for (r, line) in input.lines().enumerate() {
        if line.len() == 0 {
            grid_mode = false;
            continue;
        }
        if grid_mode {
            for (c, ch) in line.chars().enumerate() {
                if ch == '@' {
                    cur = (r as i32, 2 * c as i32);
                    grid.insert((r as i32, 2 * c as i32), '.');
                    grid.insert((r as i32, 2 * c as i32 + 1), '.');
                } else if ch == '.' {
                    grid.insert((r as i32, 2 * c as i32), ch);
                    grid.insert((r as i32, 2 * c as i32 + 1), ch);
                } else if ch == 'O' {
                    grid.insert((r as i32, 2 * c as i32), '[');
                    grid.insert((r as i32, 2 * c as i32 + 1), ']');
                } else if ch == '#' {
                    grid.insert((r as i32, 2 * c as i32), ch);
                    grid.insert((r as i32, 2 * c as i32 + 1), ch);
                } else {
                    panic!("a");
                }
            }
        } else {
            for ch in line.chars() {
                directions.push(ch);
            }
        }
    }

    for d in directions.iter() {
        let (d_r, d_c): (i32, i32) = match d {
            '^' => (-1, 0),
            'v' => (1, 0),
            '<' => (0, -1),
            '>' => (0, 1),
            _ => (0, 0),
        };

        let mut boxes_to_push: Vec<(i32, i32)> = Vec::new();
        let mut can_push_boxes = true;
        let mut positions_to_check: Vec<(i32, i32)> = Vec::new();
        positions_to_check.push((cur.0 + d_r, cur.1 + d_c));
        while positions_to_check.len() != 0 {
            let (cur_r, cur_c) = positions_to_check.pop().unwrap();
            let cur_cell = grid.get(&(cur_r, cur_c));

            if cur_cell.is_some_and(|&c| c == '[') {
                boxes_to_push.push((cur_r, cur_c));
                positions_to_check.push((cur_r + d_r, cur_c + d_c));
                if *d == 'v' || *d == '^' {
                    positions_to_check.push((cur_r + d_r, cur_c + d_c + 1));
                }
            } else if cur_cell.is_some_and(|&c| c == ']') {
                boxes_to_push.push((cur_r, cur_c - 1));
                positions_to_check.push((cur_r + d_r, cur_c + d_c));
                if *d == 'v' || *d == '^' {
                    positions_to_check.push((cur_r + d_r, cur_c + d_c - 1));
                }
            } else if cur_cell.is_some_and(|&c| c == '.') {
                continue;
            } else {
                can_push_boxes = false;
                break;
            }
        }

        if can_push_boxes {
            for (b_r, b_c) in boxes_to_push.iter() {
                grid.insert((*b_r, *b_c), '.');
                grid.insert((*b_r, b_c + 1), '.');
            }
            for (b_r, b_c) in boxes_to_push.iter() {
                grid.insert((b_r + d_r, b_c + d_c), '[');
                grid.insert((b_r + d_r, b_c + d_c + 1), ']');
            }

            cur = (cur.0 + d_r, cur.1 + d_c);
        }
    }

    let mut s = 0;
    for (&(r, c), &ch) in grid.iter() {
        if ch == '[' {
            s += 100 * r + c
        }
    }

    println!("{}", s);
}
