use std::fs;
use std::str::Lines;

#[derive(Clone, PartialEq, Debug)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn delta(&self, other: &Point) -> (i64, i64) {
        return (self.x - other.x, self.y - other.y);
    }
}

#[derive(Debug)]
struct Simulation {
    knots: Vec<Point>, // has at least 2 elements; knots[i] follows knots[i-1]
    trail: Vec<Point>,
}

impl Simulation {
    fn new(n: usize) -> Self {
        assert!(n >= 2);
        let mut knots = vec![];
        for _ in 0..n {
            knots.push(Point { x: 0, y: 0 });
        }
        return Self {
            knots,
            trail: vec![],
        };
    }

    fn add_trail(&mut self, point: &Point) {
        if !self.trail.contains(point) {
            self.trail.push(point.clone());
        }
    }

    fn update_tail(head: &Point, tail: &mut Point) {
        let (xdelta, ydelta) = head.delta(tail);
        match (xdelta, ydelta) {
            // diagonal positions
            (2, 1) | (1, 2) | (2, 2) => {
                tail.x += 1;
                tail.y += 1;
            }
            (-1, 2) | (-2, 1) | (-2, 2) => {
                tail.x -= 1;
                tail.y += 1;
            }
            (-1, -2) | (-2, -1) | (-2, -2) => {
                tail.x -= 1;
                tail.y -= 1;
            }
            (1, -2) | (2, -1) | (2, -2) => {
                tail.x += 1;
                tail.y -= 1;
            }
            // straight positions
            (2, _) => tail.x += 1,
            (-2, _) => tail.x -= 1,
            (_, 2) => tail.y += 1,
            (_, -2) => tail.y -= 1,
            // No need to move
            (-1..=1, -1..=1) => (),
            _ => unreachable!("simulation corrupted"),
        }
    }

    fn update_knots(&mut self) {
        for i in 0..(self.knots.len() - 1) {
            let head = self.knots.get(i).unwrap().clone();
            let tail = self.knots.get_mut(i + 1).unwrap();
            Self::update_tail(&head, tail);
        }

        let tail = self.knots.get(self.knots.len() - 1).unwrap().clone();
        self.add_trail(&tail);
    }

    fn move_head(&mut self, (xdelta, ydelta): (i64, i64)) {
        let head = self.knots.get_mut(0).unwrap();
        head.x += xdelta;
        head.y += ydelta;
        self.update_knots();
    }
}

fn simulate_rope(n: usize, cmds: Lines) {
    let mut sim = Simulation::new(n);
    cmds.for_each(|cmd| {
        let mut tokens = cmd.split(" ");
        let direction = tokens.next().unwrap();
        let count = tokens.next().unwrap();
        let count = count.parse::<usize>().unwrap();

        for _ in 0..count {
            match direction {
                "R" => sim.move_head((1, 0)),
                "U" => sim.move_head((0, 1)),
                "L" => sim.move_head((-1, 0)),
                "D" => sim.move_head((0, -1)),
                _ => unreachable!("Illegal direction!"),
            }
        }
    });

    println!("{}", sim.trail.len());
}

fn main() {
    let inputs = fs::read_to_string("inputs/9.txt").unwrap();

    simulate_rope(2, inputs.lines());
    simulate_rope(10, inputs.lines());
}
