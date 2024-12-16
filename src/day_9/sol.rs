#[path = "../util.rs"]
mod util;

use std::collections::{HashMap, VecDeque};

pub fn part_1() {
    let input: String = util::get_input_part(9, 1);

    let mut cur_pos = 0;
    let mut cur_file_id = 0;
    let mut disk: Vec<i32> = Vec::new();
    let mut free_positions: VecDeque<usize> = VecDeque::new();
    let mut file_mode = true;

    let line: &str = input.lines().next().unwrap();
    for ch in line.chars() {
        let len: u32 = ch.to_digit(10).unwrap();
        if file_mode {
            for _ in 0..len {
                disk.push(cur_file_id);
                cur_pos += 1;
            }
            cur_file_id += 1;
        } else {
            for _ in 0..len {
                disk.push(-1);
                free_positions.push_back(cur_pos as usize);
                cur_pos += 1;
            }
        }

        file_mode = !file_mode
    }

    let disk_len = disk.len();
    for i in (0..disk_len).rev() {
        let cur_val = disk[i];
        if cur_val == -1 {
            continue;
        }

        let free_position = free_positions.pop_front().unwrap();
        if free_position > i {
            break;
        }

        disk[i] = -1;
        disk[free_position] = cur_val;
    }

    let mut s: u64 = 0;
    for (i, file_id) in disk.iter().enumerate() {
        if *file_id == -1 {
            break;
        }
        s += ((i as i32) * *file_id) as u64;
    }

    println!("{}", s);
}

pub fn part_2() {
    let input: String = util::get_input_part(9, 1);

    let mut cur_pos = 0;
    let mut cur_file_id = 0;
    let mut disk: Vec<i32> = Vec::new();
    let mut file_lens: HashMap<i32, usize> = HashMap::new();
    let mut free_positions: VecDeque<(usize, usize)> = VecDeque::new();
    let mut file_mode = true;

    let line: &str = input.lines().next().unwrap();
    for ch in line.chars() {
        let len: u32 = ch.to_digit(10).unwrap();
        if file_mode {
            file_lens.insert(cur_file_id, len as usize);
            for _ in 0..len {
                disk.push(cur_file_id);
                cur_pos += 1;
            }
            cur_file_id += 1;
        } else {
            free_positions.push_back((cur_pos as usize, len as usize));
            for _ in 0..len {
                disk.push(-1);
                cur_pos += 1;
            }
        }

        file_mode = !file_mode
    }

    let disk_len = disk.len();
    for i in (0..disk_len).rev() {
        let cur_val = disk[i];
        if cur_val == -1 {
            continue;
        }

        let this_file_len = *file_lens.get(&cur_val).unwrap();
        let mut free_position_index = -1;
        for (j, &(_, free_position_length)) in free_positions.iter().enumerate() {
            if free_position_length < this_file_len {
                continue;
            }
            free_position_index = j as i32;
            break;
        }

        if free_position_index != -1 {
            let free_position = free_positions
                .get_mut(free_position_index as usize)
                .unwrap();

            if free_position.0 > i {
                continue;
            }

            for j in 0..this_file_len {
                disk[free_position.0 + j] = cur_val;
                disk[i - j] = -1;
            }

            if free_position.1 > this_file_len {
                free_position.0 += this_file_len;
                free_position.1 -= this_file_len;
            } else {
                free_positions.remove(free_position_index as usize);
            }
        }
    }

    let mut s: u64 = 0;
    for (i, file_id) in disk.iter().enumerate() {
        if *file_id == -1 {
            continue;
        }
        s += ((i as i32) * *file_id) as u64;
    }

    println!("{}", s);
}
