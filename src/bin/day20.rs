//! Assume there are no duplicate in the number
use std::fs;

fn parse_input(inputs: &str) -> Vec<(usize, i64)> {
    return inputs.lines().enumerate()
        .map(|(i, num)| (i, num.parse::<i64>().unwrap())).collect();
}

/// move the number at "from" forward by "delta" units. "from" is an index;
/// "delta" can be negative, 
fn move_elem<T: Copy>(arr: &mut Vec<T>, from: usize, delta: i64) {
    let dst = (from as i64 + delta).rem_euclid(arr.len() as i64 - 1);
    let removed = arr.remove(from);
    arr.insert(dst as usize, removed);
}

fn find_elem<T: Copy + Eq>(arr: &[T], target: &T) -> Option<usize> {
    for (i, elem) in arr.iter().enumerate() {
        if elem == target { return Some(i); }
    }
    return None;
}

fn mix(inputs: &str, key: i64, rounds: usize) -> i64 {
    let nums_original: Vec<(usize, i64)> = parse_input(&inputs).iter().map(|(i, x)| (*i, x * key)).collect();
    let mut nums_mut: Vec<(usize, i64)> = parse_input(&inputs).iter().map(|(i, x)| (*i, x * key)).collect();

    let mut sum = 0;
    for round in 0..rounds {
        println!("{round}");
        nums_original.iter()
            .for_each(|elem| {
                let (_, num) = elem;
                let cur_loc = find_elem(&nums_mut, elem).unwrap();
                move_elem(&mut nums_mut, cur_loc, *num);
            });

        let mut zero_loc = 0;
        nums_mut.iter().enumerate()
            .for_each(|(i, (_, num))| {
                if *num == 0 {
                    zero_loc = i;
                }
            });

        let keypoints = [1000, 2000, 3000];
        sum = keypoints.iter()
            .map(|p| {
                nums_mut.get((zero_loc + p) % nums_mut.len()).unwrap().1
            })
            .sum::<i64>();
    }
    return sum;
}

fn main() {
    let inputs = fs::read_to_string("inputs/20.txt").unwrap();
    let sum = mix(&inputs, 1, 1);
    println!("{sum}");

    let sum = mix(&inputs, 811589153, 10);
    println!("{sum}");
}
