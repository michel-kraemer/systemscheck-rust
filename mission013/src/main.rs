use std::fs;

use rustc_hash::FxHashSet;

// Right, Down, Left, Up
pub const DIRS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

pub const CLOCKWISE: [(i32, i32); 8] = [
    (1, 0),   // →
    (1, 1),   // ↘︎
    (0, 1),   // ↓
    (-1, 1),  // ↙︎
    (-1, 0),  // ←
    (-1, -1), // ↖︎
    (0, -1),  // ↑
    (1, -1),  // ↗︎
];

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let coords = input
        .trim()
        .split(':')
        .map(|s| {
            let mut parts = s.split(',');
            let x = parts.next().unwrap().parse::<i32>().unwrap();
            let y = parts.next().unwrap().parse::<i32>().unwrap();
            (x, y)
        })
        .collect::<FxHashSet<_>>();

    // part 1
    let mut total = 0;
    let mut seen = FxHashSet::default();
    for &c in &coords {
        if !seen.insert(c) {
            // we've already seen this coordinate
            continue;
        }

        // this is a new coordinate, so we increase the number of islands
        total += 1;

        // flood-fill the rest of the island
        let mut queue = Vec::new();
        queue.push(c);
        while let Some(pos) = queue.pop() {
            for (dx, dy) in CLOCKWISE {
                let nx = pos.0 + dx;
                let ny = pos.1 + dy;
                if coords.contains(&(nx, ny)) && seen.insert((nx, ny)) {
                    queue.push((nx, ny));
                }
            }
        }
    }
    println!("{total}");

    // part 2
    let mut borders = 0;
    for &c in &coords {
        for (dx, dy) in DIRS {
            let nx = c.0 + dx;
            let ny = c.1 + dy;
            if !coords.contains(&(nx, ny)) {
                borders += 1;
            }
        }
    }
    println!("{borders}");
}
