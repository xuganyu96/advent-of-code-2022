use std::fs;

struct Range {
    start: i32,
    stop: i32,
}

impl Range {
    fn from_range_str(s: &str) -> Self {
        let mut nums = s.split("-");
        let start = nums.next().unwrap().parse::<i32>().unwrap();
        let stop = nums.next().unwrap().parse::<i32>().unwrap();

        return Self { start, stop };
    }
    /// line is like "2-4,6-8"
    fn from_line(line: &str) -> (Self, Self) {
        let mut ranges = line.split(",");
        let range1 = Range::from_range_str(ranges.next().unwrap());
        let range2 = Range::from_range_str(ranges.next().unwrap());

        return (range1, range2);
    }

    fn contains(&self, other: &Self) -> bool {
        return (self.start >= other.start && self.stop <= other.stop)
            || (self.start <= other.start && self.stop >= other.stop);
    }

    fn overlaps(&self, other: &Self) -> bool {
        return !((self.stop < other.start) || (self.start > other.stop));
    }
}

fn main() {
    let input = fs::read_to_string("inputs/4.txt").unwrap();
    let sum = input
        .lines()
        .map(|line| {
            let (r1, r2) = Range::from_line(line);
            if r1.contains(&r2) || r2.contains(&r1) {
                return 1;
            }
            return 0;
        })
        .sum::<i32>();
    println!("{sum}");
    let sum = input
        .lines()
        .map(|line| {
            let (r1, r2) = Range::from_line(line);
            if r1.overlaps(&r2) || r2.overlaps(&r1) {
                return 1;
            }
            return 0;
        })
        .sum::<i32>();
    println!("{sum}");
}
