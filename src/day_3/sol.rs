#[path = "../util.rs"]
mod util;

use std::collections::HashMap;

pub fn part_1() {
    let input: String = util::get_input_part(3, 1);

    let re = regex::Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let matches = re.find_iter(&input);

    let mut sum = 0;
    for m in matches {
        let m_str = m.as_str();
        let m_str = &m_str[4..m_str.len() - 1];
        let nums = m_str.split(",").collect::<Vec<&str>>();
        let num1 = nums[0].parse::<i32>().unwrap();
        let num2 = nums[1].parse::<i32>().unwrap();
        sum += num1 * num2;
    }

    println!("{}", sum);
}

pub fn part_2() {
    let input: String = util::get_input_part(3, 1);

    let re = regex::Regex::new(r"mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\)").unwrap();
    let matches = re.find_iter(&input);

    let mut enabled = true;
    let mut sum = 0;
    for m in matches {
        if m.as_str() == "do()" {
            enabled = true;
            continue;
        } else if m.as_str() == "don't()" {
            enabled = false;
            continue;
        }

        if !enabled {
            continue;
        }

        let m_str = m.as_str();
        let m_str = &m_str[4..m_str.len() - 1];
        let nums = m_str.split(",").collect::<Vec<&str>>();
        let num1 = nums[0].parse::<i32>().unwrap();
        let num2 = nums[1].parse::<i32>().unwrap();
        sum += num1 * num2;
    }

    println!("{}", sum);
}
