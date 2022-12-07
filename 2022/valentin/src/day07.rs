use std::collections::{HashMap, HashSet};

fn parse_fs(commands: String) -> (HashSet<String>, HashMap<String, usize>) {
    let mut cur_dir: Vec<&str> = Vec::new();
    let mut directories: HashSet<String> = HashSet::new();
    let mut table: HashMap<String, usize> = HashMap::new();
    for line in commands.trim().split('\n') {
        let parent = cur_dir.join("/");
        if line.starts_with("$ cd") {
            let child = line.split(' ').last().unwrap();
            if child == ".." {
                cur_dir.pop();
            } else {
                cur_dir.push(child);
                directories.insert(cur_dir.join("/"));
                table.entry(cur_dir.join("/")).or_insert(0);
            }
        } else if line.starts_with("$ ls") {
            continue;
        } else {
            // print of the ls inside the cur dir
            let mut info = line.split(' ');
            let size_or_dir = info.next().unwrap();
            if let Ok(size) = size_or_dir.parse::<usize>() {
                let name = info.next().unwrap();
                let child = format!("{}/{}", parent, name);
                // insert file in table
                table.entry(child).or_insert(size);
                // update all parents size
                for i in 0..(cur_dir.len()) {
                    table
                        .entry(cur_dir[0..=i].join("/"))
                        .and_modify(|s| *s += size);
                }
            }
        }
    }
    (directories, table)
}
pub fn part1(input: String) -> usize {
    let (directories, sizes) = parse_fs(input);
    sizes
        .into_iter()
        .filter_map(|(path, size)| {
            if size <= 100_000 && directories.contains(&path) {
                Some(size)
            } else {
                None
            }
        })
        .sum()
}

pub fn part2(input: String) -> usize {
    let (directories, sizes) = parse_fs(input);
    let required = 30_000_000 - (70_000_000 - sizes[&"/".to_owned()]);
    sizes
        .into_iter()
        .filter_map(|(path, size)| {
            if size >= required && directories.contains(&path) {
                Some(size)
            } else {
                None
            }
        })
        .min()
        .unwrap()
}

pub const EXPECTED1: usize = 95_437;
pub const EXPECTED2: usize = 24_933_642;
