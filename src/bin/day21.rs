use std::fs;
use std::collections::HashMap;

enum Monkey {
    Num(i128),
    // left, right, operator_str, func
    Func(String, String, String, Box<dyn Fn(i128, i128) -> i128>),
}

impl Monkey {
    fn from_line(line: &str) -> (String, Self) {
        let (monkey, oper) = line.split_once(": ").unwrap();
        let monkey = monkey.to_string();
        if let Ok(num) = oper.parse::<i128>() {
            return (monkey, Self::Num(num));
        } else {
            let mut tokens = oper.split(" ");
            let oper1 = tokens.next().unwrap().to_string();
            let operator = tokens.next().unwrap();
            let oper2 = tokens.next().unwrap().to_string();
           
            let func = match operator {
                "+" => Box::new(|a, b| a + b) as Box<dyn Fn(i128, i128) -> i128>,
                "-" => Box::new(|a, b| a - b),
                "*" => Box::new(|a, b| a * b),
                "/" => Box::new(|a, b| {
                    if a % b != 0 {
                        // println!("Non integral division");
                    }
                    a / b}),
                _ => unreachable!("Illegal operator"),
            };
            let operator = operator.to_string();

            return (monkey, Self::Func(oper1, oper2, operator, func));
        }
    }

    fn get_num(&self) -> i128 {
        if let Monkey::Num(val) = self {
            return *val;
        }
        panic!("Monkey not evaluated yet");
    }
}

fn dfs(
    node: &str, 
    adj: &HashMap::<String, Vec<String>>, 
    statuses: &mut HashMap::<String, i128>, 
    toposort: &mut Vec<String>
) {
    if *statuses.get(node).unwrap() == 1 {
        panic!("Cycle");
    } else if *statuses.get(node).unwrap() == 0 {
        statuses.insert(node.to_string(), 1);
        for neighbor in adj.get(node).unwrap() {
            dfs(neighbor, adj, statuses, toposort);
        }
        toposort.insert(0, node.to_string());
        statuses.insert(node.to_string(), 2);
    }
}

fn topological_sort(nodes: &[String], adj: &HashMap<String, Vec<String>>) -> Vec<String> {
    let mut statuses = HashMap::new();
    let mut sorted = vec![];

    for node in nodes {
        statuses.insert(node.to_string(), 0);
    }

    for node in nodes {
        dfs(node, adj, &mut statuses, &mut sorted);
    }

    return sorted;
}

fn parse_inputs(inputs: &str) -> (HashMap<String, Monkey>, HashMap<String, Vec<String>>, Vec<String>) {
    let mut monkeys = HashMap::new();
    let mut adj = HashMap::new();
    let mut names = vec![];

    inputs.lines()
        .for_each(|line| {
            let (name, monkey) = Monkey::from_line(line);

            match &monkey {
                Monkey::Num(_) => {
                    adj.insert(name.clone(), Vec::new());
                },
                Monkey::Func(left, right, _, _) => {
                    adj.insert(name.clone(), vec![left.clone(), right.clone()]);
                },
            }
            names.push(name.clone());
            monkeys.insert(name.clone(), monkey);
        });

    return (monkeys, adj, names);
}

fn evaluate_monkeys(
    monkeys: &mut HashMap<String, Monkey>, 
    sorted: &[String],
) -> i128 {
    sorted.iter()
        .for_each(|name| {
            let monkey = monkeys.get(name).unwrap();
            if let Monkey::Func(left, right, _oper_str, func) = monkey {
                let left_val = monkeys.get(left).unwrap().get_num();
                let right_val = monkeys.get(right).unwrap().get_num();
                let val = func(left_val, right_val);
                // println!("{name}: {left}({left_val}) {_oper_str} {right}({right_val}) = {val}");
                monkeys.insert(name.to_string(), Monkey::Num(val));
            } else {
                // println!("{name}: {}", monkey.get_num());
            }
        });
    return monkeys.get("root").unwrap().get_num();
}

fn evaluate_equality(inputs: &str, human_input: i128) -> i128 {
    let (mut monkeys, adj, names) = parse_inputs(&inputs);
    let mut sorted = topological_sort(&names, &adj);
    sorted.reverse();
    let equality: Box<dyn Fn(i128, i128) -> i128> = Box::new(|a, b| a - b);
    let root = monkeys.get("root").unwrap();
    let (mut left, mut right) = ("foo".to_string(), "bar".to_string());
    if let Monkey::Func(old_left, old_right, _, _) = root {
        left = old_left.clone();
        right = old_right.clone();
    }
    monkeys.insert("root".to_string(), Monkey::Func(left, right, "=".to_string(), equality));
    monkeys.insert("humn".to_string(), Monkey::Num(human_input));

    let root_num = evaluate_monkeys(&mut monkeys, &sorted);
    return root_num;
}

/// Can confirm only +, -, *, /
fn main() {
    let inputs = fs::read_to_string("inputs/21.txt").unwrap();
    // part 1
    let (mut monkeys, adj, names) = parse_inputs(&inputs);
    let mut sorted = topological_sort(&names, &adj);
    sorted.reverse();
    let root_num = evaluate_monkeys(&mut monkeys, &sorted);
    println!("{root_num}");

    // part 2
    let start = 0;
    let limit = i64::max_value() as i128;
    let mut human_input = start;
    let mut jump = limit / 2;
    while jump > 0 {
        while evaluate_equality(&inputs, human_input + jump) > 0 {
            // println!("{}, {}", human_input + jump, evaluate_equality(&inputs, human_input + jump));
            human_input += jump;
        }
        jump = jump / 2;
    }
    human_input += 1;
    println!("{human_input}");
}
