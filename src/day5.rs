use std::fs;
use std::collections::VecDeque;
use std::cell::RefCell;

/// A wrapper around a deque whose first element is the crate on the top of a
/// stack, and whose last element is the crate on the bottom of a stack
struct Stack {
    crates: VecDeque<char>,
}

/// Implement the methods needed to move crates around stacks
impl Stack {
    fn from_str(s: &str) -> Self {
        let mut crates = VecDeque::new();
        for c in s.chars() {
            crates.push_back(c);
        }

        return Self{ crates };
    }

    /// Move the specified number of items from self to the other
    fn move_crates(&mut self, to: &mut Stack, n: i32, preserve_ordering: bool) {
        let mut buffer: VecDeque<char> = VecDeque::new();

        for _ in 0..n {
            // safely assume that all operations are legal
            buffer.push_back(self.crates.pop_front().unwrap());
        }

        for _ in 0..n {
            if preserve_ordering {
                to.crates.push_front(buffer.pop_back().unwrap());
            } else {
                to.crates.push_front(buffer.pop_front().unwrap());
            }
        }
    }
}

fn generate_stacks() -> Vec<RefCell<Stack>> {
    let stacks = vec![
        RefCell::new(Stack::from_str("PGRN")),
        RefCell::new(Stack::from_str("CDGHLBTJ")),
        RefCell::new(Stack::from_str("VSM")),
        RefCell::new(Stack::from_str("PZCRSL")),
        RefCell::new(Stack::from_str("QDWCVLSP")),
        RefCell::new(Stack::from_str("SMDWNTC")),
        RefCell::new(Stack::from_str("PWGDH")),
        RefCell::new(Stack::from_str("VMCSHPLZ")),
        RefCell::new(Stack::from_str("ZGWLFPR")),
    ];
    return stacks;
}

fn generate_test_stacks() -> Vec<RefCell<Stack>> {
    let stacks = vec![
        RefCell::new(Stack::from_str("NZ")),
        RefCell::new(Stack::from_str("DCM")),
        RefCell::new(Stack::from_str("P")),
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

pub fn solve(input_path: &str) {
    let stacks = generate_stacks();
    let input = fs::read_to_string(input_path).unwrap();
    for line in input.lines() {
        if line.contains("move") {
            let (from, to, n) = parse_command(line);
            let from = stacks.get(from - 1).unwrap();
            let to = stacks.get(to - 1).unwrap();

            from.borrow_mut().move_crates(&mut*to.borrow_mut(), n, false);
        }
    }
    for stack in &stacks {
        print!("{}", stack.borrow().crates[0]);
    }
    println!();

    let stacks = generate_stacks();
    let input = fs::read_to_string(input_path).unwrap();
    for line in input.lines() {
        if line.contains("move") {
            let (from, to, n) = parse_command(line);
            let from = stacks.get(from - 1).unwrap();
            let to = stacks.get(to - 1).unwrap();

            from.borrow_mut().move_crates(&mut*to.borrow_mut(), n, true);
        }
    }
    for stack in &stacks {
        print!("{}", stack.borrow().crates[0]);
    }
    println!();

}
