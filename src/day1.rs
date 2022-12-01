use std::fs;
use std::collections::BinaryHeap;


pub fn solve(input: &str) {
    let input = fs::read_to_string(input).unwrap();
    let mut sums: Vec<i32> = vec![];
    let mut sum = 0;
    let mut max = 0;
    let mut maxes: BinaryHeap<i32> = BinaryHeap::new();
    
    for line in input.lines() {
        // if line is empty, append the current calorie to sums, and set it to
        // 0; else parse it into integer then add to the current calories
        if line.len() == 0 {
            if sum > max {
                max = sum;
            }
            sums.push(sum);
            maxes.push(sum);
            sum = 0;
        } else {
            let item_cal: i32 = line.parse().unwrap();
            sum += item_cal;
        }
    }
    
    let mut max_sum = 0;
    max_sum += maxes.pop().unwrap();
    println!("{max_sum}");

    max_sum += maxes.pop().unwrap();
    max_sum += maxes.pop().unwrap();

    println!("{max_sum}");
}
