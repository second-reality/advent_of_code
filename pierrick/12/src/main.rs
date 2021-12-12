use std::collections::{HashMap, HashSet};

#[derive(Default, Clone, Eq, PartialEq, Debug)]
struct Path {
    caves: Vec<String>,
    visited: HashMap<String, usize>,
}

impl Path {
    fn enter_cave(&mut self, s: &str) {
        let s = String::from(s);
        *self.visited.entry(s.clone()).or_default() += 1;
        self.caves.push(s);
    }

    fn exit_cave(&mut self) {
        let left = self.caves.pop().unwrap();
        *self.visited.entry(left).or_default() -= 1;
    }
}

#[derive(Default, Debug)]
struct CaveMap {
    paths: HashMap<String, HashSet<String>>,
}

impl CaveMap {
    fn add_connection(&mut self, left: &str, right: &str) {
        let left = String::from(left);
        let right = String::from(right);
        self.paths.entry(left).or_default().insert(right);
    }

    fn is_small_cave(name: &str) -> bool {
        name.chars().next().unwrap().is_lowercase()
    }

    fn explore_paths(
        &self,
        start: &str,
        end: &str,
        path: &mut Path,
        can_visit: fn(&str, &Path) -> bool,
    ) -> Vec<Path> {
        path.enter_cave(start);

        let res = if start == end {
            vec![path.clone()]
        } else {
            let possibles: Vec<&String> = self.paths[start]
                .iter()
                .filter(|dest| can_visit(dest, path))
                .collect();

            possibles
                .iter()
                .map(|dest| self.explore_paths(dest, end, path, can_visit))
                .fold(Vec::new(), |mut acc, new_paths| {
                    acc.extend(new_paths);
                    acc
                })
        };

        path.exit_cave();

        res
    }
}

fn get_input(s: &str) -> CaveMap {
    let mut res = CaveMap::default();
    for connection in s.lines().filter(|s| !s.is_empty()) {
        let connection: Vec<&str> = connection.split('-').collect();
        let left = connection[0];
        let right = connection[1];

        res.add_connection(left, right);
        res.add_connection(right, left);
    }
    res
}

fn part1(s: &str) -> usize {
    let cave = get_input(s);

    fn can_visit(dest: &str, path: &Path) -> bool {
        if CaveMap::is_small_cave(dest) {
            let num_seen = *path.visited.get(dest).unwrap_or(&0);
            num_seen == 0
        } else {
            true
        }
    }
    cave.explore_paths("start", "end", &mut Path::default(), can_visit)
        .len()
}

fn part2(s: &str) -> usize {
    let cave = get_input(s);

    fn can_visit(dest: &str, path: &Path) -> bool {
        if CaveMap::is_small_cave(dest) {
            let small_was_visited_twice = path
                .visited
                .iter()
                .filter(|(cave, _)| CaveMap::is_small_cave(cave))
                .map(|(_, count)| count)
                .any(|count| *count > 1);
            let num_seen = path.visited.get(dest).unwrap_or(&0).to_owned();

            if small_was_visited_twice || dest == "start" || dest == "end" {
                num_seen == 0
            } else {
                num_seen < 2
            }
        } else {
            true
        }
    }
    cave.explore_paths("start", "end", &mut Path::default(), can_visit)
        .len()
}

fn main() {
    let test1 = "
start-A
start-b
A-c
A-b
b-d
A-end
b-end";
    let test2 = "
dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";
    let test3 = "
fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW
";
    assert_eq!(10, part1(test1));
    assert_eq!(19, part1(test2));
    assert_eq!(226, part1(test3));
    println!("{}", part1(include_str!("../input")));
    assert_eq!(36, part2(test1));
    assert_eq!(103, part2(test2));
    assert_eq!(3509, part2(test3));
    println!("{}", part2(include_str!("../input")));
}
