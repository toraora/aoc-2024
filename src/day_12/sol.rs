#[path = "../util.rs"]
mod util;

use core::num;
use num_bigint::BigInt;
use num_traits::Zero;
use std::collections::{HashMap, VecDeque};
use util::{parse_grid, Grid};

pub fn part_1() {
    let input: String = util::get_input_part(12, 1);

    let grid = parse_grid(&input);

    let mut s: i64 = 0;
    let mut visited: HashMap<(i32, i32), bool> = HashMap::new();
    for &(r, c) in grid.keys() {
        let cur_ch = grid.get(&(r, c)).unwrap();
        let mut num_nodes = 0;
        let mut num_edges = 0;
        if visited.contains_key(&(r, c)) {
            continue;
        }

        let mut q: VecDeque<(i32, i32)> = VecDeque::new();
        q.push_back((r, c));
        visited.insert((r, c), true);

        while !q.is_empty() {
            num_nodes += 1;
            let (r, c) = q.pop_front().unwrap();
            for (d_r, d_c) in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let n_r = r + d_r;
                let n_c = c + d_c;
                if grid.get(&(n_r, n_c)).is_some_and(|ch| ch == cur_ch) {
                    num_edges += 1;
                    if !visited.contains_key(&(n_r, n_c)) {
                        visited.insert((n_r, n_c), true);
                        q.push_back((n_r, n_c));
                    }
                }
            }
        }

        s += (4 * num_nodes - num_edges) * num_nodes;
    }

    println!("{}", s);
}

pub fn part_2() {
    let input: String = util::get_input_part(12, 1);

    let grid = parse_grid(&input);

    let mut s: i64 = 0;
    let mut visited: HashMap<(i32, i32), bool> = HashMap::new();

    for &(r, c) in grid.keys() {
        let cur_ch = grid.get(&(r, c)).unwrap();
        let mut nodes: Vec<(i32, i32)> = vec![];
        let mut edges: HashMap<(i32, i32), HashMap<(i32, i32), bool>> = HashMap::new();

        if visited.contains_key(&(r, c)) {
            continue;
        }

        let mut q: VecDeque<(i32, i32)> = VecDeque::new();
        q.push_back((r, c));
        visited.insert((r, c), true);

        while !q.is_empty() {
            let (r, c) = q.pop_front().unwrap();
            nodes.push((r, c));
            for (d_r, d_c) in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let n_r = r + d_r;
                let n_c = c + d_c;
                if grid.get(&(n_r, n_c)).is_some_and(|ch| ch == cur_ch) {
                    if !visited.contains_key(&(n_r, n_c)) {
                        visited.insert((n_r, n_c), true);
                        q.push_back((n_r, n_c));
                    }
                } else {
                    edges
                        .entry((d_r, d_c))
                        .or_insert(HashMap::new())
                        .insert((r, c), true);
                }
            }
        }

        let mut num_edges = 0;
        for (d, d_edges) in edges {
            for &(e_r, e_c) in d_edges.keys() {
                if !d_edges.contains_key(&(e_r - 1, e_c)) && !d_edges.contains_key(&(e_r, e_c - 1))
                {
                    num_edges += 1;
                }
            }
        }

        s += num_edges * nodes.len() as i64;
    }

    println!("{}", s);
}
