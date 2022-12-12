use std::fs;
use std::collections::{ BinaryHeap, HashMap };

#[derive(Ord, Eq, PartialEq, PartialOrd, Hash, Clone)]
struct Point(i32, i32);

fn parse_inputs(inputs: &str) -> (HashMap<Point, i32>, Point, Point) {
    // Maps (row, col) to their height
    let mut grid: HashMap<Point, i32> = HashMap::new();
    let mut start = Point(0, 0);
    let mut end = Point(0, 0);

    for (r, line) in inputs.lines().enumerate() {
        for (c, height) in line.as_bytes().iter().enumerate() {
            let point = Point(r as i32, c as i32);
            match height {
                97..=122 => {
                    grid.insert(point, (height - 97) as i32);
                },
                83 => {
                    grid.insert(point.clone(), 0);
                    start = point.clone();
                }
                69 => {
                    grid.insert(point.clone(), 25);
                    end = point.clone();
                },
                _ => unreachable!("Illegal input"),
            };
        }
    }

    return (grid, start, end);
}

/// Return a list of coordinates that the input coordinate can travel to 
fn get_neighbors(grid: &HashMap<Point, i32>, point: Point) -> Vec<Point> {
    let mut neighbors = vec![];
    let Point(r, c) = point;
    let height = grid.get(&point).unwrap();

    for neighbor in [
        Point(r-1, c),
        Point(r+1, c),
        Point(r, c-1),
        Point(r, c+1),
    ] {
        if let Some(neighbor_height) = grid.get(&neighbor) {
            if *neighbor_height <= *height + 1 {
                neighbors.push(neighbor);
            }
        }
    }

    return neighbors;
}

fn dijkstra(grid: &HashMap<Point, i32>, start: Point) -> HashMap<Point, i32> {
    let mut dists: HashMap<Point, i32> = HashMap::from([(start.clone(), 0)]);
    let mut pq: BinaryHeap<(i32, Point)> = BinaryHeap::from([(0, start.clone())]);  // this is max heap
    let mut visited: HashMap<Point, bool> = HashMap::from([(start, true)]);

    while let Some((dist, cur)) = pq.pop() {
        if !dists.contains_key(&cur) {
            dists.insert(cur.clone(), -dist);
        }

        for neighbor in get_neighbors(&grid, cur) {
            if !visited.contains_key(&neighbor) {
                pq.push((dist-1, neighbor.clone()));
                visited.insert(neighbor, true);
            }
        }
    }

    return dists;
}

fn main() {
    let inputs = fs::read_to_string("inputs/12.txt").unwrap();
    let (grid, start, stop) = parse_inputs(&inputs);

    let dists = dijkstra(&grid, start);
    let min = dists.get(&stop).unwrap();
    println!("{min}");
    let mut min = *min;

    for point in grid.keys() {
        let height = grid.get(point).unwrap();
        if *height == 0 {
            let dists = dijkstra(&grid, point.clone());
            let local_min = dists.get(&stop);
            match local_min {
                Some(m) if *m < min => { min = *m; },
                _ => (),
            }
        }
    }
    println!("{min}");
}
