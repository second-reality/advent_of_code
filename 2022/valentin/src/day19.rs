use std::{collections::HashSet, num::ParseIntError, str::FromStr};

const N_RESSOURCES: usize = 4;
// const NAMED_RESSOURCES: [&'static str; N_RESSOURCES] = ["ore", "clay", "obsidian", "geode"];
#[derive(Debug)]
struct BluePrint {
    costs: [[u32; N_RESSOURCES]; N_RESSOURCES],
    max_costs: [u32; N_RESSOURCES],
}

impl FromStr for BluePrint {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split(' ');
        let ore_cost0: u32 = words.nth(6).unwrap().parse()?;
        let ore_cost1: u32 = words.nth(5).unwrap().parse()?;
        let ore_cost2: u32 = words.nth(5).unwrap().parse()?;
        let clay_cost2: u32 = words.nth(2).unwrap().parse()?;
        let ore_cost3: u32 = words.nth(5).unwrap().parse()?;
        let obsidian_cost3: u32 = words.nth(2).unwrap().parse()?;
        Ok(BluePrint {
            costs: [
                [ore_cost0, 0, 0, 0],
                [ore_cost1, 0, 0, 0],
                [ore_cost2, clay_cost2, 0, 0],
                [ore_cost3, 0, obsidian_cost3, 0],
            ],
            max_costs: [
                [ore_cost0, ore_cost1, ore_cost2, ore_cost3]
                    .into_iter()
                    .max()
                    .unwrap(),
                clay_cost2,
                obsidian_cost3,
                u32::MAX,
            ],
        })
    }
}
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Game {
    robots: [u32; N_RESSOURCES],
    ressources: [u32; N_RESSOURCES],
    timing: u32,
}

impl Game {
    fn new() -> Self {
        Game {
            robots: [1, 0, 0, 0],
            ressources: [0, 0, 0, 0],
            timing: 0,
        }
    }

    fn craft_options(&self, blueprint: &BluePrint) -> Vec<usize> {
        blueprint
            .costs
            .iter()
            .enumerate()
            .filter_map(|(i, cost)| {
                // craft robot that you have enough ressources to build
                // but dont do it if you already have max production (since you can only 1 robot
                // per turn)
                if cost.iter().zip(self.ressources).all(|(c, r)| *c <= r)
                    && self.robots[i] < blueprint.max_costs[i]
                {
                    Some(i)
                } else {
                    None
                }
            })
            .collect()
    }

    fn production(&mut self) {
        for i in 0..N_RESSOURCES {
            self.ressources[i] += self.robots[i];
        }
    }

    fn max_geodes(self, time_budget: u32, blueprint: &BluePrint) -> usize {
        let mut stack = vec![self];
        let mut visited: HashSet<Game> = HashSet::new();
        let mut geodes = 0;
        while let Some(mut game) = stack.pop() {
            if visited.contains(&game) {
                continue;
            }
            visited.insert(game.clone());
            let mut options = Vec::new();
            while options.is_empty() && game.timing < time_budget {
                options = game.craft_options(&blueprint);
                game.production();
                game.timing += 1;
            }

            if game.timing >= time_budget {
                geodes = geodes.max(game.ressources[N_RESSOURCES - 1]);
                // println!("max = {geodes}");
                // println!("game = {:?}", game);
                continue;
            }
            if game.ressources[N_RESSOURCES - 1]
                + (time_budget - game.timing) * game.robots[N_RESSOURCES - 1]
                + (time_budget - game.timing) * (time_budget - game.timing + 1) / 2
                < geodes
            {
                continue;
            }
            // println!("------------------------");
            // println!("options = {:?}", options);
            // println!("max = {geodes}");
            // println!("{:?}", game);
            // println!("{:?}", blueprint);
            //
            stack.push(game.clone());
            for robot_index in options {
                let mut new_game = game.clone();
                new_game.robots[robot_index] += 1;
                for (i, cost) in blueprint.costs[robot_index].iter().enumerate() {
                    new_game.ressources[i] -= cost;
                }
                stack.push(new_game);
            }
        }
        geodes as usize
    }
}

pub fn part1(input: String) -> usize {
    input
        .trim()
        .split('\n')
        .enumerate()
        .map(|(i, line)| {
            let blueprint: BluePrint = line.parse().unwrap();
            let game = Game::new();
            let geodes = game.max_geodes(24, &blueprint);
            println!("Blueprint {}, best geodes = {}", i + 1, geodes);
            (i + 1) * geodes
        })
        .sum()
}

pub fn part2(input: String) -> usize {
    input
        .trim()
        .split('\n')
        .take(3)
        .map(|line| {
            let blueprint: BluePrint = line.parse().unwrap();
            let game = Game::new();
            let geodes = game.max_geodes(32, &blueprint);
            println!("Blueprint, best geodes = {}", geodes);
            geodes
        })
        .product()
}

pub const EXPECTED1: usize = 33;
pub const EXPECTED2: usize = 56 * 62;
