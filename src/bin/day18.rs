//! Day 18: Boiling Boulders
use std::collections::{HashSet, VecDeque};
use std::fs;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

impl Cube {
    fn new(x: i32, y: i32, z: i32) -> Self {
        return Self { x, y, z };
    }

    /// each line is like "2,2,2"
    fn from_str(line: &str) -> Self {
        let mut tokens = line.split(",");
        let x: i32 = tokens.next().unwrap().parse().unwrap();
        let y: i32 = tokens.next().unwrap().parse().unwrap();
        let z: i32 = tokens.next().unwrap().parse().unwrap();

        return Self { x, y, z };
    }

    fn neighbors(&self) -> [Self; 6] {
        return [
            Self::new(self.x - 1, self.y, self.z),
            Self::new(self.x + 1, self.y, self.z),
            Self::new(self.x, self.y - 1, self.z),
            Self::new(self.x, self.y + 1, self.z),
            Self::new(self.x, self.y, self.z + 1),
            Self::new(self.x, self.y, self.z - 1),
        ];
    }
}

fn get_limits(cubes: &HashSet<Cube>) -> (i32, i32, i32, i32, i32, i32) {
    let (mut x_min, mut x_max, mut y_min, mut y_max, mut z_min, mut z_max) = (0, 0, 0, 0, 0, 0);

    cubes.iter().for_each(|cube| {
        x_min = x_min.min(cube.x);
        x_max = x_max.max(cube.x);
        y_min = y_min.min(cube.y);
        y_max = y_max.max(cube.y);
        z_min = z_min.min(cube.z);
        z_max = z_max.max(cube.z);
    });

    return (x_min, x_max, y_min, y_max, z_min, z_max);
}

fn is_inbound(cube: &Cube, bounds: (i32, i32, i32, i32, i32, i32)) -> bool {
    let (x_min, x_max, y_min, y_max, z_min, z_max) = bounds;
    return cube.x >= x_min
        && cube.x <= x_max
        && cube.y >= y_min
        && cube.y <= y_max
        && cube.z >= z_min
        && cube.z <= z_max;
}

/// is outside if it can gradually expand out of bounds
fn is_outside(
    start: &Cube,
    droplets: &HashSet<Cube>,
    memo: &mut HashSet<Cube>,
    boundary: (i32, i32, i32, i32, i32, i32),
) -> bool {
    if droplets.contains(start) {
        return false;
    }
    if memo.contains(start) {
        return true;
    } // memoization
    let mut footprints: HashSet<Cube> = HashSet::new();
    let mut backlog = VecDeque::new(); // push_back and pop_front
    backlog.push_back(start.clone());
    footprints.insert(start.clone());

    while let Some(next_cube) = backlog.pop_front() {
        if !is_inbound(&next_cube, boundary) {
            memo.insert(next_cube.clone());
            for connected_cube in footprints.iter() {
                memo.insert(connected_cube.clone());
            }
            return true;
        }
        for neighbor in next_cube.neighbors() {
            if !footprints.contains(&neighbor) && !droplets.contains(&neighbor) {
                backlog.push_back(neighbor.clone());
            }
            footprints.insert(neighbor.clone());
        }
    }

    return false;
}

fn external_surface_area(inputs: &str, external_only: bool) {
    let mut droplets = HashSet::new();
    inputs.lines().for_each(|line| {
        droplets.insert(Cube::from_str(line));
    });
    let mut memo = HashSet::new();

    let surface_area = droplets
        .iter()
        .map(|cube| {
            return cube
                .neighbors()
                .iter()
                .map(|neighbor| {
                    if !external_only && !droplets.contains(neighbor) {
                        return 1;
                    }
                    if external_only
                        && is_outside(neighbor, &droplets, &mut memo, get_limits(&droplets))
                    {
                        return 1;
                    }
                    return 0;
                })
                .sum::<i32>();
        })
        .sum::<i32>();
    println!("{}", surface_area);
}

fn main() {
    let inputs = fs::read_to_string("inputs/18.txt").unwrap();
    external_surface_area(&inputs, false);
    external_surface_area(&inputs, true);
}
