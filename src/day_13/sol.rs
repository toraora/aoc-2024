#[path = "../util.rs"]
mod util;

use num_bigint::BigInt;
use std::collections::HashMap;

pub fn part_1() {
    let input: String = util::get_input_part(13, 1);

    let mut s: BigInt = BigInt::from(0);
    let lines: Vec<&str> = input.lines().collect();
    for i in 0..(lines.len() / 4 + 1) {
        let re = regex::Regex::new(r"Button A: X\+(\d+), Y\+(\d+)").unwrap();
        let caps = re.captures(lines[i * 4]).unwrap();
        let x_a: &BigInt = &caps.get(1).unwrap().as_str().parse().unwrap();
        let y_a: &BigInt = &caps.get(2).unwrap().as_str().parse().unwrap();

        let re = regex::Regex::new(r"Button B: X\+(\d+), Y\+(\d+)").unwrap();
        let caps = re.captures(lines[i * 4 + 1]).unwrap();
        let x_b: &BigInt = &caps.get(1).unwrap().as_str().parse().unwrap();
        let y_b: &BigInt = &caps.get(2).unwrap().as_str().parse().unwrap();

        let re = regex::Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
        let caps = re.captures(lines[i * 4 + 2]).unwrap();
        let x_p: &BigInt = &caps.get(1).unwrap().as_str().parse().unwrap();
        let y_p: &BigInt = &caps.get(2).unwrap().as_str().parse().unwrap();

        let b_presses = &((y_p * x_a - x_p * y_a) / (y_b * x_a - x_b * y_a));
        let a_presses = &((x_p - b_presses * x_b) / x_a);
        if a_presses * x_a + b_presses * x_b == *x_p && a_presses * y_a + b_presses * y_b == *y_p {
            s += 3 * a_presses + b_presses;
        }
    }

    println!("{}", s);
}

pub fn part_2() {
    let input: String = util::get_input_part(13, 1);

    let mut s: BigInt = BigInt::from(0);
    let lines: Vec<&str> = input.lines().collect();
    for i in 0..(lines.len() / 4 + 1) {
        let re = regex::Regex::new(r"Button A: X\+(\d+), Y\+(\d+)").unwrap();
        let caps = re.captures(lines[i * 4]).unwrap();
        let x_a: &BigInt = &caps.get(1).unwrap().as_str().parse().unwrap();
        let y_a: &BigInt = &caps.get(2).unwrap().as_str().parse().unwrap();

        let re = regex::Regex::new(r"Button B: X\+(\d+), Y\+(\d+)").unwrap();
        let caps = re.captures(lines[i * 4 + 1]).unwrap();
        let x_b: &BigInt = &caps.get(1).unwrap().as_str().parse().unwrap();
        let y_b: &BigInt = &caps.get(2).unwrap().as_str().parse().unwrap();

        let re = regex::Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
        let caps = re.captures(lines[i * 4 + 2]).unwrap();
        let x_p: &BigInt = &(caps.get(1).unwrap().as_str().parse::<BigInt>().unwrap()
            + BigInt::from(10000000000000i64));
        let y_p: &BigInt = &(caps.get(2).unwrap().as_str().parse::<BigInt>().unwrap()
            + BigInt::from(10000000000000i64));
        let b_presses = &((y_p * x_a - x_p * y_a) / (y_b * x_a - x_b * y_a));
        let a_presses = &((x_p - b_presses * x_b) / x_a);
        if a_presses * x_a + b_presses * x_b == *x_p && a_presses * y_a + b_presses * y_b == *y_p {
            s += 3 * a_presses + b_presses;
        }
    }

    println!("{}", s);
}
