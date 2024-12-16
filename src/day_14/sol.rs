#[path = "../util.rs"]
mod util;

use std::collections::HashMap;

struct Robot {
    p_x: i64,
    p_y: i64,
    v_x: i64,
    v_y: i64,
}

fn move_one(r: &mut Robot) {
    r.p_x = ((r.p_x + r.v_x) + W) % W;
    r.p_y = ((r.p_y + r.v_y) + H) % H;
}

fn p2(robots: &Vec<Robot>, p: bool) -> i32 {
    let mut g: Vec<Vec<i64>> = Vec::new();
    for _ in 0..H {
        g.push(vec![0; W as usize]);
    }

    for r in robots {
        g[r.p_y as usize][r.p_x as usize] = 1;
    }

    if p {
        for row in 0..H {
            let s: Vec<String> = g[row as usize].iter().map(|x| x.to_string()).collect();
            println!("{}", s.join(""));
        }
    }

    let mut neighbors = 0;
    let directions = [
        [1, 0],
        [1, 1],
        [0, 1],
        [-1, 1],
        [-1, 0],
        [-1, -1],
        [0, -1],
        [1, -1],
    ];
    for r in robots {
        if r.p_y == 0 || r.p_x == 0 || r.p_y == H - 1 || r.p_x == W - 1 {
            continue;
        }

        for d in directions {
            if g[(r.p_y + d[0]) as usize][(r.p_x + d[1]) as usize] == 1 {
                neighbors += 1;
            }
        }
    }

    return neighbors;
}

const W: i64 = 101;
const H: i64 = 103;

pub fn part_1() {
    let input: String = util::get_input_part(14, 1);

    let mut robots: Vec<Robot> = Vec::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let p: Vec<i64> = parts[0][2..]
            .split(",")
            .map(|p| p.parse().unwrap())
            .collect();
        let v: Vec<i64> = parts[1][2..]
            .split(",")
            .map(|p| p.parse().unwrap())
            .collect();
        robots.push(Robot {
            p_x: p[0],
            p_y: p[1],
            v_x: v[0],
            v_y: v[1],
        });
    }

    for _ in 0..100 {
        for r in &mut robots {
            move_one(r);
        }
    }

    let mut cnt = vec![0, 0, 0, 0];
    for r in robots {
        if r.p_x < W / 2 && r.p_y < H / 2 {
            cnt[0] += 1;
        }
        if r.p_x > W / 2 && r.p_y < H / 2 {
            cnt[1] += 1;
        }
        if r.p_x > W / 2 && r.p_y > H / 2 {
            cnt[2] += 1;
        }
        if r.p_x < W / 2 && r.p_y > H / 2 {
            cnt[3] += 1;
        }
    }

    println!("{}", cnt[0] * cnt[1] * cnt[2] * cnt[3])
}

pub fn part_2() {
    let input: String = util::get_input_part(14, 1);

    let mut robots: Vec<Robot> = Vec::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let p: Vec<i64> = parts[0][2..]
            .split(",")
            .map(|p| p.parse().unwrap())
            .collect();
        let v: Vec<i64> = parts[1][2..]
            .split(",")
            .map(|p| p.parse().unwrap())
            .collect();
        robots.push(Robot {
            p_x: p[0],
            p_y: p[1],
            v_x: v[0],
            v_y: v[1],
        });
    }

    for i in 1..10000 {
        for r in &mut robots {
            move_one(r);
        }
        let n = p2(&robots, false);
        if n as usize > 2 * robots.len() {
            println!("iteration {} neighbors {}", i, n);
            p2(&robots, true);
        }
    }
}
