#[path = "../util.rs"]
mod util;

use std::collections::HashMap;

pub fn part_1() {
    let input: String = util::get_input(1);
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split("   ").collect();
        left.push(parts[0].parse().unwrap());
        right.push(parts[1].parse().unwrap());
    }

    left.sort();
    right.sort();
    let mut sum = 0;
    for i in 0..left.len() {
        sum += (left[i] - right[i]).abs();
    }

    println!("{}", sum);
}

pub fn part_2() {
    let input: String = util::get_input(1);
    let mut left: HashMap<i32, i32> = HashMap::new();
    let mut right: HashMap<i32, i32> = HashMap::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split("   ").collect();
        *left.entry(parts[0].parse().unwrap()).or_insert(0) += 1;
        *right.entry(parts[1].parse().unwrap()).or_insert(0) += 1;
    }

    let mut sum = 0;
    for k in left.keys() {
        sum += k * left[k] * right.get(k).unwrap_or(&0);
    }

    println!("{}", sum);
}
