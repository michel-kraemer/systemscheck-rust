use std::cmp::Reverse;
use std::collections::{BinaryHeap, VecDeque};
use std::fs;

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

fn cell_costs(x: i32, y: i32, grid: &[u8], width: usize, height: usize) -> i64 {
    for (dx, dy) in CLOCKWISE {
        let nx = x + dx;
        let ny = y + dy;
        if nx >= 0
            && (nx as usize) < width
            && ny >= 0
            && (ny as usize) < height
            && grid[ny as usize * width + nx as usize] == b'#'
        {
            return 2;
        }
    }
    1
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let lines = input.lines().collect::<Vec<_>>();
    let width = lines[0].len();
    let height = lines.len();
    let grid = lines
        .into_iter()
        .flat_map(|l| l.as_bytes())
        .copied()
        .collect::<Vec<_>>();

    // part 1
    let mut queue = VecDeque::new();
    queue.push_back((0, 0, 0));
    let mut seen = vec![false; grid.len()];
    while let Some((x, y, steps)) = queue.pop_front() {
        if grid[y * width + x] == b'*' {
            println!("{steps}");
            break;
        }

        for (dx, dy) in DIRS {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0
                && (nx as usize) < width
                && ny >= 0
                && (ny as usize) < height
                && grid[ny as usize * width + nx as usize] != b'#'
                && !seen[ny as usize * width + nx as usize]
            {
                seen[ny as usize * width + nx as usize] = true;
                queue.push_back((nx as usize, ny as usize, steps + 1));
            }
        }
    }

    // part 2
    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0, 0, 0)));
    let mut costs = vec![i64::MAX; grid.len()];
    costs[0] = 0;

    while let Some(Reverse((c, x, y))) = queue.pop() {
        if grid[y * width + x] == b'*' {
            println!("{c}");
            break;
        }

        for (dx, dy) in DIRS {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0
                && (nx as usize) < width
                && ny >= 0
                && (ny as usize) < height
                && grid[ny as usize * width + nx as usize] != b'#'
            {
                let new_costs = c + cell_costs(nx, ny, &grid, width, height);
                if costs[ny as usize * width + nx as usize] > new_costs {
                    costs[ny as usize * width + nx as usize] = new_costs;
                    queue.push(Reverse((new_costs, nx as usize, ny as usize)));
                }
            }
        }
    }
}
