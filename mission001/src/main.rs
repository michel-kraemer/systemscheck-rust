use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let mut disabled: HashMap<u64, u64> = HashMap::new();
    let mut capacities: HashMap<u64, usize> = HashMap::new();

    // part 1
    let mut max = 0;
    let mut max_disabled = 0;
    for l in input.lines() {
        let parts = l.split_ascii_whitespace().collect::<Vec<_>>();
        let t = parts[0][2..].parse::<u64>().unwrap();
        let id = parts[1][1..].parse::<u64>().unwrap();
        if parts[2] == "enabled" {
            let dt = disabled.remove(&id).unwrap();
            max = max.max(t - dt);
        } else {
            disabled.insert(id, t);
            max_disabled = max_disabled.max(disabled.len());
        }
        capacities.insert(t, disabled.len());
    }
    println!("{max}");

    // part 2
    let mut capacities = capacities.into_iter().collect::<Vec<_>>();
    capacities.sort_unstable();
    let mut max_disabled_time = 0;
    for i in 0..capacities.len() - 1 {
        if capacities[i].1 == max_disabled {
            max_disabled_time = max_disabled_time.max(capacities[i + 1].0 - capacities[i].0);
        }
    }
    println!("{max_disabled_time}");
}
