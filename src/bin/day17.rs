//! Day 17: Pyroclastic Flow
//! Grid is 7 units wide, each new rock spawns such that its left edge is 2
//! units from the wall, and its bottom edge is 3 units from the the highest
//! rock or floor. After a rock is spawned, it is first pushed by the stream
//! then falls down.
use std::fs;
use std::collections::HashSet;

#[derive(Debug)]
enum Shape {
    Hori, Cross, Corner, Verti, Square
}

enum Dir { Left, Right, Down }

#[derive(Clone,Debug,Hash,Eq,PartialEq)]
struct Point { x: i64, y: i64 }


impl Point {
    fn new(x: i64, y: i64) -> Self {
        return Self { x, y };
    }
}

struct Simulation {
    peak: i64,  // level of the highest rock; level starts at 0
    width: i64,
    stable_rocks: HashSet<Point>,
    moving_rocks: HashSet<Point>,
}

impl Simulation {
    fn new(width: i64) -> Self {
        return Self {
            peak: -1, width, 
            stable_rocks: HashSet::new(), 
            moving_rocks: HashSet::new(),
        };
    }

    /// Check that the set of rocks is not out of bounds: too left, too right,
    /// lower than the floor
    fn is_in_bounds(&self, rocks: &HashSet<Point>) -> bool {
        if rocks.len() == 0 {
            return true;   // empty set is trivially within bounds
        }
        let (mut x_min, mut x_max, mut y_min) = (0, 0, 0);
        
        rocks.iter().for_each(|rock| {
            x_min = x_min.min(rock.x);
            x_max = x_max.max(rock.x);
            y_min = y_min.min(rock.y);
        });

        return (x_min >= 0) && (x_max < self.width) && (y_min >= 0);
    }

    /// Check that the input set of rocks does not collide with existing set
    /// of stable rocks
    fn is_colliding(&self, rocks: &HashSet<Point>) -> bool {
        let intersection: Vec<&Point> = rocks.intersection(&self.stable_rocks).collect();
        return intersection.len() > 0;
    }

    fn get_next_move(&self, dir: Dir) -> HashSet<Point> {
        let (x_delta, y_delta) = match dir {
            Dir::Left => (-1, 0),
            Dir::Right => (1, 0),
            Dir::Down => (0, -1),
        };
        let mut after_move = HashSet::new();
        self.moving_rocks.iter().for_each(|rock| {
            after_move.insert(Point::new(rock.x + x_delta, rock.y + y_delta));
        });
        
        return after_move;
    }

    /// Given that "new" is a legal next move, commit the move by replacing
    /// self.unstable_rocks
    fn commit_move(&mut self, new: HashSet<Point>) {
        self.moving_rocks = new;
    }

    /// Assuming that the moving rocks has landed, transition it into the set
    /// of stable rocks and update the peak
    fn stabilize_moving_rocks(&mut self) {
        self.moving_rocks.drain().for_each(|rock| {
            self.peak = self.peak.max(rock.y);
            self.stable_rocks.insert(rock);
        });
    }

    /// Assuming that self.moving_rocks is empty, create a new set and replace
    /// it
    fn spawn(&mut self, shape: &Shape) {
        let rocks: Vec<Point> = match shape {
            Shape::Hori => vec![
                Point::new(2, self.peak + 4),  // self.peak can be -1
                Point::new(3, self.peak + 4),
                Point::new(4, self.peak + 4),
                Point::new(5, self.peak + 4),
            ],
            Shape::Cross => vec![
                Point::new(2, self.peak + 5),
                Point::new(3, self.peak + 4),
                Point::new(3, self.peak + 5),
                Point::new(3, self.peak + 6),
                Point::new(4, self.peak + 5),
            ],
            Shape::Corner => vec![
                Point::new(2, self.peak + 4),
                Point::new(3, self.peak + 4),
                Point::new(4, self.peak + 4),
                Point::new(4, self.peak + 5),
                Point::new(4, self.peak + 6),
            ],
            Shape::Verti => vec![
                Point::new(2, self.peak + 4),
                Point::new(2, self.peak + 5),
                Point::new(2, self.peak + 6),
                Point::new(2, self.peak + 7),
            ],
            Shape::Square => vec![
                Point::new(2, self.peak + 4),
                Point::new(3, self.peak + 4),
                Point::new(2, self.peak + 5),
                Point::new(3, self.peak + 5),
            ],
        };
        let mut output = HashSet::new();
        rocks.iter().for_each(|rock| {
            output.insert(rock.clone());
        });
        self.moving_rocks = output;
    }
}

fn simulate(rounds: usize, stream: &[char]) {
    let mut stream_cur: usize = 0;
    let new_shapes = vec![Shape::Hori, Shape::Cross, Shape::Corner, Shape::Verti, Shape::Square];
    let mut sim = Simulation::new(7);
    for round in 0..rounds {
        let shape = new_shapes.get(round % new_shapes.len()).unwrap();
        sim.spawn(shape);

        while sim.moving_rocks.len() > 0 { // move horizontally first, then vertically
            let hori_dir = match stream.get(stream_cur % stream.len()).unwrap() {
                '<' => Dir::Left,
                '>' => Dir::Right,
                _ => unreachable!("Illegal input"),
            };
            stream_cur += 1;
            let next = sim.get_next_move(hori_dir);
            if sim.is_in_bounds(&next) && !sim.is_colliding(&next) {
                sim.commit_move(next);
            }

            let next = sim.get_next_move(Dir::Down);
            if sim.is_in_bounds(&next) && !sim.is_colliding(&next) {
                sim.commit_move(next);
            } else {
                sim.stabilize_moving_rocks();
            }
        }
    }

    println!("{}", sim.peak + 1);
}

fn main() {
    let inputs = fs::read_to_string("inputs/17.test").unwrap();
    let stream: Vec<char> = inputs.lines().next().unwrap().chars().collect();
    simulate(2022, &stream);

    // TODO: Solve part 2 by finding the period of simulation
}
