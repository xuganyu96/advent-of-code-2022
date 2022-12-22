use std::fs;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

#[derive(Debug,Clone,Eq,PartialEq,Hash)]
struct Point {
    x: i32, y: i32
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        return Self { x, y };
    }
}

#[derive(Debug,Eq,PartialEq,Hash,Clone)]
enum Tile {
    Open(Point), Wall(Point)
}

impl Tile {
    fn new_open(x: i32, y: i32) -> Self {
        return Self::Open(Point::new(x, y));
    }

    fn new_wall(x: i32, y: i32) -> Self {
        return Self::Wall(Point::new(x, y));
    }

    fn get_pos(&self) -> Point {
        return match self {
            Tile::Open(p) => p.clone(),
            Tile::Wall(p) => p.clone(),
        }
    }
}

#[derive(Debug)]
struct Board {
    grid: Vec<Vec<Option<Tile>>>,
    tiles: HashMap<Point, Tile>,
}

impl Board {
    /// If cursor moves for one unit at the cursor's direction, it will run
    /// into void; find the tile that is the wrap-around
    fn wrap(&self, cursor: &Cursor) -> Cursor {
        let tile = match cursor.dir {
            Dir::Up => {
                // iterate through all tiles with the same x, return the one
                // with the maximum y
                let y = self.tiles.iter()
                    .filter(|(p, _)| p.x == cursor.pos.x)
                    .map(|(p, _)| p.y)
                    .max().unwrap();
                let p = Point::new(cursor.pos.x, y);
                self.tiles.get(&p).unwrap().clone()
            },
            Dir::Down => {
                // iterate through all tiles with the same x, return the one
                // with the minimum y
                let y = self.tiles.iter()
                    .filter(|(p, _)| p.x == cursor.pos.x)
                    .map(|(p, _)| p.y)
                    .min().unwrap();
                let p = Point::new(cursor.pos.x, y);
                self.tiles.get(&p).unwrap().clone()
            },
            Dir::Left => {
                // iterate through all tiles with the same y, return the one
                // with the maximum x
                let x = self.tiles.iter()
                    .filter(|(p, _)| p.y == cursor.pos.y)
                    .map(|(p, _)| p.x)
                    .max().unwrap();
                let p = Point::new(x, cursor.pos.y);
                self.tiles.get(&p).unwrap().clone()
            },
            Dir::Right => {
                // itearte through all tiles with the same y, return the one
                // with the minimum x
                let x = self.tiles.iter()
                    .filter(|(p, _)| p.y == cursor.pos.y)
                    .map(|(p, _)| p.x)
                    .min().unwrap();
                let p = Point::new(x, cursor.pos.y);
                self.tiles.get(&p).unwrap().clone()
            },
        };

        if let Tile::Open(next_tile_pos) = tile {
            return Cursor::new(&next_tile_pos, &cursor.dir);
        } else {
            return Cursor::new(&cursor.pos, &cursor.dir);
        }
    }

    /// Insanely ugly wrapping algorithm
    fn cube_wrap(&self, cursor: &Cursor) -> Cursor {
        let (next_pos, next_dir) = match (&cursor.pos.x, &cursor.pos.y, &cursor.dir) {
            (100..=149, 49, Dir::Down) => {  // 1A
                let pos = Point::new(99, 50 + (cursor.pos.x - 100));
                (pos, Dir::Left)
            },
            (99, 50..=99, Dir::Right) => { // 1B
                let pos = Point::new(100 + (cursor.pos.y - 50), 49);
                (pos, Dir::Up)
            },
            (149, 0..=49, Dir::Right) => { // 2A
                let pos = Point::new(99, 100 + (49 - cursor.pos.y));
                (pos, Dir::Left)
            },
            (99, 100..=149, Dir::Right) => { // 2B
                let pos = Point::new(149, 49 - (cursor.pos.y - 100));
                (pos, Dir::Left)
            },
            (50..=99, 149, Dir::Down) => { // 3A
                let pos = Point::new(49, 150 + (cursor.pos.x - 50));
                (pos, Dir::Left)
            },
            (49, 150..=199, Dir::Right) => {  // 3B
                let pos = Point::new(50 + (cursor.pos.y - 150), 149);
                (pos, Dir::Up)
            },
            (50, 50..=99, Dir::Left) => { // 4A
                let pos = Point::new(49 - (99 - cursor.pos.y), 100);
                (pos, Dir::Down)
            },
            (0..=49, 100, Dir::Up) => {  // 4B
                let pos = Point::new(50, 99 - (49 - cursor.pos.x));
                (pos, Dir::Right)
            },
            (0, 100..=149, Dir::Left) => {  // 5A
                let pos = Point::new(50, 49 - (cursor.pos.y - 100));
                (pos, Dir::Right)
            },
            (50, 0..=49, Dir::Left) => { // 5B
                let pos = Point::new(0, 100 + (49 - cursor.pos.y));
                (pos, Dir::Right)
            },
            (50..=99, 0, Dir::Up) => { // 6A
                let pos = Point::new(0, 150 + (cursor.pos.x - 50));
                (pos, Dir::Right)
            }
            (0, 150..=199, Dir::Left) => { // 6B
                let pos = Point::new(50 + (cursor.pos.y - 150), 0);
                (pos, Dir::Down)
            }
            (100..=149, 0, Dir::Up) => { // 7A
                let pos = Point::new(cursor.pos.x - 100, 199);
                (pos, Dir::Up)
            },
            (0..=49, 199, Dir::Down) => { // 7B
                let pos = Point::new(100 + cursor.pos.x, 0);
                (pos, Dir::Down)
            },
            _ => unreachable!("Illegal wrapping!"),
        };

        let tile = self.tiles.get(&next_pos).unwrap();  // guaranteed to exist
        if let Tile::Open(_) = tile {
            return Cursor::new(&next_pos, &next_dir);
        } else {
            return Cursor::new(&cursor.pos, &cursor.dir);
        }
    }

    fn new() -> Self {
        return Self { grid: vec![], tiles: HashMap::new() };
    }

    fn from_inputs(inputs: &str) -> Self {
        let mut board = Self::new();

        inputs.lines()
            .enumerate()
            .for_each(|(i, line)| {
                let mut row = vec![];
                line.chars().enumerate()
                    .for_each(|(j, c)| {
                        let tile = match c {
                            '.' => Some(Tile::new_open(j as i32, i as i32)),
                            '#' => Some(Tile::new_wall(j as i32, i as i32)),
                            _ => None,
                        };
                        row.push(tile.clone());
                        if let Some(tile) = tile {
                            let point = tile.get_pos();
                            board.tiles.insert(point, tile);
                        }
                    });
                board.grid.push(row);
            });
        
        return board;
    }

    /// Top row, left most open position
    fn get_start(&self) -> Cursor {
        for tile in self.grid[0].iter() {
            if let Some(tile) = tile {
                if let Tile::Open(pos) = tile {
                    return Cursor::new(pos, &Dir::Right);
                }
            }
        }

        unreachable!("First row does not contain open tile!");
    }
}

#[derive(Debug)]
enum Instr {
    Move(i32),
    Right,  // clockwise
    Left,  // counterclockwise
}

#[derive(Debug,Clone)]
enum Dir { Up, Down, Left, Right }

impl Dir {
    fn score(&self) -> i32 {
        return match self {
            Self::Right => 0,
            Self::Down => 1,
            Self::Left => 2,
            Self::Up => 3,
        };
    }
}

#[derive(Debug)]
struct Cursor {
    pos: Point,
    dir: Dir,
}

impl Cursor {
    fn new(p: &Point, dir: &Dir) -> Self {
        return Self { pos: p.clone(), dir: dir.clone() };
    }

    /// Return a new cursor that is the state of the current cursor having
    /// moved one unit in the direction it currently faces
    fn increment(&self, board: &Board) -> Self {
        let (x_delta, y_delta) = match self.dir {
            Dir::Up => (0, -1),
            Dir::Down => (0, 1),
            Dir::Left => (-1, 0),
            Dir::Right => (1, 0),
        };
        let new_pos = Point::new(self.pos.x + x_delta, self.pos.y + y_delta);

        // Check if new pos is "Void", "Wall" or "Open"
        let next_tile = board.tiles.get(&new_pos);
        if let None = next_tile { // will run into void, get the wrap
            return board.wrap(&self);
        } else if let Some(Tile::Wall(_)) = next_tile {  // don't move
            return Self::new(&self.pos, &self.dir);
        } else if let Some(Tile::Open(_)) = next_tile {
            return Self::new(&new_pos, &self.dir);
        }

        todo!();
    }

    fn cube_increment(&self, board: &Board) -> Self {
        let (x_delta, y_delta) = match self.dir {
            Dir::Up => (0, -1),
            Dir::Down => (0, 1),
            Dir::Left => (-1, 0),
            Dir::Right => (1, 0),
        };
        let new_pos = Point::new(self.pos.x + x_delta, self.pos.y + y_delta);

        // Check if new pos is "Void", "Wall" or "Open"
        let next_tile = board.tiles.get(&new_pos);
        if let None = next_tile { // will run into void, get the wrap
            return board.cube_wrap(&self);
        } else if let Some(Tile::Wall(_)) = next_tile {  // don't move
            return Self::new(&self.pos, &self.dir);
        } else if let Some(Tile::Open(_)) = next_tile {
            return Self::new(&new_pos, &self.dir);
        }

        todo!();
    }


    /// rotate the cursor
    fn rotate(&self, clockwise: bool) -> Self {
        let new_dir = if clockwise {
            match self.dir {
                Dir::Right => Dir::Down,
                Dir::Down => Dir::Left,
                Dir::Left => Dir::Up,
                Dir::Up => Dir::Right,
            }
        } else {
            match self.dir {
                Dir::Right => Dir::Up,
                Dir::Up => Dir::Left,
                Dir::Left => Dir::Down,
                Dir::Down => Dir::Right,
            }
        };

        return Self::new(&self.pos, &new_dir);
    }
}

fn parse_instructions(instr_str: &str) -> Vec<Instr> {
    let mut instrs = vec![];
    let mut num_str = String::new();
    for c in instr_str.chars() {
        match c {
            'R' => {
                let num = num_str.parse::<i32>().unwrap();
                instrs.push(Instr::Move(num));
                instrs.push(Instr::Right);
                num_str = String::new();
            },
            'L' => {
                let num = num_str.parse::<i32>().unwrap();
                instrs.push(Instr::Move(num));
                instrs.push(Instr::Left);
                num_str = String::new();
            },
            '\n' => (),
            _ => {
                num_str.push(c);
            }
        }
    }
    let num = num_str.parse::<i32>().unwrap();
    instrs.push(Instr::Move(num));
    return instrs;
}

fn render(board: &Board, cursor: &Cursor, range: i32) {
    let x_start = cursor.pos.x - range;
    let x_stop = cursor.pos.x + range;
    let y_start = cursor.pos.y - range;
    let y_stop = cursor.pos.y + range;
    let mut grid_str = String::new();
    for r in y_start..=y_stop {
        for c in x_start..=x_stop {
            let point = Point::new(c, r);
            if point == cursor.pos {
                match cursor.dir {
                    Dir::Up => grid_str.push_str("^"),
                    Dir::Right => grid_str.push_str(">"),
                    Dir::Down => grid_str.push_str("v"),
                    Dir::Left => grid_str.push_str("<"),
                }
            } else {
                let tile = board.tiles.get(&point);
                if let None = tile {
                    grid_str.push_str(" ");
                } else if let Some(Tile::Open(_)) = tile {
                    grid_str.push_str(".");
                } else if let Some(Tile::Wall(_)) = tile {
                    grid_str.push_str("#");
                }
            }
        }
        grid_str.push_str("\n");
    }
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    print!("{grid_str}");
    thread::sleep(Duration::from_millis(160));
}

fn traverse(board: &Board, instrs: &[Instr], cubed: bool) -> i32 {
    let mut cursor = board.get_start();
    instrs.iter()
        .for_each(|instr| {
            match instr {
                Instr::Move(num) => {
                    for _ in 0..*num {
                        if cubed {
                            cursor = cursor.cube_increment(&board);
                        } else {
                            cursor = cursor.increment(&board);
                        }
                    }
                },
                Instr::Right => {
                    cursor = cursor.rotate(true);
                },
                Instr::Left => {
                    cursor = cursor.rotate(false);
                }
            }
        });

    let row = cursor.pos.y + 1;
    let col = cursor.pos.x + 1;
    let dir_score = cursor.dir.score();
    let score = 1000 * row + 4 * col + dir_score;
    return score;
}

fn main() {
    let inputs = fs::read_to_string("inputs/22.txt").unwrap();
    let (grid_str, instr_str) = inputs.split_once("\n\n").unwrap();
    let board = Board::from_inputs(&grid_str);
    let instrs = parse_instructions(&instr_str);

    println!("{}", traverse(&board, &instrs, false)); // part 1
    println!("{}", traverse(&board, &instrs, true)); // part 2
}

