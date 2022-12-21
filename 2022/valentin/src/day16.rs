use std::{collections::VecDeque, fmt::Debug, iter::repeat, str::FromStr};
struct Valves {
    names: Vec<String>,
    pressures: Vec<usize>,
    dists: Vec<Vec<(usize, usize)>>,
}

impl Debug for Valves {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pstr = self
            .pressures
            .iter()
            .enumerate()
            .fold("    ".to_owned(), |acc, (i, p)| {
                format!("{acc}{} : {p}\n    ", self.names[i])
            });
        let dstr = self
            .dists
            .iter()
            .enumerate()
            .map(|(i, v)| {
                v.iter()
                    .map(|(j, dist)| format!("{} {} -> {dist}", self.names[i], self.names[*j]))
                    .fold(String::new(), |acc, s| format!("{s}   {acc}"))
            })
            .fold("    ".to_owned(), |acc, s| format!("{acc}{s}\n    "));
        write!(f, "pressures = [\n{}]\ndists = [\n{}\n]", pstr, dstr)
    }
}

impl FromStr for Valves {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut neighbors: Vec<Vec<String>> = Vec::new();
        let mut pressures = Vec::new();
        let names: Vec<String> = s
            .split('\n')
            .map(|line| {
                let mut it = line.split(' ');
                let valve = it.nth(1).unwrap().to_owned();
                let pressure: usize = it
                    .nth(2)
                    .unwrap()
                    .split('=')
                    .last()
                    .unwrap()
                    .replace(";", "")
                    .parse()
                    .unwrap();
                pressures.push(pressure);
                it.nth(3);
                neighbors.push(it.map(|o| o.replace(",", "")).collect());
                valve
            })
            .collect();

        let neighbors: Vec<Vec<usize>> = neighbors
            .into_iter()
            .map(|v| {
                v.iter()
                    .map(|name| names.iter().position(|n| n == name).unwrap())
                    .collect()
            })
            .collect();
        let mut starts = vec![names.iter().position(|n| n == "AA").unwrap()];
        starts.extend((1..(names.len())).filter(|i| pressures[*i] != 0));

        let mut res = Valves {
            names,
            pressures,
            dists: neighbors.iter().map(|_| Vec::new()).collect(),
        };

        for start in starts {
            res.init_dist(start, &neighbors);
        }

        Ok(res)
    }
}

impl Valves {
    fn init_dist(&mut self, start: usize, neighbors: &Vec<Vec<usize>>) {
        let mut visited: Vec<usize> = vec![start];
        let mut queue = VecDeque::from([(start, 0)]);
        while let Some((cur_index, d_from_start)) = queue.pop_front() {
            for adjacent in neighbors[cur_index].iter() {
                if !visited.contains(adjacent) {
                    let next = (*adjacent, d_from_start + 1);
                    if self.pressures[*adjacent] > 0 {
                        self.dists[start].push(next);
                    }
                    queue.push_back(next);
                    visited.push(*adjacent);
                }
            }
        }
    }

    fn bruteforce(&self, time_budget: usize) -> (usize, Vec<usize>) {
        let mut pt_max = 0;
        let mut best_path = vec![];
        let start = self.names.iter().position(|n| n == "AA").unwrap();
        let mut stack = vec![(start, 0, 0, vec![start])];
        while let Some((cur_index, pressure_total, time, prevs)) = stack.pop() {
            if time >= time_budget {
                pt_max = pt_max.max(pressure_total);
                best_path = prevs;
                continue;
            }
            let mut no_change = true;
            for (adj_index, dist) in self.dists[cur_index].iter() {
                let next_time = time + dist + 1;
                if (!prevs.contains(adj_index)) && (next_time <= time_budget) {
                    let mut path = prevs.clone();
                    path.push(*adj_index);
                    let pressure_gain = self.pressures[*adj_index] * (time_budget - next_time);
                    let pt = pressure_total + pressure_gain;
                    stack.push((*adj_index, pt, next_time, path));
                    no_change = false;
                }
            }
            if no_change {
                pt_max = pt_max.max(pressure_total);
                best_path = prevs;
            }
        }
        (pt_max, best_path)
    }

    fn update_bests_scores(
        &self,
        known_best: &mut Vec<usize>,
        start: usize,
        time_budget: usize,
        path1: &Vec<usize>,
        path2: &Vec<usize>,
        cache_dists: &Vec<Vec<usize>>,
    ) {
        for a in known_best.iter_mut() {
            *a = 0;
        }
        let mut prev = start;
        let mut t = 0;
        let mut timed_pressures: Vec<(usize, usize)> = path1
            .iter()
            .map(|&cur| {
                t += cache_dists[prev][cur] + 1;
                prev = cur;
                (t, self.pressures[cur] * (time_budget - t))
            })
            .collect();
        prev = start;
        t = 0;
        timed_pressures.extend(path2.iter().map(|&cur| {
            t += cache_dists[prev][cur] + 1;
            prev = cur;
            (t, self.pressures[cur] * (time_budget - t))
        }));
        timed_pressures.sort();
        for (t, press) in timed_pressures {
            for i in t..(known_best.len()) {
                known_best[i] += press;
            }
        }
    }

    fn bruteforce_with_elephant(&self, time_budget: usize) -> usize {
        let useful_valves: Vec<usize> = self
            .pressures
            .iter()
            .enumerate()
            .filter_map(|(i, p)| if *p > 0 { Some(i) } else { None })
            .collect();
        let cache_dists: Vec<Vec<usize>> = ((0..(self.pressures.len())).map(|i| {
            let mut res: Vec<usize> = repeat(0).take(self.pressures.len()).collect();
            for (j, dist) in self.dists[i].iter() {
                res[*j] = *dist;
            }
            res
        }))
        .collect();
        let mut known_best: Vec<usize> = repeat(0).take(time_budget + 1).collect();
        let mut pts_max = (0, 0);
        let index_init = self.names.iter().position(|name| name == "AA").unwrap();
        let mut best_paths = (vec![], vec![]);
        let start = (index_init, 0, 0, vec![]);
        let mut queue = vec![(start.clone(), start, 0)];
        while let Some((your_info, elephant_info, min_pt_at_tmin)) = queue.pop() {
            let (y_cur, y_pt, y_time, y_prevs) = your_info.clone();
            let (e_cur, e_pt, e_time, e_prevs) = elephant_info.clone();
            let tmin = e_time.min(y_time);
            let e_is_min = e_time == tmin;
            if known_best[tmin] > min_pt_at_tmin {
                continue;
            }
            if y_time >= time_budget && e_time >= time_budget {
                if pts_max.0 + pts_max.1 < y_pt + e_pt {
                    pts_max = (y_pt, e_pt);
                    self.update_bests_scores(
                        &mut known_best,
                        index_init,
                        time_budget,
                        &y_prevs,
                        &e_prevs,
                        &cache_dists,
                    );
                    best_paths = (y_prevs, e_prevs);
                }
                continue;
            }
            let mut no_change = true;
            let candidates: Vec<usize> = useful_valves
                .iter()
                .filter_map(|nei| {
                    if (!y_prevs.contains(nei)) && (!e_prevs.contains(nei)) {
                        Some(*nei)
                    } else {
                        None
                    }
                })
                .collect();

            for y_nei in candidates.iter() {
                let y_new_time = y_time + cache_dists[y_cur][*y_nei] + 1;
                if y_new_time > time_budget {
                    continue;
                }
                let gain = self.pressures[*y_nei] * (time_budget - y_new_time);
                let new_y_pt = y_pt + gain;

                let mut new_y_path = y_prevs.clone();
                new_y_path.push(*y_nei);
                let new_min_pt = if e_is_min {
                    min_pt_at_tmin
                } else {
                    min_pt_at_tmin + gain
                };
                queue.push((
                    (*y_nei, new_y_pt, y_new_time, new_y_path),
                    elephant_info.clone(),
                    new_min_pt,
                ));
                no_change = false;
            }

            for e_nei in candidates.iter() {
                let next_time = e_time + cache_dists[e_cur][*e_nei] + 1;
                if next_time > time_budget {
                    continue;
                }
                let gain = self.pressures[*e_nei] * (time_budget - next_time);
                let pt = e_pt + gain;
                let mut path = e_prevs.clone();
                path.push(*e_nei);
                let new_min_pt = if e_is_min {
                    min_pt_at_tmin + gain
                } else {
                    min_pt_at_tmin
                };
                queue.push((your_info.clone(), (*e_nei, pt, next_time, path), new_min_pt));
                no_change = false;
            }

            if no_change {
                if pts_max.0 + pts_max.1 < y_pt + e_pt {
                    pts_max = (y_pt, e_pt);
                    self.update_bests_scores(
                        &mut known_best,
                        index_init,
                        time_budget,
                        &y_prevs,
                        &e_prevs,
                        &cache_dists,
                    );
                    best_paths = (y_prevs, e_prevs);
                }
            }
        }
        let _path: Vec<&String> = best_paths.0.into_iter().map(|i| &self.names[i]).collect();
        println!("Your path {:?}", _path);
        println!("Your score = {}", pts_max.0);
        let _path: Vec<&String> = best_paths.1.into_iter().map(|i| &self.names[i]).collect();
        println!("Elephant path {:?}", _path);
        println!("Elephant score = {}", pts_max.1);
        pts_max.0 + pts_max.1
    }
}

pub fn part1(input: String) -> usize {
    let valves: Valves = input.trim().parse().unwrap();
    let (res, _path) = valves.bruteforce(30);
    let _path: Vec<&String> = _path.into_iter().map(|i| &valves.names[i]).collect();
    println!("{:?}", _path);
    res
}

pub fn part2(input: String) -> usize {
    let valves: Valves = input.trim().parse().unwrap();
    valves.bruteforce_with_elephant(26)
}

pub const EXPECTED1: usize = 1651;
pub const EXPECTED2: usize = 1707;
