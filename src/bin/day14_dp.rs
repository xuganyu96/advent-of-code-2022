use std::fs;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
struct Point { x: i32, y: i32 }

impl Point {
    fn new(x: i32, y: i32) -> Self {
        return Self { x, y };
    }
}

struct Simulation {
    rocks: HashSet<Point>,
    sands: HashSet<Point>,
    bottom_line: i32,  // depth of the deepest rock
    unstable_sand: Option<Point>,  // the next sand to move
    floor: Option<i32>,
}

impl Simulation {
    fn new() -> Self {
        return Self {
            rocks: HashSet::new(),
            sands: HashSet::new(),
            bottom_line: 0,
            unstable_sand: None,
            floor: None,
        };
    }

    fn from_input(inputs: &str) -> Self {
        let mut sim = Self::new();
        inputs.lines()
        .for_each(|line| {
            let mut points: Vec<Point> = vec![];
            line.split(" -> ")
                .for_each(|point| {
                    let mut nums = point.split(",");
                    let x = nums.next().unwrap().parse::<i32>().unwrap();
                    let y = nums.next().unwrap().parse::<i32>().unwrap();
                    points.push(Point{ x, y });
                });
            sim.add_line(&points);
        });
        return sim;
    }

    fn set_floor(&mut self) {
        self.floor = Some(self.bottom_line + 2);
    }

    fn add_rock(&mut self, rock: &Point) {
        let rock = rock.clone();
        if !self.rocks.contains(&rock) { 
            if rock.y > self.bottom_line { self.bottom_line = rock.y; }
            self.rocks.insert(rock); 
        }
    }

    fn render_rocks(start: &Point, stop: &Point) -> Vec<Point> {
        let mut rocks = vec![];
        if start.x == stop.x {
            if start.y <= stop.y {
                for y in start.y..=stop.y {
                    rocks.push(Point{ x: start.x, y });
                }
            } else {
                for y in stop.y..=start.y {
                    rocks.push(Point{ x: start.x, y });
                }
            }
        } else if start.y == stop.y {
            if start.x <= stop.x {
                for x in start.x..=stop.x {
                    rocks.push(Point{ x, y: start.y });
                }
            } else {
                for x in stop.x..=start.x {
                    rocks.push(Point{ x, y: start.y });
                }
            }
        } else {
            unreachable!("Only render rocks orthogonally");
        }

        return rocks;
    }

    fn add_line(&mut self, points: &[Point]) {
        points.windows(2)
            .for_each(|window| {
                let p1 = &window[0];
                let p2 = &window[1];
                let rocks: Vec<Point> = Self::render_rocks(p1, p2);
                rocks.iter().for_each(|rock| self.add_rock(rock));
            });
    }

    fn is_air(&self, p: &Point) -> bool {
        if let Some(floor) = self.floor {
            if p.y >= floor {
                return false;
            }
        }
        return !(self.rocks.contains(p) || self.sands.contains(p));
    }

    /// part 2
    fn is_safe_to_stand(&self, p: &Point) -> bool {
        return !self.is_air(p) && self.is_stable();
    }

    /// the next coordinate that this sand will fall onto, unless the sand
    /// cannot move, then return None
    fn next_move(&self, sand: &Point) -> Option<Point> {
        let bottom = Point{ x: sand.x, y: sand.y + 1 };
        let diag_left = Point{ x: sand.x - 1, y: sand.y + 1};
        let diag_right = Point{ x: sand.x + 1, y: sand.y + 1 };
        
        if self.is_air(&bottom) { return Some(bottom); }
        if self.is_air(&diag_left) { return Some(diag_left); }
        if self.is_air(&diag_right) { return Some(diag_right); }

        return None;
    }

    /// Simulation is stable if all sands are stable
    fn is_stable(&self) -> bool {
        if let None = self.unstable_sand { return true; }
        return false;
    }

    /// Add a sand at the set position; check if this sand is stable, if not, set
    /// unstable_sand
    fn add_sand(&mut self, sand: &Point) {
        self.sands.insert(sand.clone());
        if let Some(_) = self.next_move(sand) {
            self.unstable_sand = Some(sand.clone());
        } else {
            self.unstable_sand = None;
        }
    }

    /// Look through self.sands and move them. return whether sand moved or not
    fn step(&mut self) {
        if let Some(unstable_sand) = &self.unstable_sand {
            let next_sand = self.next_move(unstable_sand).unwrap();
            self.sands.remove(unstable_sand);
            self.sands.insert(next_sand.clone());
            if let Some(_) = self.next_move(&next_sand) {
                self.unstable_sand = Some(next_sand);
            } else {
                self.unstable_sand = None;
            }
        }
    }

    /// Falls into abyss if any sand is deeper than the deepest of all rocks
    fn is_abyss(&self) -> bool {
        if let Some(unstable_sand) = &self.unstable_sand {
            return unstable_sand.y >= self.bottom_line;
        } else {
            return false;
        }
    }

    fn render(&self, width: i32, depth: i32, source: &Point) {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        let mut grid: Vec<String> = Vec::new();
        for d in 0..=depth {
            let mut line = String::new();
            
            for w in -width..=width {
                let p = Point{ x: source.x + w, y: d};

                if self.rocks.contains(&p) { line.push_str("#"); }
                else if self.sands.contains(&p) { line.push_str("o"); }
                else if p == *source {
                    line.push_str("X");
                }
                else if let Some(floor) = self.floor {
                    if p.y >= floor { line.push_str("#"); }
                    else { line.push_str("."); }
                }
                else { line.push_str("."); }
            }

            grid.push(line);
        }
        
        grid.iter().for_each(|line| println!("{line}"));
    }
}

/// One part for parsing inputs into a simulation struct
///
/// A second part for running the simulation
fn main() {
    let inputs = fs::read_to_string("inputs/14.txt").unwrap();
    let mut sim = Simulation::from_input(&inputs);
 
    while !sim.is_abyss() {
        sim.add_sand(&Point{ x: 500, y: 0 });
        while !sim.is_stable() && !sim.is_abyss() {
            sim.step();
        }
    }
    println!("{}", sim.sands.len() - 1);

    // part 2
    let mut sim = Simulation::from_input(&inputs);
    sim.set_floor();

    let source = Point{ x: 500, y: 0};
    let rocks = &sim.rocks;
    let floor = &sim.floor.unwrap();
    let mut sand_count = 0;
    let mut cur_depth = 0;
    let mut cur_level = HashSet::new();
    cur_level.insert(source);

    while cur_depth < *floor {
        sand_count += cur_level.len();
        let mut next_level = HashSet::new();
        cur_level.iter().for_each(|sand| {
            for next_ in [
                Point::new(sand.x, sand.y+1),
                Point::new(sand.x-1, sand.y+1),
                Point::new(sand.x+1, sand.y+1),
            ] {
                if !rocks.contains(&next_) {
                    next_level.insert(next_);
                }
            }
        });
        cur_level = next_level;
        cur_depth += 1;
    }

    println!("{sand_count}");
}
