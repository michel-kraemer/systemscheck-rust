use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs;

#[derive(PartialEq, Eq)]
struct Robot<'a> {
    pos: &'a str,
    next_move_at: u64,
}

impl Ord for Robot<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.next_move_at.cmp(&other.next_move_at)
    }
}

impl PartialOrd for Robot<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let (directions, waste) = input.trim().split_once("\n\n").unwrap();

    // parse
    let mut incoming_edge_counts: HashMap<&str, u64> = HashMap::new();
    let mut mapping: HashMap<&str, (&str, u64)> = HashMap::new();
    for l in directions.lines() {
        let from = &l[0..3];
        let to = &l[l.len() - 3..];

        let i = l.find('[').unwrap();
        let j = l.rfind(']').unwrap();
        let weight = l[i + 1..j].parse::<u64>().unwrap();

        incoming_edge_counts.entry(from).or_insert(0);
        *incoming_edge_counts.entry(to).or_default() += 1;
        mapping.insert(from, (to, weight));
    }

    let mut rubbish: HashMap<&str, u64> = HashMap::new();
    for l in waste.lines() {
        let (to, amt) = l.split_once(" ").unwrap();
        let amt = amt.parse::<u64>().unwrap();
        rubbish.insert(to, amt);
    }

    // part 1
    let mut min = u64::MAX;
    for i in incoming_edge_counts.iter().filter(|i| *i.1 == 0) {
        let mut pos = *i.0;
        let mut total = 0;
        while let Some((next, weight)) = mapping.get(pos) {
            total += weight;
            pos = *next;
        }
        min = min.min(total);
    }
    println!("{min}");

    // part 2
    let mut queue = BinaryHeap::new();
    for i in incoming_edge_counts.iter().filter(|i| *i.1 == 0) {
        queue.push(Reverse(Robot {
            pos: i.0,
            next_move_at: mapping.get(i.0).unwrap().1 + *rubbish.get(i.0).unwrap_or(&0),
        }));
    }

    'outer: loop {
        // drain all robots from the queue that need to move now (at next_t)
        let mut next_robots = vec![queue.pop().unwrap().0];
        let next_t = next_robots[0].next_move_at;
        while !queue.is_empty() && queue.peek().unwrap().0.next_move_at == next_t {
            next_robots.push(queue.pop().unwrap().0);
        }

        // move robots
        let mut to_clear = HashSet::new();
        for mut r in next_robots {
            r.pos = mapping.get(r.pos).unwrap().0;
            to_clear.insert(r.pos);

            let next = mapping.get(r.pos);
            if let Some(next) = next {
                r.next_move_at = next_t + next.1 + *rubbish.get(r.pos).unwrap_or(&0);
            } else {
                // first robot arrived at the root
                println!("{next_t}");
                break 'outer;
            }

            queue.push(Reverse(r));
        }

        // drain rubbish so no future robot has to wait
        for tc in to_clear {
            rubbish.remove(tc);
        }
    }
}
