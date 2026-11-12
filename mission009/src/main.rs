use std::cmp::Reverse;
use std::collections::{BinaryHeap, VecDeque};
use std::fs;

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
    let lines = input.lines().collect::<Vec<_>>();
    let width = lines[0].split_ascii_whitespace().count();
    let height = lines.len();
    let mut grid = lines
        .into_iter()
        .flat_map(|l| l.split_ascii_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut start = (0, 0);
    let mut end = (0, 0);
    for y in 0..height {
        for x in 0..width {
            let i = y * width + x;
            if grid[i] == "^" {
                start = (x, y);
                grid[i] = "0";
            } else if grid[i] == "*" {
                end = (x, y);
                grid[i] = "0";
            }
        }
    }

    let grid = grid
        .into_iter()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    // part 1 - flood fill
    let mut queue = VecDeque::new();
    let mut seen = vec![false; width * height];
    queue.push_back((start.0, start.1, 0));
    seen[start.1 * width + start.0] = true;
    while let Some((x, y, steps)) = queue.pop_front() {
        if (x, y) == end {
            println!("{steps}");
            break;
        }

        for (dx, dy) in CLOCKWISE {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0
                && ny >= 0
                && (nx as usize) < width
                && (ny as usize) < height
                && grid[ny as usize * width + nx as usize].abs_diff(grid[y * width + x]) <= 1
                && !seen[ny as usize * width + nx as usize]
            {
                seen[ny as usize * width + nx as usize] = true;
                queue.push_back((nx as usize, ny as usize, steps + 1));
            }
        }
    }

    // part 2 - Dijkstra's
    let mut queue = BinaryHeap::new();
    let mut best = vec![u32::MAX; width * height];
    queue.push(Reverse((0, end.0, end.1)));
    best[end.1 * width + end.0] = 0;
    while let Some(Reverse((cost, x, y))) = queue.pop() {
        if (x, y) == start {
            println!("{cost}");
            break;
        }

        for (dx, dy) in CLOCKWISE {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0
                && ny >= 0
                && (nx as usize) < width
                && (ny as usize) < height
                && grid[ny as usize * width + nx as usize].abs_diff(grid[y * width + x]) <= 1
            {
                let uphill = grid[ny as usize * width + nx as usize] > grid[y * width + x];
                let new_cost = if uphill { cost + 5 } else { cost + 1 };
                if best[ny as usize * width + nx as usize] > new_cost {
                    best[ny as usize * width + nx as usize] = new_cost;
                    queue.push(Reverse((new_cost, nx as usize, ny as usize)));
                }
            }
        }
    }
}
