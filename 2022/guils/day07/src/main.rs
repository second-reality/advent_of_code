use itertools::Itertools;
use std::collections::HashMap;

type PathMap = HashMap<String, i32>;

//const INPUT: &str = include_str!("../test.txt");
const INPUT: &str = include_str!("../input.txt");

fn read_input() -> Vec<String> {
    INPUT.trim().split('\n').map(str::to_string).collect()
}

fn parse_fs(commands: &[String]) -> PathMap {
    commands
        .iter()
        .fold(
            (&mut Vec::new(), &mut PathMap::new()),
            |(path, path_map), line| {
                let cmd = line.split(' ').collect_vec();
                if cmd[0] == "$" {
                    if cmd[1] == "cd" {
                        if cmd[2] == "/" {
                            path.clear();
                        } else if cmd[2] == ".." {
                            path.pop();
                        } else {
                            path.push(cmd[2]);
                        }
                    }
                } else {
                    let size = path_map.entry(path.join("/")).or_insert(0);
                    if cmd[0] != "dir" {
                        *size += cmd[0].parse::<i32>().unwrap();
                    }
                }
                (path, path_map)
            },
        )
        .1
        .to_owned()
}

fn collect_dir_sizes(path_map: &PathMap) -> PathMap {
    path_map
        .keys()
        .sorted_by_key(|x| x.len())
        .rev()
        .filter(|x| !x.is_empty())
        .fold(&mut path_map.clone(), |path_map, path_str| {
            let size = path_map.get(path_str).unwrap().to_owned();
            let parent_str = path_str.split('/').dropping_back(1).join("/");
            path_map.entry(parent_str).and_modify(|x| *x += size);
            path_map
        })
        .to_owned()
}

fn collect_step1(path_map: &PathMap) -> i32 {
    path_map.values().filter(|&v| *v <= 100000).sum()
}

fn step1() {
    let input = read_input();
    let path_map = parse_fs(&input);
    let dir_map = collect_dir_sizes(&path_map);
    let res = collect_step1(&dir_map);
    println!("step1: {res}");
}

fn collect_step2(path_map: &PathMap) -> i32 {
    let limit = path_map.get("").unwrap() - 40000000;
    path_map
        .values()
        .filter(|&size| *size >= limit)
        .min()
        .unwrap()
        .to_owned()
}

fn step2() {
    let input = read_input();
    let path_map = parse_fs(&input);
    let dir_map = collect_dir_sizes(&path_map);
    collect_dir_sizes(&dir_map);
    let res = collect_step2(&dir_map);
    println!("step2: {res}");
}

fn main() {
    step1();
    step2();
}
