use std::fs;
use std::collections::{ HashSet, VecDeque };
use std::time;

#[derive(Debug,Eq,PartialEq,Hash,Clone)]
struct Point {
    x: i32, y: i32
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        return Self { x, y };
    }
}

#[derive(Debug,Eq,PartialEq,Hash,Clone)]
enum Dir {
    Up, Down, Left, Right
}

#[derive(Debug,Eq,PartialEq,Hash,Clone)]
struct Storm {
    pos: Point, dir: Dir
}

impl Storm {
    fn new(pos: &Point, dir: &Dir) -> Self {
        return Self { pos: pos.clone(), dir: dir.clone() };
    }
}

#[derive(Debug,Eq,PartialEq,Clone)]
struct Map {
    storms: HashSet<Storm>,
    walls: HashSet<Point>,
    cur: Point,
    stop: Point,
    shape: (usize, usize),  // nrows, ncols
}

impl Map {
    fn parse_inputs(inputs: &str) -> Self {
        let mut storms = HashSet::new();
        let mut walls = HashSet::new();
        let cur = Point::new(1, 0);
        let nrows = inputs.lines().count();
        let ncols = inputs.lines().next().unwrap().chars().count();
        let shape = (nrows, ncols);
        let stop = Point::new(ncols as i32 - 2 , nrows as i32 - 1);
        
        for (r, row) in inputs.lines().enumerate() {
            for (c, char_) in row.chars().enumerate() {
                match char_ {
                    '#' => {
                        walls.insert(Point::new(c as i32, r as i32));
                    },
                    '>' => {
                        let pos = Point::new(c as i32, r as i32);
                        let dir = Dir::Right;
                        storms.insert(Storm::new(&pos, &dir));
                    },
                    '^' => {
                        let pos = Point::new(c as i32, r as i32);
                        let dir = Dir::Up;
                        storms.insert(Storm::new(&pos, &dir));
                    },
                    '<' => {
                        let pos = Point::new(c as i32, r as i32);
                        let dir = Dir::Left;
                        storms.insert(Storm::new(&pos, &dir));
                    },
                    'v' => {
                        let pos = Point::new(c as i32, r as i32);
                        let dir = Dir::Down;
                        storms.insert(Storm::new(&pos, &dir));
                    },
                    _ => {},
                }
            }
        }

        return Self { storms, walls, cur, stop, shape };
    }

    fn is_in_bounds(&self, pos: &Point) -> bool {
        let (nrows, ncols) = self.shape;
        return pos.x >= 0 && pos.x < ncols as i32 
            && pos.y >= 0 && pos.y < nrows as i32;
    }

    fn render(&self) {
        let (y_max, x_max) = self.shape;
        let mut grid_str = String::new();
        
        for r in 0..y_max {
            for c in 0..x_max {
                let pos = Point::new(c as i32, r as i32);
                let storm_count = self.storms.iter()
                    .filter(|storm| storm.pos == pos)
                    .count();
                if self.walls.contains(&pos) {
                    grid_str.push_str("#");
                } else if storm_count > 1 {
                    let num = format!("{storm_count}");
                    grid_str.push_str(&num);
                } else if let Some(storm) = self.storms.iter()
                    .filter(|storm| storm.pos == pos)
                    .next() {
                    match storm.dir {
                        Dir::Up => grid_str.push_str("^"),
                        Dir::Down => grid_str.push_str("v"),
                        Dir::Left => grid_str.push_str("<"),
                        Dir::Right => grid_str.push_str(">"),
                    }
                } else if pos == self.cur {
                    grid_str.push_str("E");
                } else {
                    grid_str.push_str(".");
                }
            }
            grid_str.push_str("\n");
        }

        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        print!("{grid_str}");
    }

    /// Return a HashSet of "storms" that contains the new coordinates
    fn step_storm(&self) -> HashSet<Storm> {
        let mut storms = HashSet::new();
        let (nrows, ncols) = self.shape;
        
        self.storms.iter()
            .for_each(|storm| {
                let (x_delta, y_delta) = match storm.dir {
                    Dir::Up => (0, -1),
                    Dir::Down => (0, 1),
                    Dir::Right => (1, 0),
                    Dir::Left => (-1, 0),
                };
                let new_pos = Point::new(storm.pos.x + x_delta, storm.pos.y + y_delta);
                if self.walls.contains(&new_pos) {
                    let new_pos = match storm.dir {
                        Dir::Up => Point::new(storm.pos.x, nrows as i32 - 2),
                        Dir::Down => Point::new(storm.pos.x, 1),
                        Dir::Right => Point::new(1, storm.pos.y),
                        Dir::Left => Point::new(ncols as i32 - 2, storm.pos.y),
                    };
                    storms.insert(Storm::new(&new_pos, &storm.dir));
                } else {
                    storms.insert(Storm::new(&new_pos, &storm.dir));
                }
            });

        return storms;
    }

    /// Get valid "next_pos" which is defined by "not wall and not storm";
    /// I will put "down" and "right" first because they are preferable
    fn get_next_curs(&self) -> Vec<Point> {
        let mut next_curs = vec![];
        let next_storms = self.step_storm(); // 新的风暴已经出现

        for (x_delta, y_delta) in [
            (0, -1), (0, 1),  // going right and down are preferrable
            (0, 0),  // 怎么能够停滞不前
                     // staying put is better than getting further
            (1, 0), (-1, 0),   // going lef tnad up are not
        ] {
            let new_cur = Point::new(self.cur.x + x_delta, self.cur.y + y_delta);
            if next_storms.iter()
                .filter(|storm| storm.pos == new_cur)
                .count() == 0 
                && !self.walls.contains(&new_cur)
                && self.is_in_bounds(&new_cur) {
                next_curs.push(new_cur);
            }
        }

        return next_curs;
    }
}

/// BFS, which means when you reach the destination, it's the best time
fn bfs(cur_state: &Map, max_time: usize) -> (usize, Map) {
    let mut backlog: VecDeque<(usize, Map)> = VecDeque::new();
    let mut footprints: HashSet<(usize, Point)> = HashSet::new();
    backlog.push_back((0, cur_state.clone()));
    footprints.insert((0, cur_state.cur.clone()));

    while backlog.len() > 0 {
        let (top_time, top_map) = backlog.pop_front().unwrap();
        // println!("{top_time} {:?}", top_map.cur);
        if top_map.cur == top_map.stop { return (top_time, top_map); }
        if top_time > max_time {  panic!("ran out of time") }  // all items behind it will
                                                 // take longer
        for next_cur in top_map.get_next_curs() {
            if !footprints.contains(&(top_time + 1, next_cur.clone())) {
                let mut next_map = top_map.clone();
                next_map.cur = next_cur.clone();
                next_map.storms = top_map.step_storm();
                footprints.insert((top_time + 1, next_cur));
                backlog.push_back((top_time + 1, next_map));
            }
        }
    }

    panic!("ran out of time")
}

fn main() {
    let inputs = fs::read_to_string("inputs/24.txt").unwrap();
    // part 1
    let map = Map::parse_inputs(&inputs);
    let start = time::Instant::now();
    let (time, _) = bfs(&map, 1000);
    println!("{time:?}, {:.2?} elapsed", start.elapsed());

    let start = time::Instant::now();
    let mut map = Map::parse_inputs(&inputs);
    let (time_1, new_map) = bfs(&map, 1000);
    println!("Trip 1 took {time_1}, {:.2?} elapsed", start.elapsed());
    map = new_map;
    map.stop = Point::new(1, 0);
    let (time_2, new_map) = bfs(&map, 1000);
    println!("Trip 2 took {time_2}, {:.2?} elapsed", start.elapsed());
    map = new_map;
    let (nrows, ncols) = map.shape;
    map.stop = Point::new(ncols as i32 - 2, nrows as i32 - 1);
    let (time_3, _) = bfs(&map, 1000);
    println!("Trip 3 took {time_3}, {:.2?} elapsed", start.elapsed());
    println!("{}", time_1 + time_2 + time_3);
}
