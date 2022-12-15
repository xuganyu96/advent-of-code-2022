use std::fs;
use std::collections::HashSet;

#[derive(Debug,Clone,Eq,PartialEq)]
struct Point { x: i128, y: i128 }

impl Point {
    fn parse_coordinate_str(c: &str) -> Self { // "x=2, y=18"
        let mut tokens = c.split(", ");
        let x = &tokens.next().unwrap()[2..].parse::<i128>().unwrap();
        let y = &tokens.next().unwrap()[2..].parse::<i128>().unwrap();

        return Point{ x: *x, y: *y };
    }

    /// Return squared distance
    fn dist(&self, other: &Point) -> i128 {
        let xdelta = self.x - other.x;
        let ydelta = self.y - other.y;
        return xdelta.abs() + ydelta.abs();
    }

    /// Return the top, bottom, left, and right limits of the circle centered
    /// at self and with a point at p
    fn limits(&self, p: &Point) -> (i128, i128, i128, i128) {
        let (mut left, mut right, mut top, mut bottom) = (self.x, self.x, self.y, self.y);
        
        while self.dist(&Point{ x: left, y: self.y }) <= self.dist(p) {
            left -= 1;
        }

        while self.dist(&Point{ x: right, y: self.y }) <= self.dist(p) {
            right += 1;
        }
        while self.dist(&Point{ x: self.x, y: top }) <= self.dist(p) {
            top -= 1;
        }
        while self.dist(&Point{ x: self.x, y: bottom }) <= self.dist(p) {
            bottom += 1;
        }
        return (left, right, top, bottom);
    }
}

/// Inclusive on both ends
#[derive(PartialEq,Eq,Clone,Debug,Hash)]
struct Range {
    start: i128,
    stop: i128,
}

impl Range {
    fn len(&self) -> i128 {
        return self.stop - self.start + 1;
    }

    fn new(start: i128, stop: i128) -> Self {
        return Self { start, stop };
    }

    fn intersect(&self, other: &Range) -> Option<Self> {
        if self.stop < other.start || self.start > other.stop {
            return None;
        }
        let start = self.start.max(other.start);
        let stop = self.stop.min(other.stop);
        return Some(Self::new(start, stop));
    }

    fn can_union(&self, other: &Range) -> bool {
        return !(self.stop < other.start - 1) && !(self.start > other.stop + 1);
    }
    
    /// Assume that the two actually overlaps
    fn union(&self, other: &Range) -> Self {
        let start = self.start.min(other.start);
        let stop = self.stop.max(other.stop);
        return Self::new(start, stop);
    }
}

/// A collection of ranges
#[derive(Debug)]
struct Coverage {
    ranges: HashSet<Range>,  // disjoint!
}

impl Coverage {
    fn get_gap(&self) -> i128 {
        let mut ranges = vec![];
        self.ranges.iter().for_each(|range| ranges.push(range.clone()));
        ranges.sort_by_key(|range| range.start);
        
        let mut gap = 0;
        ranges.windows(2)
            .for_each(|two_ranges| {
                let first_r = &two_ranges[0];
                let second_r = &two_ranges[1];
                if first_r.stop + 2 == second_r.start {
                    gap = first_r.stop + 1;
                }
            });
        
        return gap;
    }

    fn new() -> Self {
        return Self { ranges: HashSet::new() };
    }

    /// Number of numbers that fall within any of the ranges
    fn len(&self) -> i128 {
        return self.ranges.iter().map(|range| range.stop - range.start + 1).sum::<i128>();
    }

    fn add(&mut self, new: Range) {
        let mut intersected: Option<Range> = None;

        for range in &self.ranges {
            if range.can_union(&new) {
                intersected = Some(range.clone());
            }
        }

        if let Some(range) = intersected {
            self.ranges.remove(&range);
            let new = new.union(&range);
            self.add(new);
        } else {
            self.ranges.insert(new);
        }
    }

    /// Intersect each element with other. If None, remove the element. If Some, insert back in
    fn intersect(&mut self, other: &Range) {
        let mut ranges = vec![];
        self.ranges.drain().for_each(|range| ranges.push(range));
        ranges.iter()
            .for_each(|range| {
                let intersection = range.intersect(other);
                if let Some(intersection) = intersection {
                    self.ranges.insert(intersection);
                }
            });
    }
}

#[derive(Debug)]
struct Network {
    readings: Vec<(Point, Point)>,  // sensors and their closest beacons
    left: i128,
    right: i128,
    top: i128,
    bottom: i128,
}

impl Network {
    fn new() -> Self {
        let readings = vec![];
        let (left, right, top, bottom) = (0, 0, 0, 0);
        return Self { readings, left, right, top, bottom };
    }

    /// return the left most and right most x's such that (x, y)
    /// cannot be a valid beacon, unless the line does not intersect with the
    /// diamond, then return None
    fn coverage(sensor: &Point, beacon: &Point, y: i128) -> Option<Range> {
        let dist = sensor.dist(beacon);
        let ydelta = (y - sensor.y).abs();

        // ydelta = 0: sensor.x - dist ---- sensor.x + dist
        // ydelta = 1: sensor.x - (dist-1) ---- sensor.x + (dist-1)
        // ...
        // ydelta = dist; sensor.x -- sensor.x
        let xdelta = dist - ydelta;
        if xdelta < 0 { return None ;}
        return Some(Range::new(sensor.x - xdelta, sensor.x + xdelta));
    }

    fn add_sensor(&mut self, sensor: &Point, beacon: &Point) {
        self.readings.push((sensor.clone(), beacon.clone()));
        let (left, right, top, bottom) = sensor.limits(beacon);
        if left < self.left { self.left = left };
        if right > self.right { self.right = right };
        if top < self.top { self.top = top };
        if bottom > self.bottom { self.bottom = bottom };
        // println!("{sensor:?}, {beacon:?}");
        // println!("{left} {right} {top} {bottom}");
    }

    /// Not at or closer to any sensor than that sensor's closest beacon
    fn is_valid_beacon(&self, p: &Point, count_beacon: bool) -> bool {
        let mut is_beacon = false;
        let is_outside = self.readings.iter()
            .map(|(sen, bea)| {
                if p == bea { is_beacon = true }
                let closest_dist = sen.dist(bea);
                let cur_dist = sen.dist(p);
                if cur_dist <= closest_dist {
                    // println!("p {p:?}: {cur_dist} is too close to sensor {sen:?} --{closest_dist}-- {bea:?}");
                }
                return cur_dist > closest_dist;
            })
            .all(|x| x);
        // println!("{p:?} {is_beacon} {is_outside}");
        return is_beacon && count_beacon || is_outside;
    }
}

fn main() {
    let inputs = fs::read_to_string("inputs/15.txt").unwrap();
    let y = 2000000;
    let limit = 4000000;
    let multiplier = 4000000;
    let mut network = Network::new();
    
    inputs.lines()
        .for_each(|line| {
            let mut clauses = line.split(": closest beacon is at ");
            let sensor_clause = &clauses.next().unwrap()[10..];
            let beacon_clause = clauses.next().unwrap();
            let sensor = Point::parse_coordinate_str(sensor_clause);
            let beacon = Point::parse_coordinate_str(beacon_clause);
            network.add_sensor(&sensor, &beacon);
        });
  
    let mut count = 0;
    for x in network.left..=network.right {
        let point = Point{ x, y };
        if !network.is_valid_beacon(&point, true) {
            count += 1;
        }
    }
    println!("{}", count);


   
    // part 2:
    // Iterating over all (limit * limit) possibilities is not an option
    let mut tuning_freq = 0;
    for y in 0..=limit {
        let entire_row = Range::new(0, limit);
        let mut coverage = Coverage::new();
        for (sensor, beacon) in network.readings.iter() {
            let range = Network::coverage(sensor, beacon, y);
            if let Some(range) = range {
                coverage.add(range);
            }
        }
        coverage.intersect(&entire_row);
        if coverage.len() != (limit + 1) {  // have found the coverage we are looking for
            let x = coverage.get_gap();
            tuning_freq = x * multiplier + y;
        }
    }
    println!("{tuning_freq}");
}

