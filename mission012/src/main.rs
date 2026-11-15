use std::collections::VecDeque;
use std::fs;

use rayon::prelude::*;
use rustc_hash::{FxHashMap, FxHashSet};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let walls = input
        .trim()
        .split(':')
        .map(|s| {
            let mut parts = s.split(',');
            let x = parts.next().unwrap().parse::<i64>().unwrap();
            let y = parts.next().unwrap().parse::<i64>().unwrap();
            let z = parts.next().unwrap().parse::<i64>().unwrap();
            (x, y, z)
        })
        .collect::<FxHashSet<_>>();

    let dirs = [
        (1, -1, 0),
        (-1, 1, 0),
        (1, 0, -1),
        (0, 1, -1),
        (0, -1, 1),
        (-1, 0, 1),
    ];

    // part 1: perform flood fill to find all inner cells
    let mut queue = Vec::new();
    queue.push((0, 0, 0));
    let mut area = FxHashSet::default();
    while let Some(pos) = queue.pop() {
        for (dx, dy, dz) in dirs {
            let p = (pos.0 + dx, pos.1 + dy, pos.2 + dz);
            if !area.contains(&p) && !walls.contains(&p) {
                area.insert(p);
                queue.push(p);
            }
        }
    }
    println!("{}", area.len());

    // Solve parts 2 and 3 simultaneously: Perform n flood fills (one per inner
    // cell) to find all distances to all other cells. Normally, Floyd-Warshall
    // can be used for this, but it has a complexity of O(n^3) and since we have
    // a big number of cells (around 8000 for my input), this would take too
    // long. n flood fills are actually much faster, especially since we can
    // parallelize them.
    let (max, min) = area
        .par_iter()
        .map(|&a| {
            let mut queue = VecDeque::new();
            queue.push_back((0, a));
            let mut distances: FxHashMap<(i64, i64, i64), u64> = FxHashMap::default();
            distances.insert(a, 0);
            while let Some((steps, pos)) = queue.pop_front() {
                for (dx, dy, dz) in dirs {
                    let p = (pos.0 + dx, pos.1 + dy, pos.2 + dz);
                    if area.contains(&p) && !distances.contains_key(&p) {
                        distances.insert(p, steps + 1);
                        queue.push_back((steps + 1, p));
                    }
                }
            }
            let m = *distances.values().max().unwrap();
            (m, m)
        })
        .reduce(
            || (0, u64::MAX),
            |mut acc, m| {
                acc.0 = acc.0.max(m.0);
                acc.1 = acc.1.min(m.1);
                acc
            },
        );
    println!("{max}");
    println!("{min}");
}
