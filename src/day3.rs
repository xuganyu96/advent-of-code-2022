use std::fs;

struct Rucksack {
    first_priorities: Vec<i32>,  // priorities of the first compartment
    second_priorities: Vec<i32>,  // pirorities of the second compartment
}

impl Rucksack {
    fn get_priority(c: u8) -> i32 {
        return match c {
            97..=122 => (c as i32) - 97 + 1,  // a-z maps to 1-26
            65..=90 => (c as i32) - 65 + 27,  // A-Z maps to 27-52
            _ => unreachable!("illegal input"),
        };
    }

    fn from_line(line: &str) -> Self {
        let mut first_priorities: Vec<i32> = vec![];
        let mut second_priorities: Vec<i32> = vec![];
        
        assert!(line.len() % 2 == 0);
        for (i, c) in line.as_bytes().iter().enumerate() {
            if i < line.len() / 2 {
                first_priorities.push(Self::get_priority(*c));
            } else {
                second_priorities.push(Self::get_priority(*c));
            }
        }
        return Self{ first_priorities, second_priorities };
    }

    fn get_common_type_priority(&self) -> i32 {
        for priority in 1..=52 {
            if self.first_priorities.contains(&priority) && self.second_priorities.contains(&priority) {
                return priority;
            }
        }
        return 0;
    }

    fn has_type_priority(&self, priority: i32) -> bool {
        return self.first_priorities.contains(&priority) || self.second_priorities.contains(&priority);
    }
}

fn find_badge_from_lines(l1: &str, l2: &str, l3: &str) -> i32 {
    let (sack1, sack2, sack3) = (
        Rucksack::from_line(l1),
        Rucksack::from_line(l2),
        Rucksack::from_line(l3),
    );
    for priority in 1..=52 {
        if sack1.has_type_priority(priority) 
            && sack2.has_type_priority(priority) 
            && sack3.has_type_priority(priority) {
            return priority;
        }
    };
    return 0;
}

pub fn solve(input: &str) {
    let input = fs::read_to_string(input).unwrap();
    // part 1: finding common priority across the two compartments
    let sum = input.lines()
        .map(|line| {
            let sack = Rucksack::from_line(line);
            return sack.get_common_type_priority();
        })
        .sum::<i32>();
    println!("{sum}");

    // part 2: finding common priority across every 3 lines
    let lines: Vec<&str> = input.lines().collect();
    let mut i = 0;
    let mut sum = 0;
    while i + 2 < lines.len() {
        sum += find_badge_from_lines(lines[i], lines[i+1], lines[i+2]);
        i += 3;
    }
    println!("{sum}");
}
