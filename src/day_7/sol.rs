#[path = "../util.rs"]
mod util;

use std::collections::HashMap;

pub fn part_1() {
    let input: String = util::get_input_part(7, 1);

    let mut s: u64 = 0;

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let target_str = parts.get(0).unwrap();
        let target_str = &target_str[..target_str.len() - 1];
        let target: u64 = target_str.parse().unwrap();

        let nums: Vec<u64> = parts[1..].iter().map(|p| p.parse().unwrap()).collect();

        if r(target, nums[0], &nums[1..]) {
            s += target;
        }
    }

    println!("{}", s);
}

pub fn part_2() {
    let input: String = util::get_input_part(7, 1);

    let mut s: u64 = 0;

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let target_str = parts.get(0).unwrap();
        let target_str = &target_str[..target_str.len() - 1];
        let target: u64 = target_str.parse().unwrap();

        let nums: Vec<u64> = parts[1..].iter().map(|p| p.parse().unwrap()).collect();

        if r2(target, nums[0], &nums[1..]) {
            s += target;
        }
    }

    println!("{}", s);
}

fn r(target: u64, acc: u64, rest: &[u64]) -> bool {
    if acc > target {
        return false;
    }

    if rest.len() == 0 {
        if acc == target {
            return true;
        }
        return false;
    }

    let cur = rest[0];
    let rest = &rest[1..];
    return r(target, acc + cur, rest) || r(target, acc * cur, rest);
}

fn r2(target: u64, acc: u64, rest: &[u64]) -> bool {
    if acc > target {
        return false;
    }

    if rest.len() == 0 {
        if acc == target {
            return true;
        }
        return false;
    }

    let cur = rest[0];
    let rest = &rest[1..];

    let cat_lhs = acc.checked_mul((10 as u64).pow(cur.to_string().len() as u32));
    let cat_result = cat_lhs.and_then(|c| c.checked_add(cur));
    let cat = cat_result.is_some_and(|c| r2(target, c, rest));

    return r2(target, acc + cur, rest) || r2(target, acc * cur, rest) || cat;
}
