use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
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

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    // parse all rectangles
    let mut rects: Vec<(usize, usize, usize, usize)> = Vec::new();
    for l in input.lines() {
        let cs = l.split_ascii_whitespace().collect::<Vec<_>>();
        let mut min_x = usize::MAX;
        let mut min_y = usize::MAX;
        let mut max_x = 0;
        let mut max_y = 0;
        for c in cs {
            let (x, y) = c.split_once(',').unwrap();
            let x = x.parse::<usize>().unwrap();
            let y = y.parse::<usize>().unwrap();
            min_x = min_x.min(x);
            min_y = min_y.min(y);
            max_x = max_x.max(x);
            max_y = max_y.max(y);
        }
        rects.push((min_x, min_y, max_x, max_y));
    }

    // compute bounding box of entire area
    let mut min_x = usize::MAX;
    let mut min_y = usize::MAX;
    let mut max_x = 0;
    let mut max_y = 0;
    for r in &rects {
        min_x = min_x.min(r.0);
        min_y = min_y.min(r.1);
        max_x = max_x.max(r.2);
        max_y = max_y.max(r.3);
    }

    // create area
    let width = max_x - min_x + 1;
    let height = max_y - min_y + 1;
    let mut grid = vec![b'.'; width * height];

    // draw rectangles
    for r in rects {
        for y in r.1..=r.3 {
            for x in r.0..=r.2 {
                grid[(y - min_y) * width + (x - min_x)] = b'*';
            }
        }
    }

    // find outlines
    for y in 0..height {
        for x in 0..width {
            if grid[y * width + x] != b'*' {
                continue;
            }

            for (dx, dy) in CLOCKWISE {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if !(nx >= 0 && ny >= 0 && (nx as usize) < width && (ny as usize) < height)
                    || grid[ny as usize * width + nx as usize] == b'.'
                {
                    grid[y * width + x] = b'#';
                    break;
                }
            }
        }
    }

    println!("{}", grid.iter().filter(|c| **c == b'#').count());

    // extract polygons
    let mut polygons = Vec::new();
    for y in 0..height {
        for x in 0..width {
            if grid[y * width + x] != b'#' {
                continue;
            }

            let mut tiles = 0;
            let mut queue = vec![(x, y)];
            grid[y * width + x] = b'.';
            while let Some((x, y)) = queue.pop() {
                tiles += 1;
                for (dx, dy) in DIRS {
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;
                    if nx >= 0
                        && ny >= 0
                        && (nx as usize) < width
                        && (ny as usize) < height
                        && grid[ny as usize * width + nx as usize] == b'#'
                    {
                        grid[ny as usize * width + nx as usize] = b'.';
                        queue.push((nx as usize, ny as usize));
                    }
                }
            }

            polygons.push((x + min_x, y + min_y, tiles));
        }
    }

    // run Dijkstra's
    let mut queue = BinaryHeap::new();
    for (i, s) in polygons.iter().enumerate() {
        let mut available = (1 << polygons.len()) - 1;
        available &= !(1 << i);
        queue.push(Reverse((s.0 + s.1 + s.2, (s.0, s.1), available)));
    }
    let mut total_steps = usize::MAX;
    let mut best: HashMap<((usize, usize), usize), usize> = HashMap::new();
    while let Some(Reverse((steps, prev, available))) = queue.pop() {
        if available == 0 {
            total_steps = total_steps.min(steps + prev.0 + prev.1);
            continue;
        }

        let mut m: usize = available;
        while m > 0 {
            // select LSB and reset it
            let k = m.trailing_zeros() as usize;
            m &= m - 1;

            let s = &polygons[k];
            let new_steps = steps + s.0.abs_diff(prev.0) + s.1.abs_diff(prev.1) + s.2;
            let new_available = available & !(1 << k);
            if *best
                .get(&((s.0, s.1), new_available))
                .unwrap_or(&usize::MAX)
                > new_steps
            {
                best.insert(((s.0, s.1), new_available), new_steps);
                queue.push(Reverse((new_steps, (s.0, s.1), new_available)));
            }
        }
    }
    println!("{total_steps}");
}
