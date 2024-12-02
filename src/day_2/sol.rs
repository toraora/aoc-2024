#[path = "../util.rs"]
mod util;

use std::collections::HashMap;

fn vec_is_increasing(v: &Vec<i32>) -> bool {
    for i in 0..(v.len() - 1) {
        if v[i] >= v[i + 1] || v[i + 1] - v[i] > 3 {
            return false;
        }
    }

    true
}

fn vec_is_decreasing(v: &Vec<i32>) -> bool {
    for i in 0..(v.len() - 1) {
        if v[i] <= v[i + 1] || v[i] - v[i + 1] > 3 {
            return false;
        }
    }

    true
}

pub fn part_1() {
    let input: String = util::get_input_part(2, 1);

    let mut num_valid = 0;
    for line in input.lines() {
        let nums = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        if (vec_is_increasing(&nums) || vec_is_decreasing(&nums)) {
            num_valid += 1
        }
    }

    println!("{}", num_valid);
}

pub fn part_2() {
    let input: String = util::get_input_part(2, 1);

    let mut num_valid = 0;
    for line in input.lines() {
        let nums = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        for i in 0..nums.len() {
            let mut nums_copy = nums.clone();
            nums_copy.remove(i);
            if vec_is_increasing(&nums_copy) || vec_is_decreasing(&nums_copy) {
                num_valid += 1;
                break;
            }
        }
    }

    println!("{}", num_valid);
}
