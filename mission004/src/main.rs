use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    // part 1
    let mut freq: HashMap<(u8, u8), u64> = HashMap::new();
    let mut total = 0;
    for l in input.lines() {
        let first = l.find('|').unwrap();
        let last = l.rfind('|').unwrap();

        let expected_checksum = l[last + 1..].parse::<u64>().unwrap();

        let actual_checksum = l.as_bytes()[0..last + 1]
            .iter()
            .fold(0u64, |a, b| a ^ *b as u64);

        if actual_checksum != expected_checksum {
            for c in l.as_bytes()[first + 1..last].chunks(2) {
                *freq.entry((c[0], c[1])).or_default() += 1;
            }
            total += expected_checksum;
        }
    }
    println!("{total}");

    // part 2
    let mut freq = freq.values().collect::<Vec<_>>();
    freq.sort_unstable();
    println!("{}", freq[freq.len() - 2] * freq[freq.len() - 1]);
}
