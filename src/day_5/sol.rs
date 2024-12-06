#[path = "../util.rs"]
mod util;

use std::collections::HashMap;

pub fn part_1() {
    let input: String = util::get_input_part(5, 1);

    let mut part = 1;
    let mut map: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut updates: Vec<Vec<u32>> = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            part = 2;
            continue;
        }

        if part == 1 {
            let a: u32 = line[0..2].parse().unwrap();
            let b: u32 = line[3..5].parse().unwrap();
            if !map.contains_key(&b) {
                map.insert(b, Vec::new());
            }
            map.get_mut(&b).unwrap().push(a);
        }

        if part == 2 {
            let mut v: Vec<u32> = Vec::new();
            for n in line.split(",") {
                v.push(n.parse().unwrap());
            }
            updates.push(v);
        }
    }

    let mut sum = 0;
    for update in updates {
        let mut ok = true;
        for i in 0..(update.len() - 1) {
            let rules = map.get(&update[i]);
            if rules.is_none() {
                continue;
            }
            for j in (i + 1)..update.len() {
                if rules.unwrap().contains(&update[j]) {
                    ok = false;
                    break;
                }
            }
        }

        if ok {
            sum += update.get((update.len() - 1) / 2).unwrap();
        }
    }

    println!("{}", sum);
}

fn topological_sort(map: &HashMap<u32, Vec<u32>>, vec: &Vec<u32>) -> Vec<u32> {
    let mut sorted: Vec<u32> = Vec::new();
    let mut visited: HashMap<u32, bool> = HashMap::new();

    for &n in vec {
        if !visited.contains_key(&n) {
            visit(n, map, vec, &mut visited, &mut sorted);
        }
    }

    sorted
}

fn visit(
    n: u32,
    map: &HashMap<u32, Vec<u32>>,
    vec: &Vec<u32>,
    visited: &mut HashMap<u32, bool>,
    sorted: &mut Vec<u32>,
) {
    if visited.contains_key(&n) {
        return;
    }

    visited.insert(n, true);

    if map.contains_key(&n) {
        for m in map.get(&n).unwrap() {
            if vec.contains(m) {
                visit(*m, map, vec, visited, sorted);
            }
        }
    }

    sorted.push(n);
}

pub fn part_2() {
    let input: String = util::get_input_part(5, 1);

    let mut part = 1;
    let mut map: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut updates: Vec<Vec<u32>> = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            part = 2;
            continue;
        }

        if part == 1 {
            let a: u32 = line[0..2].parse().unwrap();
            let b: u32 = line[3..5].parse().unwrap();
            if !map.contains_key(&b) {
                map.insert(b, Vec::new());
            }
            map.get_mut(&b).unwrap().push(a);
        }

        if part == 2 {
            let mut v: Vec<u32> = Vec::new();
            for n in line.split(",") {
                v.push(n.parse().unwrap());
            }
            updates.push(v);
        }
    }

    let mut sum = 0;
    for update in updates {
        let mut ok = true;
        for i in 0..(update.len() - 1) {
            let rules = map.get(&update[i]);
            if rules.is_none() {
                continue;
            }
            for j in (i + 1)..update.len() {
                if rules.unwrap().contains(&update[j]) {
                    ok = false;
                    break;
                }
            }
        }

        if !ok {
            let sorted = topological_sort(&map, &update);
            sum += *sorted.get((sorted.len() - 1) / 2).unwrap();
        }
    }

    println!("{}", sum);
}
