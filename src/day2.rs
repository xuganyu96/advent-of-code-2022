use std::fs;

#[derive(Debug)]
enum Hand {
    Rock,  // 1 point
    Paper,  // 2 points
    Scissor,  // 3 points
}

impl Hand {
    fn from_plaintext(c: char) -> Self {
        match c {
            'A' => Hand::Rock,
            'B' => Hand::Paper,
            'C' => Hand::Scissor,
            _ => unreachable!(),
        }
    }

    fn from_ciphertext(c: char) -> Self {
        match c {
            'X' => Hand::Rock,
            'Y' => Hand::Paper,
            'Z' => Hand::Scissor,
            _ => unreachable!(),
        }
    }

    fn from_line(s: &str) -> (Self, Self) {
        let mut chars = s.chars();
        let other = chars.next().unwrap();
        chars.next();
        let self_ = chars.next().unwrap();

        return (Self::from_ciphertext(self_), Self::from_plaintext(other));
    }

    fn from_outcomes(line: &str) -> (Self, Self) {
        let mut chars = line.chars();
        let other = chars.next().unwrap();
        let other = Hand::from_plaintext(other);
        chars.next();
        let outcome = chars.next().unwrap();
        let outcome = match outcome {
            'X' => Hand::from_expected_outcome(&other, 0),
            'Y' => Hand::from_expected_outcome(&other, 3),
            'Z' => Hand::from_expected_outcome(&other, 6),
            _ => unreachable!(),
        };

        return (outcome, other);
    }

    fn shape_score(&self) -> i32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissor => 3,
        }
    }

    fn outcome_score(&self, other: &Hand) -> i32 {
        match (self, other) {
            // draws
            (Hand::Scissor, Hand::Scissor) => 3,
            (Hand::Paper, Hand::Paper) => 3,
            (Hand::Rock, Hand::Rock) => 3,

            // wins
            (Hand::Scissor, Hand::Paper) => 6,
            (Hand::Paper, Hand::Rock) => 6,
            (Hand::Rock, Hand::Scissor) => 6,

            // losses
            _ => 0,
        }
    }

    fn from_expected_outcome(other: &Self, score: i32) -> Self {
        assert!(score == 0 || score == 3 || score == 6);

        match (other, score) {
            (Self::Rock, 0) => Self::Scissor,
            (Self::Scissor, 0) => Self::Paper,
            (Self::Paper, 0) => Self::Rock,
            (Self::Rock, 3) => Self::Rock,
            (Self::Paper, 3) => Self::Paper,
            (Self::Scissor, 3) => Self::Scissor,
            (Self::Rock, 6) => Self::Paper,
            (Self::Paper, 6) => Self::Scissor,
            (Self::Scissor, 6) => Self::Rock,
            _ => unreachable!(),
        }
    }
}

pub fn solve(input: &str) {
    let input = fs::read_to_string(input).unwrap();
    let score = input.lines()
        .map(|line| {
            let (self_hand, other_hand) = Hand::from_line(line);
            return self_hand.shape_score() + self_hand.outcome_score(&other_hand);
        })
        .sum::<i32>();
    println!("{score}");
    let score = input.lines()
        .map(|line| {
            let (self_hand, other_hand) = Hand::from_outcomes(line);
            return self_hand.shape_score() + self_hand.outcome_score(&other_hand);
        })
        .sum::<i32>();
    println!("{score}");
}
