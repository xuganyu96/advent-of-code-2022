use std::fs;
use std::collections::VecDeque;

/// a single stack
struct Stack {
    crates: VecDeque<char>,
}

impl Stack {
    fn from_slice(s: &[char]) -> Self {
        let mut crates = VecDeque::new();
        for c in s {
            crates.push_back(*c);
        }

        return Self{ crates };
    }

    /// Move the specified number of items from self to the other
    fn move_crates(&mut self, to: &mut Stack, n: i32) {
        let mut buffer: VecDeque<char> = VecDeque::new();

        for _ in 0..n {
            // safely assume that all operations are legal
            buffer.push_back(self.crates.pop_front().unwrap());
        }

        for _ in 0..n {
            to.crates.push_front(buffer.pop_back().unwrap());
        }
    }

    fn pop_crates(&mut self, n: i32) -> VecDeque<char> {
        let mut buffer = VecDeque::new();
        
        for _ in 0..n {
            buffer.push_back(self.crates.pop_front().unwrap());
        }

        return buffer;
    }

    fn push_crates(&mut self, mut buffer: VecDeque<char>, preserve_ordering: bool) {
        let n = buffer.len();
        for _ in 0..n {
            if preserve_ordering {
                self.crates.push_front(buffer.pop_back().unwrap());
            } else {
                self.crates.push_front(buffer.pop_front().unwrap());
            }
        }
    }
}

fn generate_stacks() -> Vec<Stack> {
    let s1 = Stack::from_slice(&vec!['P', 'G', 'R', 'N']);
    let s2 = Stack::from_slice(&vec!['C', 'D', 'G', 'F', 'L', 'B', 'T', 'J']);
    let s3 = Stack::from_slice(&vec!['V', 'S', 'M']);
    let s4 = Stack::from_slice(&vec!['P', 'Z', 'C', 'R', 'S', 'L']);
    let s5 = Stack::from_slice(&vec!['Q', 'D', 'W', 'C', 'V', 'L', 'S', 'P']);
    let s6 = Stack::from_slice(&vec!['S', 'M', 'D', 'W', 'N', 'T', 'C']);
    let s7 = Stack::from_slice(&vec!['P', 'W', 'G', 'D', 'H']);
    let s8 = Stack::from_slice(&vec!['V', 'M', 'C', 'S', 'H', 'P', 'L', 'Z']);
    let s9 = Stack::from_slice(&vec!['Z', 'G', 'W', 'L', 'F', 'P', 'R']);

    let stacks = vec![s1, s2, s3, s4, s5, s6, s7, s8, s9];
    return stacks;
}

fn generate_test_stacks() -> Vec<Stack> {
    let stacks = vec![
        Stack::from_slice(&vec!['N', 'Z']),
        Stack::from_slice(&vec!['D', 'C', 'M']),
        Stack::from_slice(&vec!['P']),
    ];

    return stacks;
}

fn parse_command(line: &str) -> (usize, usize, i32) {
    let mut tokens = line.split(" ");
    tokens.next(); // "move"
    let n: i32 = tokens.next().unwrap().parse().unwrap();
    tokens.next(); // "from"
    let from: usize = tokens.next().unwrap().parse().unwrap();
    tokens.next(); // "to"
    let to: usize = tokens.next().unwrap().parse().unwrap();

    return (from, to, n);
}

/// If I have a list of Stacks and I want to mutate two distinct elements
/// within the same list. How do I do that?
pub fn solve(input: &str) {
    let mut stacks = generate_stacks();
    let input = fs::read_to_string(input).unwrap();
    for line in input.lines() {
        if line.contains("move") {
            let (fromi, toi, n) = parse_command(line);
            let buffer = stacks.get_mut(fromi - 1).unwrap().pop_crates(n);
            stacks.get_mut(toi - 1).unwrap().push_crates(buffer, false);
        }
    }
    for stack in &stacks {
        print!("{}", stack.crates[0]);
    }
    println!();

    let mut stacks = generate_stacks();
    for line in input.lines() {
        if line.contains("move") {
            let (fromi, toi, n) = parse_command(line);
            let buffer = stacks.get_mut(fromi - 1).unwrap().pop_crates(n);
            stacks.get_mut(toi - 1).unwrap().push_crates(buffer, true);
        }
    }
    for stack in &stacks {
        print!("{}", stack.crates[0]);
    }
    println!();
}

