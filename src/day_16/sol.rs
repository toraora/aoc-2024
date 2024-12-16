#[path = "../util.rs"]
mod util;

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use util::{parse_grid, Grid};

const DIRECTIONS: [[i32; 2]; 4] = [[0, 1], [1, 0], [0, -1], [-1, 0]];

#[derive(Eq, Hash, PartialEq, Clone, Copy, PartialOrd, Ord, Debug)]
struct Node {
    r: i32,
    c: i32,
    d: usize,
}

fn dijkstra_all(
    start: Node,
    end: Node,
    graph: &HashMap<Node, Vec<(Node, i32)>>,
) -> (i32, Vec<Vec<Node>>) {
    let mut dist: HashMap<Node, i32> = HashMap::new();
    let mut prev: HashMap<Node, Vec<Node>> = HashMap::new();
    let mut pq: BinaryHeap<Reverse<(i32, Node)>> = BinaryHeap::new();

    dist.insert(start, 0);
    pq.push(Reverse((0, start)));

    while let Some(Reverse((cur_dist, cur_node))) = pq.pop() {
        if cur_node == end {
            break;
        }
        if let Some(&existing_dist) = dist.get(&cur_node) {
            if existing_dist < cur_dist {
                continue;
            }
        }
        if let Some(neighbors) = graph.get(&cur_node) {
            for &(next, cost) in neighbors.iter() {
                let new_dist = cur_dist + cost;
                if !dist.contains_key(&next) || new_dist < dist[&next] {
                    dist.insert(next, new_dist);
                    prev.insert(next, vec![cur_node]);
                    pq.push(Reverse((new_dist, next)));
                } else if new_dist == dist[&next] {
                    prev.get_mut(&next).unwrap().push(cur_node);
                }
            }
        }
    }

    let mut paths: Vec<Vec<Node>> = Vec::new();
    let mut stack: Vec<(Node, Vec<Node>)> = vec![(end, vec![])];

    while let Some((cur, mut cur_path)) = stack.pop() {
        cur_path.push(cur);
        if cur == start {
            cur_path.reverse();
            paths.push(cur_path);
        } else if let Some(prev_nodes) = prev.get(&cur) {
            for &prev_node in prev_nodes {
                stack.push((prev_node, cur_path.clone()));
            }
        }
    }

    (dist[&end], paths)
}

pub fn part_1() {
    let input: String = util::get_input_part(16, 1);

    let mut grid: Grid = Grid::new();
    let mut start: (i32, i32) = (0, 0);
    let mut end: (i32, i32) = (0, 0);

    for (r, line) in input.lines().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            if ch == 'S' {
                start = (r as i32, c as i32);
                grid.insert((r as i32, c as i32), '.');
            } else if ch == 'E' {
                end = (r as i32, c as i32);
                grid.insert((r as i32, c as i32), '.');
            } else {
                grid.insert((r as i32, c as i32), ch);
            }
        }
    }

    let mut stack: Vec<Node> = Vec::new();
    let mut visited: HashMap<Node, bool> = HashMap::new();
    let mut graph: HashMap<Node, Vec<(Node, i32)>> = HashMap::new();
    stack.push(Node {
        r: start.0,
        c: start.1,
        d: 0,
    });
    while !stack.is_empty() {
        let cur = stack.pop().unwrap();
        let dir = DIRECTIONS[cur.d];
        if visited.contains_key(&cur) {
            continue;
        }
        visited.insert(cur, true);
        let mut neighbors: Vec<(Node, i32)> = Vec::new();
        neighbors.push((
            Node {
                r: cur.r + dir[0],
                c: cur.c + dir[1],
                d: cur.d,
            },
            1,
        ));
        neighbors.push((
            Node {
                r: cur.r,
                c: cur.c,
                d: (cur.d + 1) % 4,
            },
            1000,
        ));
        neighbors.push((
            Node {
                r: cur.r,
                c: cur.c,
                d: (cur.d + 3) % 4,
            },
            1000,
        ));
        for (n, cost) in neighbors.iter() {
            if grid.contains_key(&(n.r, n.c)) {
                if grid[&(n.r, n.c)] == '.' {
                    graph.entry(cur).or_insert(Vec::new()).push((*n, *cost));
                    stack.push(*n);
                }
            }
        }
    }

    let start_node = Node {
        r: start.0,
        c: start.1,
        d: 0,
    };
    let mut min_cost = 0;
    for d in 0..4 {
        let end_node = Node {
            r: end.0,
            c: end.1,
            d: d,
        };

        let (cost, _) = dijkstra_all(start_node, end_node, &graph);
        if min_cost == 0 || cost < min_cost {
            min_cost = cost;
        }
    }
    println!("{}", min_cost);
}

pub fn part_2() {
    let input: String = util::get_input_part(16, 1);

    let mut grid: Grid = Grid::new();
    let mut start: (i32, i32) = (0, 0);
    let mut end: (i32, i32) = (0, 0);

    for (r, line) in input.lines().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            if ch == 'S' {
                start = (r as i32, c as i32);
                grid.insert((r as i32, c as i32), '.');
            } else if ch == 'E' {
                end = (r as i32, c as i32);
                grid.insert((r as i32, c as i32), '.');
            } else {
                grid.insert((r as i32, c as i32), ch);
            }
        }
    }

    let mut stack: Vec<Node> = Vec::new();
    let mut visited: HashMap<Node, bool> = HashMap::new();
    let mut graph: HashMap<Node, Vec<(Node, i32)>> = HashMap::new();
    stack.push(Node {
        r: start.0,
        c: start.1,
        d: 0,
    });
    while !stack.is_empty() {
        let cur = stack.pop().unwrap();
        let dir = DIRECTIONS[cur.d];
        if visited.contains_key(&cur) {
            continue;
        }
        visited.insert(cur, true);
        let mut neighbors: Vec<(Node, i32)> = Vec::new();
        neighbors.push((
            Node {
                r: cur.r + dir[0],
                c: cur.c + dir[1],
                d: cur.d,
            },
            1,
        ));
        neighbors.push((
            Node {
                r: cur.r,
                c: cur.c,
                d: (cur.d + 1) % 4,
            },
            1000,
        ));
        neighbors.push((
            Node {
                r: cur.r,
                c: cur.c,
                d: (cur.d + 3) % 4,
            },
            1000,
        ));
        for (n, cost) in neighbors.iter() {
            if grid.contains_key(&(n.r, n.c)) {
                if grid[&(n.r, n.c)] == '.' {
                    graph.entry(cur).or_insert(Vec::new()).push((*n, *cost));
                    stack.push(*n);
                }
            }
        }
    }

    let start_node = Node {
        r: start.0,
        c: start.1,
        d: 0,
    };
    let end_node = Node {
        r: end.0,
        c: end.1,
        d: 0,
    };

    let (_, paths) = dijkstra_all(start_node, end_node, &graph);
    let mut visited_nodes: HashMap<(i32, i32), bool> = HashMap::new();
    for path in paths.iter() {
        for node in path.iter() {
            visited_nodes.insert((node.r, node.c), true);
        }
    }
    println!("{}", visited_nodes.len());
}
