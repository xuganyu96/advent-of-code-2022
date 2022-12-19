//! Day 19: Not Enough Minerals
use std::collections::HashMap;
use std::fs;

enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug)]
struct Cost {
    ore: i32,
    obsidian: i32,
    clay: i32,
}

impl Cost {
    fn new(ore: i32, obsidian: i32, clay: i32) -> Self {
        return Self {
            ore,
            obsidian,
            clay,
        };
    }
}

#[derive(Debug)]
struct Blueprint {
    ore_bots_cost: Cost,
    clay_bots_cost: Cost,
    obsidian_bots_cost: Cost,
    geode_bots_cost: Cost,
}

impl Blueprint {
    fn new(
        ore_bots_cost: Cost,
        clay_bots_cost: Cost,
        obsidian_bots_cost: Cost,
        geode_bots_cost: Cost,
    ) -> Self {
        return Self {
            ore_bots_cost,
            clay_bots_cost,
            obsidian_bots_cost,
            geode_bots_cost,
        };
    }

    fn from_line(line: &str) -> Self {
        let tokens: Vec<&str> = line.split(" ").collect();
        let ore_bots_cost = Cost::new(tokens[6].parse::<i32>().unwrap(), 0, 0);
        let clay_bots_cost = Cost::new(tokens[12].parse::<i32>().unwrap(), 0, 0);
        let obsidian_bots_cost = Cost::new(
            tokens[18].parse::<i32>().unwrap(),
            0,
            tokens[21].parse::<i32>().unwrap(),
        );
        let geode_bots_cost = Cost::new(
            tokens[27].parse::<i32>().unwrap(),
            tokens[30].parse::<i32>().unwrap(),
            0,
        );
        return Self::new(
            ore_bots_cost,
            clay_bots_cost,
            obsidian_bots_cost,
            geode_bots_cost,
        );
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct State {
    ore_bots: i32,
    clay_bots: i32,
    obsidian_bots: i32,
    geode_bots: i32,
    ore: i32,
    clay: i32,
    obsidian: i32,
    geode: i32,
}

impl State {
    fn can_build_bot(&self, resource: &Resource, blueprint: &Blueprint) -> bool {
        let cost = match resource {
            Resource::Ore => &blueprint.ore_bots_cost,
            Resource::Clay => &blueprint.clay_bots_cost,
            Resource::Obsidian => &blueprint.obsidian_bots_cost,
            Resource::Geode => &blueprint.geode_bots_cost,
        };

        return self.ore >= cost.ore && self.clay >= cost.clay && self.obsidian >= cost.obsidian;
    }

    /// If the current state already has as many or more bots of input resource than the highest
    /// cost of of this resource to build any bot, then you should not build this bot
    fn should_build_bot(&self, resource: &Resource, blueprint: &Blueprint) -> bool {
        let (max_cost, cur_output) = match resource {
            Resource::Ore => {
                let _max_cost = blueprint.ore_bots_cost.ore;
                let _max_cost = _max_cost.max(blueprint.clay_bots_cost.ore);
                let _max_cost = _max_cost.max(blueprint.obsidian_bots_cost.ore);
                let _max_cost = _max_cost.max(blueprint.geode_bots_cost.ore);
                (_max_cost, self.ore_bots)
            }
            Resource::Clay => (blueprint.obsidian_bots_cost.clay, self.clay_bots),
            Resource::Obsidian => (blueprint.geode_bots_cost.obsidian, self.obsidian_bots),
            _ => (0, 0),
        };
        return cur_output < max_cost;
    }

    fn build_bot(&self, resource: &Resource, blueprint: &Blueprint) -> Self {
        let (cost, bot_deltas) = match resource {
            Resource::Ore => (&blueprint.ore_bots_cost, (1, 0, 0, 0)),
            Resource::Clay => (&blueprint.clay_bots_cost, (0, 1, 0, 0)),
            Resource::Obsidian => (&blueprint.obsidian_bots_cost, (0, 0, 1, 0)),
            Resource::Geode => (&blueprint.geode_bots_cost, (0, 0, 0, 1)),
        };

        return State {
            ore: self.ore + self.ore_bots - cost.ore,
            clay: self.clay + self.clay_bots - cost.clay,
            obsidian: self.obsidian + self.obsidian_bots - cost.obsidian,
            geode: self.geode + self.geode_bots,
            ore_bots: self.ore_bots + bot_deltas.0,
            clay_bots: self.clay_bots + bot_deltas.1,
            obsidian_bots: self.obsidian_bots + bot_deltas.2,
            geode_bots: self.geode_bots + bot_deltas.3,
        };
    }

    fn idle(&self) -> Self {
        return State {
            ore: self.ore + self.ore_bots,
            clay: self.clay + self.clay_bots,
            obsidian: self.obsidian + self.obsidian_bots,
            geode: self.geode + self.geode_bots,
            ore_bots: self.ore_bots,
            clay_bots: self.clay_bots,
            obsidian_bots: self.obsidian_bots,
            geode_bots: self.geode_bots,
        };
    }
}

/// Given the starting state, return the maximum number of geode that can be
/// obtained given the remaining amount of time
fn dfs(
    start: State,
    blueprint: &Blueprint,
    t_remain: i32,
    memo: &mut HashMap<(State, i32), i32>,
    gmax: &mut i32,
) -> i32 {
    if let Some(max_geode) = memo.get(&(start.clone(), t_remain)) {
        return *max_geode;
    }
    if t_remain <= 0 {
        *gmax = (*gmax).max(start.geode);
        return start.geode;
    }
    let max_potential = start.geode + (start.geode_bots + start.geode_bots + t_remain - 1) * t_remain / 2; 
    if max_potential < *gmax {
        return start.geode;
    }
    // you can: do nothing, build an ore bot, build a clay bot, build an
    // obsidian bot, build a geode bot
    let mut max_geode = start.geode;

    if start.can_build_bot(&Resource::Geode, blueprint) {
        let next_state = start.build_bot(&Resource::Geode, blueprint);
        max_geode = max_geode.max(dfs(next_state, blueprint, t_remain - 1, memo, gmax));
        return max_geode;
    }
    for resource in [Resource::Obsidian, Resource::Ore, Resource::Clay] {
        if start.can_build_bot(&resource, blueprint) 
            && start.should_build_bot(&resource, blueprint)
        {
            let next_state = start.build_bot(&resource, blueprint);
            max_geode = max_geode.max(dfs(next_state, blueprint, t_remain - 1, memo, gmax));
        }
    }
    max_geode = max_geode.max(dfs(start.idle(), blueprint, t_remain - 1, memo, gmax));
    memo.insert((start, t_remain), max_geode);
    return max_geode;
}

fn main() {
    let inputs = fs::read_to_string("inputs/19.txt").unwrap();
    let mut blueprints = vec![];
    for line in inputs.lines() {
        blueprints.push(Blueprint::from_line(line));
    }

    let init_state = State {
        ore: 0,
        clay: 0,
        obsidian: 0,
        geode: 0,
        ore_bots: 1,
        clay_bots: 0,
        obsidian_bots: 0,
        geode_bots: 0,
    };
    let quality_score = blueprints
        .iter()
        .enumerate()
        .map(|(i, bp)| {
            let mut memo = HashMap::new();
            let mut gmax = 0;
            let max_geode = dfs(init_state.clone(), bp, 24, &mut memo, &mut gmax);
            return (i as i32 + 1) * max_geode;
        })
        .sum::<i32>();
    println!("{quality_score}");

    let mut max_geodes = vec![];
    blueprints.iter().take(3).enumerate().for_each(|(i, bp)| {
        let mut memo = HashMap::new();
        let mut gmax = 0;
        let max_geode = dfs(init_state.clone(), bp, 32, &mut memo, &mut gmax);
        max_geodes.push(max_geode);
    });
    let mut product = 1;
    max_geodes.iter().for_each(|max_geode| product *= max_geode);
    println!("{product}");
}
