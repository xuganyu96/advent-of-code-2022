use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/10.txt").unwrap();
    let mut levels: Vec<i64> = vec![0];
    let mut level: i64 = 1;

    input.lines().for_each(|line| {
        let mut tokens = line.split(" ");
        let cmd = tokens.next().unwrap();

        match cmd {
            "addx" => {
                let delta = tokens.next().unwrap().parse::<i64>().unwrap();
                levels.push(level);
                level += delta;
                levels.push(level);
            }
            "noop" => levels.push(level),
            _ => unreachable!("Illegal command!"),
        }
    });

    let cycles: [i64; 6] = [20, 60, 100, 140, 180, 220];
    let mut strength: i64 = 0;

    for cycle in &cycles {
        let level = levels.get((cycle - 1) as usize).unwrap();
        strength += level * cycle;
    }

    println!("{strength}");

    for r in 0..6 {
        for c in 0..40 {
            let cycle = c + 40 * r;
            let mid_pos = levels.get(cycle).unwrap();
            if c as i64 >= mid_pos - 1 && c as i64 <= mid_pos + 1 {
                print!("##");
            } else {
                print!("  ");
            }
        }
        println!();
    }
}
