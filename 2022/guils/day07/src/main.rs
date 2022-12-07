use itertools::Itertools;
use std::collections::HashMap;

type PathMap = HashMap<String, i32>;

//const INPUT: &str = include_str!("../test.txt");
const INPUT: &str = include_str!("../input.txt");

fn read_input() -> Vec<String> {
    INPUT.trim().split('\n').map(str::to_string).collect()
}

fn parse_fs(commands: &[String]) -> PathMap {
    let mut path = Vec::<String>::new();
    let mut path_map = PathMap::new();
    commands
        .iter()
        .map(|line| {
            let cmd = line.split(' ').collect_vec();
            match cmd[0] {
                "$" => {
                    if cmd[1] == "cd" {
                        if cmd[2] == "/" {
                            path.clear();
                        } else if cmd[2] == ".." {
                            path.pop().unwrap();
                        } else {
                            path.push(cmd[2].to_string());
                        }
                    }
                    (String::from(""), -1)
                }
                "dir" => (path.join("/"), 0),
                size => (path.join("/"), size.parse::<i32>().unwrap()),
            }
        })
        .filter(|x| x.1 >= 0)
        .map(|(path_str, e_size)| match path_map.get(&path_str) {
            Some(size) => path_map.insert(path_str.clone(), size + e_size),
            None => path_map.insert(path_str.clone(), e_size),
        })
        .collect_vec();
    path_map
}

fn collect_dir_sizes(path_map: &PathMap) -> PathMap {
    let mut dir_map = path_map.clone();
    path_map
        .keys()
        .sorted_by_key(|x| x.len())
        .rev()
        .filter(|&x| x.ne(""))
        .map(|path_str| {
            let size = dir_map.get(path_str).unwrap();
            let parent_str = path_str.split('/').dropping_back(1).join("/");
            match dir_map.get(&parent_str) {
                Some(psize) => {
                    dir_map.insert(parent_str, psize + size);
                }
                None => panic!(),
            }
        })
        .collect_vec();
    dir_map
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
        .sorted()
        .skip_while(|&x| *x < limit)
        .take(1)
        .sum()
}

fn step2() {
    let input = read_input();
    let path_map = parse_fs(&input);
    let dir_map = collect_dir_sizes(&path_map);
    let res = collect_step2(&dir_map);
    println!("step2: {res}");
}

fn main() {
    step1();
    step2();
}
