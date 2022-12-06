use std::fs;
use std::collections::HashMap;

fn has_repeat(s: &str) -> bool {
    let mut map: HashMap<char, i32> = HashMap::new();
    for c in s.chars() {
        let count = map.get(&c);
        match count {
            Some(v) => {
                map.insert(c, v + 1);
            },
            None => {
                map.insert(c, 1);
            }
        }
    }

    for c in s.chars() {
        let count = map.get(&c).unwrap();
        if *count > 1 {
            return true;
        }
    }
    return false;
}

pub fn solve(input: &str) {
    let input = fs::read_to_string(input).unwrap();

    for i in 0..(input.len() - 3) {
        let slice = &input[i..(i+4)];
        if !has_repeat(slice) {
            println!("{}", i + 4);
            break;
        }
    }

    for i in 0..(input.len() - 13) {
        let slice = &input[i..(i+14)];
        if !has_repeat(slice) {
            println!("{}", i + 14);
            break;
        }
    }
}
