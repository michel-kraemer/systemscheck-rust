use std::collections::HashMap;
use std::fs;

fn run1(bytes: &[u8]) -> usize {
    let mut result = 0;
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == 0xFF {
            i += 2;
            result += bytes[i] as usize;
        } else if bytes[i] == 0xFE {
            i += 2;
            result += 3;
        } else {
            result += 1;
        }
        i += 1;
    }
    result
}

fn run2(
    bytes: &[u8],
    start: usize,
    end: usize,
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if let Some(c) = cache.get(&(start, end)) {
        return *c;
    }

    let mut result = 0;
    let mut i = start;
    while i < end {
        if bytes[i] == 0xFF {
            i += 2;
            result += bytes[i] as usize;
        } else if bytes[i] == 0xFE {
            let s = i - bytes[i + 1] as usize;
            i += 2;
            let e = s + bytes[i] as usize;
            result += run2(bytes, s, e, cache);
        } else {
            result += 1;
        }
        i += 1;
    }

    cache.insert((start, end), result);

    result
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let bytes = input
        .trim()
        .split_ascii_whitespace()
        .map(|b| u8::from_str_radix(b, 16).unwrap())
        .collect::<Vec<_>>();

    // part 1
    println!("{}", run1(&bytes));

    // part 2
    println!("{}", run2(&bytes, 0, bytes.len(), &mut HashMap::new()));
}
