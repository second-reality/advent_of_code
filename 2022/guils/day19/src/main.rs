use itertools::Itertools;
use std::collections::HashSet;
use std::collections::VecDeque;

//const INPUT: &str = include_str!("../test.txt");
const INPUT: &str = include_str!("../input.txt");
const MAX_TIME_1: i32 = 24;
const MAX_TIME_2: i32 = 32;
const VERBOSE: bool = false;

fn read_input() -> Vec<BPrint> {
    INPUT
        .trim()
        .split('\n')
        .map(|x| {
            let costs = x
                .replace("Blueprint ", "")
                .replace(": Each ore robot costs", "")
                .replace(" ore. Each clay robot costs", "")
                .replace(" ore. Each obsidian robot costs", "")
                .replace(" ore and", "")
                .replace(" clay. Each geode robot costs", "")
                .replace(" obsidian.", "")
                .split(' ')
                .map(|x| x.parse::<i32>().unwrap())
                .collect_vec();
            BPrint {
                r_ors: Cost {
                    ors: costs[1],
                    cls: 0,
                    obs: 0,
                },
                r_cls: Cost {
                    ors: costs[2],
                    cls: 0,
                    obs: 0,
                },
                r_obs: Cost {
                    ors: costs[3],
                    cls: costs[4],
                    obs: 0,
                },
                r_gds: Cost {
                    ors: costs[5],
                    cls: 0,
                    obs: costs[6],
                },
            }
        })
        .collect_vec()
}

#[derive(Debug, Copy, Clone, Default, Hash, Eq, PartialEq)]
struct State {
    time: i32,
    ors: i32,
    cls: i32,
    obs: i32,
    n_ors: i32,
    n_cls: i32,
    n_obs: i32,
    n_gds: i32,
    gds: i32,
}

#[derive(Debug, Copy, Clone, Default)]
struct Cost {
    ors: i32,
    cls: i32,
    obs: i32,
}

#[derive(Debug, Copy, Clone, Default)]
struct BPrint {
    r_ors: Cost,
    r_cls: Cost,
    r_obs: Cost,
    r_gds: Cost,
}

fn walk(bp: &BPrint, max_time: i32) -> i32 {
    let update_items_n = |state: &mut State, n: i32| {
        state.ors += state.n_ors * n;
        state.cls += state.n_cls * n;
        state.obs += state.n_obs * n;
        state.gds += state.n_gds * n;
    };
    let update_items = |state: &mut State| update_items_n(state, 1);
    let push = |states: &mut VecDeque<State>, state: State| states.push_front(state);
    let mut results = Vec::<State>::new();
    let mut visited = HashSet::<State>::new();
    let mut states = VecDeque::<State>::new();
    states.push_front(State {
        n_ors: 1,
        ..State::default()
    });

    let mut gds_times = vec![i32::MAX; max_time as usize];
    let mut obs_times = vec![i32::MAX; max_time as usize];
    let mut cls_times = vec![i32::MAX; max_time as usize];

    assert!(bp.r_gds.obs > bp.r_gds.ors);
    assert!(bp.r_obs.cls > bp.r_obs.ors);
    let (short_gds, short_obs, short_cls, short_build) = (true, false, false, false);
    let (opt_gds, opt_obs, mut opt_cls) = (true, true, true);
    if bp.r_cls.ors < bp.r_obs.ors {
        opt_cls = false;
    }

    while !states.is_empty() {
        let state = states.pop_back().unwrap();
        if state.time >= max_time {
            assert!(state.time == max_time);
            results.push(state);
            continue;
        }
        if visited.contains(&state) {
            continue;
        }
        visited.insert(state);

        let mut building = false;
        if state.ors >= bp.r_gds.ors && state.obs >= bp.r_gds.obs {
            let mut state = state;
            state.time += 1;
            state.ors -= bp.r_gds.ors;
            state.obs -= bp.r_gds.obs;
            update_items(&mut state);
            state.n_gds += 1;
            push(&mut states, state);
            building = true;
            if opt_gds {
                if state.time > gds_times[state.n_gds as usize + 1] {
                    continue;
                }
                if state.time < gds_times[state.n_gds as usize] {
                    gds_times[state.n_gds as usize] = state.time;
                }
            }
            if short_gds {
                continue;
            }
        }
        if state.ors >= bp.r_obs.ors && state.cls >= bp.r_obs.cls {
            let mut state = state;
            state.time += 1;
            state.ors -= bp.r_obs.ors;
            state.cls -= bp.r_obs.cls;
            update_items(&mut state);
            state.n_obs += 1;
            push(&mut states, state);
            building = true;
            if opt_obs {
                if state.time > obs_times[state.n_obs as usize + 1] {
                    continue;
                }
                if state.time < obs_times[state.n_obs as usize] {
                    obs_times[state.n_obs as usize] = state.time;
                }
            }
            if short_obs {
                continue;
            }
        }
        if state.ors >= bp.r_cls.ors {
            let mut state = state;
            state.time += 1;
            state.ors -= bp.r_cls.ors;
            update_items(&mut state);
            state.n_cls += 1;
            push(&mut states, state);
            building = true;
            if opt_cls {
                if state.time > cls_times[state.n_cls as usize + 1] {
                    continue;
                }
                if state.time < cls_times[state.n_cls as usize] {
                    cls_times[state.n_cls as usize] = state.time;
                }
            }
            if short_cls {
                continue;
            }
        }
        if state.ors >= bp.r_ors.ors {
            let mut state = state;
            state.time += 1;
            state.ors -= bp.r_ors.ors;
            update_items(&mut state);
            state.n_ors += 1;
            push(&mut states, state);
            building = true;
        }
        if !short_build || !building {
            let mut state = state;
            state.time += 1;
            update_items_n(&mut state, 1);
            push(&mut states, state);
        }
    }
    if !results.is_empty() {
        results.sort_by(|a, b| b.gds.cmp(&a.gds));
        results[0].gds
    } else {
        0
    }
}

fn step1() {
    let input = read_input();
    let all_res = input
        .iter()
        .map(|x| {
            let res = walk(x, MAX_TIME_1);
            if VERBOSE {
                println!("{res}");
            }
            res
        })
        .collect_vec();
    let res = all_res
        .iter()
        .enumerate()
        .map(|(i, r)| (i + 1) as i32 * r)
        .sum::<i32>();
    println!("step1: {res}");
}

fn step2() {
    let input = read_input();
    let all_res = input
        .iter()
        .take(3)
        .map(|x| {
            let res = walk(x, MAX_TIME_2);
            if VERBOSE {
                println!("{res}");
            }
            res
        })
        .collect_vec();
    let res = all_res.iter().product::<i32>();
    println!("step2: {res}");
}

fn main() {
    step1();
    step2();
}
