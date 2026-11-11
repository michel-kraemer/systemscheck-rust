use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let mut load_types: HashMap<&[u8], u64> = HashMap::new();
    let mut total_part1 = 0;
    for l in input.lines() {
        let mut parts = l.split_ascii_whitespace().skip(1);
        let input = parts.next().unwrap().parse::<u64>().unwrap();
        let output = parts.next().unwrap().parse::<u64>().unwrap();
        let efficiency = 100 * output / input;
        if efficiency < 97 {
            total_part1 += efficiency;

            let pip = l.find('|').unwrap() + 1;
            let id = &l[pip..pip + 9];
            for lt in id.as_bytes().chunks(3) {
                *load_types.entry(lt).or_default() += efficiency;
            }
        }
    }

    println!("{total_part1}");
    println!("{}", load_types.values().max().unwrap());
}
