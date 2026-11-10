use std::collections::{HashSet, VecDeque};
use std::fs;

fn count_paths(pos: usize, relay: &[(u64, u64)], seen: &mut [bool]) -> usize {
    if pos == relay.len() - 1 {
        return 1;
    }

    let neighbors = relay
        .iter()
        .enumerate()
        .filter(|(i, _)| !seen[*i])
        .filter(|(_, r)| r.0.abs_diff(relay[pos].0) + r.1.abs_diff(relay[pos].1) <= 5)
        .collect::<Vec<_>>();
    let mut result = 0;
    for (n, _) in neighbors {
        seen[n] = true;
        result += count_paths(n, relay, seen);
        seen[n] = false;
    }

    result
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let (stations, messages) = input.trim().split_once("\n\n").unwrap();
    let messages = messages
        .split(',')
        .map(|m| m.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    // part 1
    let mut relay = Vec::new();
    for l in stations.lines() {
        let mut parts = l.split_ascii_whitespace().skip(1);
        let x = parts.next().unwrap()[1..].parse::<u64>().unwrap();
        let y = parts.next().unwrap()[1..].parse::<u64>().unwrap();
        relay.push((x, y));
    }

    let mut seen = vec![false; relay.len()];
    seen[0] = true;
    println!("{}", count_paths(0, &relay, &mut seen));

    // part 2
    let mut queues: Vec<VecDeque<u64>> = vec![VecDeque::new(); relay.len()];
    let mut seen: Vec<HashSet<u64>> = vec![HashSet::new(); relay.len()];
    queues[0].extend(&messages);
    seen[0].extend(messages);

    'outer: loop {
        // find next station with non-empty queue
        for i in 0..queues.len() {
            if queues[i].is_empty() {
                continue;
            }

            // the last station does not send anymore
            if i == queues.len() - 1 {
                break 'outer;
            }

            // find neighbors in reach
            let mut neighbors = relay
                .iter()
                .enumerate()
                .filter(|(j, _)| *j != i)
                .map(|(j, r)| (j, r.0.abs_diff(relay[i].0) + r.1.abs_diff(relay[i].1)))
                .filter(|(_, d)| *d <= 5)
                .collect::<Vec<_>>();

            // sort neighbors
            neighbors.sort_by(|n1, n2| {
                n1.1.cmp(&n2.1).then(
                    queues[n1.0]
                        .len()
                        .cmp(&queues[n2.0].len())
                        .then(n1.0.cmp(&n2.0)),
                )
            });

            // try to send all chunks in queue
            let mut sent = false;
            while let Some(chunk) = queues[i].pop_front() {
                // select a neighbor that accepts the chunk
                let mut dest = usize::MAX;
                for _ in 0..neighbors.len() {
                    if !seen[neighbors[0].0].contains(&chunk) {
                        dest = neighbors[0].0;
                    }
                    neighbors.rotate_left(1);
                    if dest != usize::MAX {
                        break;
                    }
                }

                if dest != usize::MAX {
                    // send chunk to dest
                    queues[dest].push_back(chunk);
                    seen[dest].insert(chunk);
                    sent = true;
                } else {
                    // chunk was not accepted by any neighbor - discard it
                }
            }

            if sent {
                // queue was completely sent - start from the beginning
                break;
            }

            // queue was completely drained - continue with the next best queue
        }
    }

    let last_queue = queues.last().unwrap().iter().copied().collect::<Vec<_>>();
    let sum = last_queue[last_queue.len() - 5..].iter().sum::<u64>();
    println!("{}", sum * last_queue.len() as u64);
}
