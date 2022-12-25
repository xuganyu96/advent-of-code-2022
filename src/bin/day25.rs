use std::fs;

fn to_decimal(snafu: &str) -> i64 {
    let mut decimal: i64 = 0;
    let mut base: i64 = 1;
    for digit in snafu.chars().rev() {
        // println!("  {digit} {base}");
        let increment = match digit {
            '=' => -2 * base,
            '-' => -1 * base,
            '0' => 0,
            '1' => 1 * base,
            '2' => 2 * base,
            _ => unreachable!("Illegal digit"),
        };
        decimal += increment;
        base *= 5;
    }
    return decimal;
}

fn to_snafu(decimal: i64) -> String {
    let mut decimal = decimal;
    let mut snafu = String::new();
    while decimal != 0 {
        let rem = decimal.rem_euclid(5);
        let (digit, decrement) = match rem {
            0 => ("0", 0),
            1 => ("1", 1),
            2 => ("2", 2),
            3 => ("=", -2),
            4 => ("-", -1),
            _ => unreachable!(),
        };
        snafu.insert_str(0, digit);
        decimal -= decrement;
        decimal /= 5;
    }
    return snafu;
}

fn main() {
    let inputs = fs::read_to_string("inputs/25.txt").unwrap();
    let decimal = inputs.lines()
        .map(|line| to_decimal(line))
        .sum::<i64>();
    let snafu = to_snafu(decimal);
    println!("{}", snafu);
}
