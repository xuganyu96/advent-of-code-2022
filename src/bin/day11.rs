// use std::fs;
use std::collections::VecDeque;

struct Game {
    items: Vec<VecDeque<i128>>,  // list of monkeys, where each monkey is a list of worry levels
    modulos: Vec<i128>,   // list of moduloes
    operations: Vec<Box<dyn Fn(i128) -> i128>>,
    targets: Vec<(usize, usize)>,  // target monkey if true, if false
    business: Vec<i128>,  // count of number of inspection per monkey
    level_divisor: i128,
    lcm: i128,
}

impl Game {
    // TODO: need to parse inputs from text instead of hardcoding them!
    fn prod(level_divisor: i128) -> Self {
        let items = vec![
            VecDeque::from(vec![59, 74, 65, 86]),
            VecDeque::from(vec![62, 84, 72, 91, 68, 78, 51]),
            VecDeque::from(vec![78, 84, 96]),
            VecDeque::from(vec![97, 86]),
            VecDeque::from(vec![50]),
            VecDeque::from(vec![73, 65, 69, 65, 51]),
            VecDeque::from(vec![69, 82, 97, 93, 82, 84, 58, 63]),
            VecDeque::from(vec![81, 78, 82, 76, 79, 80]),
        ];

        let modulos = vec![7, 2, 19, 3, 13, 11, 5, 17];
        let mut lcm = 1;
        for m in &modulos { lcm *= m };
        let operations = vec![
            Box::new(|x| x * 19i128) as Box<dyn Fn(i128) -> i128>,
            Box::new(|x| x + 1),
            Box::new(|x| x + 8),
            Box::new(|x| x * x),
            Box::new(|x| x + 6),
            Box::new(|x| x * 17),
            Box::new(|x| x + 5),
            Box::new(|x| x + 3),
        ];
        let targets = vec![
            (6, 2), (2, 0), (6, 5), (1, 0), (3, 1), (4, 7), (5, 7), (3, 4),
        ];
        let business = Vec::from([0; 8]);

        return Self { items, modulos, operations, targets, business, level_divisor, lcm };
    }

    /// for monkey i, inspect all items in order, and make throws accordingly
    fn turn(&mut self, i: usize) {
        let modulo = self.modulos.get(i).unwrap();
        let items = self.items.get_mut(i).unwrap();
        let business_delta = items.len();
        let mut levels = vec![];
        let func = self.operations.get(i).unwrap();
        // pop current set of items and record the final worry levels
        while let Some(top) = items.pop_front() {
            levels.push(func(top) / self.level_divisor % self.lcm);
        }

        let (true_t, false_t) = self.targets.get(i).unwrap();
        levels.iter()
            .for_each(|level| {
                match level % modulo {
                    0 => {
                        let target_items = self.items.get_mut(*true_t).unwrap();
                        target_items.push_back(*level);
                    },
                    _ => {
                        let target_items = self.items.get_mut(*false_t).unwrap();
                        target_items.push_back(*level);
                    },
                }
            });
        
        // update business
        let business = self.business.get_mut(i).unwrap();
        *business += business_delta as i128;
    }

    fn round(&mut self) {
        for i in 0..self.items.len() {
            self.turn(i);
        }
    }
}

fn main() {
    // let input = fs::read_to_string("inputs/11.txt").unwrap();
    let mut game = Game::prod(3);
    for _ in 0..20 {
        game.round();
    }
    let mut business_sorted = game.business.clone();
    business_sorted.sort_by_key(|count| -count);
    println!("{}", business_sorted[0] * business_sorted[1]);

    let mut game = Game::prod(1);
    for _ in 0..10000 {
        game.round();
    }
    let mut business_sorted = game.business.clone();
    business_sorted.sort_by_key(|count| -count);
    println!("{}", business_sorted[0] * business_sorted[1]);
}
