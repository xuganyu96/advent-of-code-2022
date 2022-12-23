use std::fs;
use std::collections::{ HashSet, HashMap };

#[derive(Debug,Eq,PartialEq,Hash,Clone)]
struct Point {
    x: i32, y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        return Self { x, y };
    }
}

#[derive(Debug,Clone,Hash,Eq,PartialEq)]
enum Dir {
    N, S, W, E,
}

#[derive(Debug)]
struct Group {
    elves: HashSet<Point>,
}

impl Group {
    fn new() -> Self {
        let elves = HashSet::new();

        return Self { elves };
    }

    /// on round 0 consider N, S, W, E
    /// on round 1 consider S, W, E, N
    /// on round 2 ocnsdier W, E, N, S
    /// on round 3 consider E, N, S, W
    fn propose_next(&self, elf: &Point, round: usize) -> (Point, Option<Dir>) {
        let (mut north_empty, mut south_empty, mut west_empty, mut east_empty) = (true, true, true, true);
        for (x_delta, y_delta) in [
            (0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1)
        ] {
            let next_pos = Point::new(elf.x + x_delta, elf.y + y_delta);
            if self.elves.get(&next_pos).is_some() {
                // found an adjacent elf
                if y_delta == -1 { north_empty = false; }
                if y_delta == 1 { south_empty = false; }
                if x_delta == 1 { east_empty = false; }
                if x_delta == -1 { west_empty = false; }
            }
        }

        if north_empty && south_empty && west_empty && east_empty {
            return (elf.clone(), None);
        }
        for i in round..=round+3 {
            let j = i.rem_euclid(4);
            match j {
                0 if north_empty => {
                    return (Point::new(elf.x, elf.y - 1), Some(Dir::N));
                },
                1 if south_empty => {
                    return (Point::new(elf.x, elf.y + 1), Some(Dir::S));
                },
                2 if west_empty => {
                    return (Point::new(elf.x - 1, elf.y), Some(Dir::W));
                },
                3 if east_empty => {
                    return (Point::new(elf.x + 1, elf.y), Some(Dir::E));
                },
                _ => (),
            }
        }
        return (elf.clone(), None);
    }

    /// This time I will immutably step, also return the number of elves that moved
    fn step(&self, round: usize) -> (Self, usize) {
        let mut moves: Vec<(Point, Point)> = vec![]; // (old_pos, new_pos)
        let mut counts: HashMap<Point, i32> = HashMap::new();
        let mut count_moves = 0;

        self.elves.iter()
            .for_each(|elf| {
                let (next_pos, _) = self.propose_next(elf, round);
                moves.push((elf.clone(), next_pos.clone()));
                if counts.get(&next_pos).is_none() {
                    counts.insert(next_pos.clone(), 1);
                    if next_pos != elf.clone() {
                        count_moves += 1;
                    }
                } else {
                    let count = counts.get(&next_pos).unwrap();
                    counts.insert(next_pos.clone(), count + 1);
                }
            });

        let mut new_group = Self::new();
        moves.iter()
            .for_each(|(old_pos, new_pos)| {
                if *counts.get(new_pos).unwrap() == 1 {  // must have been inserted before
                    new_group.elves.insert(new_pos.clone());
                } else {
                    new_group.elves.insert(old_pos.clone());
                }
            });
        return (new_group, count_moves);
    }

    /// x_min, x_max, y_min, y_max, all inclusive
    fn get_bounds(&self) -> (i32, i32, i32, i32) {
        let x_min = self.elves.iter()
            .map(|elf| elf.x)
            .min().unwrap();
        let x_max = self.elves.iter()
            .map(|elf| elf.x)
            .max().unwrap();
        let y_min = self.elves.iter()
            .map(|elf| elf.y)
            .min().unwrap();
        let y_max = self.elves.iter()
            .map(|elf| elf.y)
            .max().unwrap();
        return (x_min, x_max, y_min, y_max);
    }

    fn render(&self) {
        let (x_min, x_max, y_min, y_max) = self.get_bounds();
        let mut grid_str = String::new();
        
        for r in y_min..=y_max {
            for c in x_min..=x_max {
                let pos = Point::new(c, r);
                if self.elves.contains(&pos) {
                    grid_str.push_str("#");
                } else {
                    grid_str.push_str(".");
                }
            }
            grid_str.push_str("\n");
        }

        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        print!("{grid_str}");
    }

    fn count_spread(&self) -> usize {
        let (x_min, x_max, y_min, y_max) = self.get_bounds();
        let mut grid_str = String::new();
        let mut count = 0;
        
        for r in y_min..=y_max {
            for c in x_min..=x_max {
                let pos = Point::new(c, r);
                if !self.elves.contains(&pos) {
                    count += 1;
                }
            }
            grid_str.push_str("\n");
        }

        return count;
    }

    fn from_inputs(inputs: &str) -> Self {
        let mut group = Group::new();
        for (y, line) in inputs.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    group.elves.insert(Point::new(x as i32, y as i32));
                }
            }
        }
        return group;
    }
}

fn main() {
    let inputs = fs::read_to_string("inputs/23.txt").unwrap();

    // part 1
    let mut group = Group::from_inputs(&inputs);
    for round in 0..10 {
        let (next_group, _) = group.step(round);
        group = next_group;
    }
    println!("{}", group.count_spread());

    // part 2
    let mut group = Group::from_inputs(&inputs);
    for round in 0.. {
        let (next_group, nmoves) = group.step(round);
        if nmoves == 0 {
            println!("{}", round + 1);
            break;
        } else {
            group = next_group;
        }
    }
}
