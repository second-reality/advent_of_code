use itertools::Itertools;
use std::cmp;
use std::collections::HashMap;
use std::collections::VecDeque;

//const INPUT: &str = include_str!("../test.txt");
const INPUT: &str = include_str!("../input.txt");

type VName = String;
type VId = usize;
type Succs = Vec<VName>;
type Rate = i32;
type VMap = HashMap<VName, (Rate, Succs)>;
type VNames = HashMap<VId, String>;
type VNamesIds = HashMap<String, VId>;
type VGraph = HashMap<(VId, VId), i32>;
type VRates = HashMap<VId, i32>;
type VMem = HashMap<(VId, i32), i32>;
type VPath = Vec<(VId, i32, i32)>;
type VResult = (i32, i32, VPath);

fn read_input() -> VMap {
    INPUT
        .trim()
        .split('\n')
        .map(|l| {
            let toks = l
                .replace("Valve ", "")
                .replace("has flow rate=", "")
                .replace("; tunnels lead to valves", "")
                .replace("; tunnel leads to valve", "")
                .replace(", ", " ")
                .split(' ')
                .map(str::to_string)
                .collect_vec();
            let succs = toks[2..].iter().map(|x| x.to_owned()).collect_vec();
            (toks[0].to_owned(), (toks[1].parse::<i32>().unwrap(), succs))
        })
        .collect()
}

fn get_all_succs(start: VId, graph: &VGraph, rates: &VRates) -> VGraph {
    let mut new_graph = VGraph::new();
    let mut to_visit = graph
        .keys()
        .map(|(a, _)| *a)
        .sorted()
        .dedup()
        .filter(|a| *a == start || rates.get(a).is_some())
        .map(|a| (a, a, 0))
        .collect::<VecDeque<_>>();
    while !to_visit.is_empty() {
        let (start, current, current_d) = to_visit.pop_front().unwrap();
        let next = if current != start {
            let old_d = *new_graph.get(&(start, current)).unwrap_or(&i32::MAX);
            if current_d < old_d {
                new_graph.insert((start, current), current_d);
                Some(current)
            } else {
                None
            }
        } else {
            Some(current)
        };
        if let Some(current) = next {
            to_visit.extend(graph.iter().filter_map(|((a, b), d)| {
                if *a == current {
                    Some((start, *b, current_d + d))
                } else if *b == current {
                    Some((start, *a, current_d + d))
                } else {
                    None
                }
            }));
        }
    }
    new_graph
}

fn prepare_graph(valves: &VMap) -> (VNamesIds, VNames, VGraph, VRates) {
    let names_ids: VNamesIds = valves
        .keys()
        .sorted()
        .enumerate()
        .map(|(id, s)| (s.clone(), id))
        .collect();
    let mut graph = VGraph::new();
    let mut rates = VRates::new();
    for (v, (_, succs)) in valves.iter() {
        let id = names_ids.get(v).unwrap();
        for n in succs.iter() {
            let nid = names_ids.get(n).unwrap();
            graph.insert((*id, *nid), 1);
        }
    }
    for (v, (r, _)) in valves.iter() {
        let id = names_ids.get(v).unwrap();
        if *r > 0 {
            rates.insert(*id, *r);
        }
    }
    let names: VNames = names_ids.iter().map(|(s, id)| (*id, s.clone())).collect();
    (names_ids, names, graph, rates)
}

fn dyn_visit(
    graph: &VGraph,
    rates: &VRates,
    cache: &mut VMem,
    current: VId,
    time: i32,
    pressure: i32,
    path: &VPath,
) -> VResult {
    let mut new_path = path.clone();
    new_path.push((current, time, pressure));
    if time == 0 {
        return (0, pressure, new_path);
    }
    let last_pressure = *cache.get(&(current, time)).unwrap_or(&-1);
    if pressure <= last_pressure {
        return (-1, 0, new_path);
    }
    cache.insert((current, time), pressure);
    let mut cands = Vec::<VResult>::new();
    let succs = graph
        .iter()
        .filter_map(|((a, b), d)| {
            if *a == current && rates.contains_key(b) {
                Some((*b, *d))
            } else {
                None
            }
        })
        .collect_vec();
    for (next, dist) in succs.iter() {
        if time - *dist > 0 {
            let rate = *rates.get(next).unwrap();
            let mut new_rates = rates.clone();
            new_rates.remove(next);
            let (ntime, npressure, full_path) = dyn_visit(
                graph,
                &new_rates,
                cache,
                *next,
                time - *dist - 1,
                pressure + (time - *dist - 1) * rate,
                &new_path,
            );
            if ntime == 0 {
                cands.push((ntime, npressure, full_path));
            }
        }
    }
    if cands.is_empty() {
        cands.push((0, pressure, new_path));
    }
    cands.sort_by(|(_, pa, _), (_, pb, _)| pb.cmp(pa));
    cands[0].to_owned()
}

fn find_pressure(valves: &VMap) -> i32 {
    let (names_ids, _names, graph, rates) = prepare_graph(valves);
    let start = *names_ids.get(&"AA".to_string()).unwrap();
    let graph = get_all_succs(start, &graph, &rates);
    let mut cache = VMem::new();
    let path = VPath::new();
    let (_time, pressure, _path) = dyn_visit(&graph, &rates, &mut cache, start, 30, 0, &path);
    pressure
}

fn step1() {
    let input = read_input();
    let res = find_pressure(&input);
    println!("step1: {res}");
}

#[derive(Default, Debug, Clone, Copy)]
struct PState {
    node: VId,
    time: i32,
    pressure: i32,
}
type PState2 = (Vec<PState>, Vec<PState>);
type PResult2 = (PState, PState);
type VMem2 = HashMap<((VId, VId), i32), i32>;

fn dyn_visit_2(graph: &VGraph, rates: &VRates, cache: &mut VMem2, states: &PState2) -> PResult2 {
    let mut last_state = (states.0[states.0.len() - 1], states.1[states.1.len() - 1]);
    if last_state.0.time == 0 && last_state.1.time == 0 {
        return last_state;
    }

    let find =
        |t: i32, v: &Vec<PState>| Some(v.iter().take_while(|x| x.time >= t).last()?.to_owned());
    let min_time = cmp::max(last_state.0.time, last_state.1.time);
    let s0 = find(min_time, &states.0);
    let s1 = find(min_time, &states.1);
    if let (Some(s0), Some(s1)) = (s0, s1) {
        let pressure = s0.pressure + s1.pressure;
        let pos = (s0.node, s1.node);
        let last_pressure = *cache.get(&(pos, min_time)).unwrap_or(&-1);
        if pressure <= last_pressure {
            last_state.0.time = -1;
            last_state.1.time = -1;
            return last_state;
        }
        cache.insert((pos, min_time), pressure);
    }
    let mut cands = Vec::<PResult2>::new();
    let succs0 = graph
        .iter()
        .filter_map(|((a, b), d)| {
            if *a == last_state.0.node && rates.contains_key(b) {
                Some((*b, *d))
            } else {
                None
            }
        })
        .collect_vec();
    let succs1 = graph
        .iter()
        .filter_map(|((a, b), d)| {
            if *a == last_state.1.node && rates.contains_key(b) {
                Some((*b, *d))
            } else {
                None
            }
        })
        .collect_vec();
    for (next0, dist0) in succs0.iter().copied() {
        for (next1, dist1) in succs1.iter().copied() {
            if next0 == next1 {
                continue;
            }
            if last_state.0.time - dist0 <= 0 && last_state.1.time - dist1 <= 0 {
                continue;
            }
            let (rate0, rate1) = (*rates.get(&next0).unwrap(), *rates.get(&next1).unwrap());
            let mut new_rates = rates.clone();
            let mut new_state = last_state;
            let mut new_states = states.clone();
            if last_state.0.time - dist0 > 0 {
                new_rates.remove(&next0);
                new_state.0.node = next0;
                new_state.0.time -= dist0 + 1;
                new_state.0.pressure += new_state.0.time * rate0;
                new_states.0.push(new_state.0);
            }
            if last_state.1.time - dist1 > 0 {
                new_rates.remove(&next1);
                new_state.1.node = next1;
                new_state.1.time -= dist1 + 1;
                new_state.1.pressure += new_state.1.time * rate1;
                new_states.1.push(new_state.1);
            }
            let full_states = dyn_visit_2(graph, &new_rates, cache, &new_states);
            if full_states.0.time == 0 && full_states.1.time == 0 {
                cands.push(full_states);
            }
        }
    }
    if cands.is_empty() {
        last_state.0.time = 0;
        last_state.1.time = 0;
        cands.push(last_state);
    }
    cands.sort_by(|a, b| (b.0.pressure + b.1.pressure).cmp(&(a.0.pressure + a.1.pressure)));
    cands[0].to_owned()
}

fn find_pressure2(valves: &VMap) -> i32 {
    let (names_ids, _names, graph, rates) = prepare_graph(valves);
    let start = *names_ids.get(&"AA".to_string()).unwrap();
    let graph = get_all_succs(start, &graph, &rates);
    let mut cache = VMem2::new();
    let mut init = (PState::default(), PState::default());
    init.0.node = start;
    init.0.time = 26;
    init.1.node = start;
    init.1.time = 26;
    let states = (vec![init.0], vec![init.1]);
    let results = dyn_visit_2(&graph, &rates, &mut cache, &states);
    results.0.pressure + results.1.pressure
}

fn step2() {
    let input = read_input();
    let res = find_pressure2(&input);
    println!("step2: {res}");
}

fn main() {
    step1();
    step2();
}
