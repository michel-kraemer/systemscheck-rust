use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");

    let mut asteroids = Vec::new();
    for l in input.lines() {
        asteroids.push(
            l.split(&[' ', ','])
                .map(|v| v.parse::<i64>().unwrap())
                .collect::<Vec<_>>(),
        );
    }

    let mut total1 = 0;
    let mut min = u64::MAX;
    for shift in -10..=10 {
        let mut collisions = 0;
        for a in &asteroids {
            // compute intersection point between asteroid and ship
            if (shift - a[0]) % a[2] == 0 {
                let t = (shift - a[0]) / a[2];
                let ay = a[1] + t * a[3];
                if ay == t {
                    // there is a discrete intersection point and the collision
                    // time is also discrete
                    collisions += 1;
                }
            }
        }

        min = min.min(collisions);

        // part 1
        if shift == 0 {
            total1 = collisions;
        }
    }

    println!("{total1}");
    println!("{min}");
}
