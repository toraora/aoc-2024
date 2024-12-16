#[path = "../util.rs"]
mod util;

use num_bigint::BigInt;
use num_traits::Zero;
use std::collections::{HashMap, VecDeque};

pub fn part_1() {
    let input: String = util::get_input_part(11, 1);

    let mut stones: Vec<BigInt> = input
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

    for _ in 0..25 {
        let mut new_stones: Vec<BigInt> = Vec::new();
        for stone in stones.iter() {
            if stone.is_zero() {
                new_stones.push(BigInt::from(1));
            } else if stone.to_string().len() % 2 == 0 {
                let stone_s = stone.to_string();
                new_stones.push(stone_s[..stone_s.len() / 2].parse().unwrap());
                new_stones.push(stone_s[stone_s.len() / 2..].parse().unwrap());
            } else {
                new_stones.push(stone * 2024);
            }
        }
        stones = new_stones;
    }

    println!("{}", stones.len());
}

pub fn part_2() {
    let input: String = util::get_input_part(11, 1);

    let stones: Vec<BigInt> = input
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

    let mut stone_counts: HashMap<BigInt, usize> = HashMap::new();
    for stone in stones {
        *stone_counts.entry(stone).or_insert(0) += 1;
    }

    for _ in 0..75 {
        let mut new_stone_counts: HashMap<BigInt, usize> = HashMap::new();
        for (stone, count) in stone_counts.iter() {
            if stone.is_zero() {
                *new_stone_counts.entry(BigInt::from(1)).or_insert(0) += count;
            } else if stone.to_string().len() % 2 == 0 {
                let stone_s = stone.to_string();

                *new_stone_counts
                    .entry(stone_s[..stone_s.len() / 2].parse().unwrap())
                    .or_insert(0) += count;
                *new_stone_counts
                    .entry(stone_s[stone_s.len() / 2..].parse().unwrap())
                    .or_insert(0) += count;
            } else {
                *new_stone_counts.entry(stone * 2024).or_insert(0) += count;
            }
        }
        stone_counts = new_stone_counts;
    }

    println!("{}", stone_counts.values().sum::<BigInt>());
}
