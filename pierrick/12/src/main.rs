use std::collections::{HashMap, HashSet};

#[derive(Default, Debug)]
struct CaveMap {
    paths: HashMap<String, HashSet<String>>,
}

impl CaveMap {
    fn add_connection(&mut self, left: &str, right: &str) {
        let left = String::from(left);
        let right = String::from(right);
        self.paths
            .entry(left.clone())
            .or_insert_with(HashSet::new)
            .insert(right.clone());
        self.paths
            .entry(right)
            .or_insert_with(HashSet::new)
            .insert(left);
    }

    fn is_small_cave(name: &str) -> bool {
        name.chars().next().unwrap().is_lowercase()
    }

    fn explore_paths(&self, start: &str, end: &str) -> HashSet<Vec<String>> {
        self.recursively_find_path(start, end, &[])
    }

    fn recursively_find_path(
        &self,
        start: &str,
        end: &str,
        visited: &[String],
    ) -> HashSet<Vec<String>> {
        let mut visited = visited.to_owned();
        visited.push(String::from(start));

        //println!("from {} to {}", start, end);

        if start == end {
            HashSet::from_iter([visited])
        } else {
            let mut res = HashSet::new();

            for dest in self.paths[start].iter() {
                if Self::is_small_cave(dest) && visited.iter().any(|c| c == dest) {
                    continue;
                }
                let new_paths = self.recursively_find_path(dest, end, &visited);
                res.extend(new_paths);
            }

            res
        }
    }
}

fn get_input(s: &str) -> CaveMap {
    let mut res = CaveMap::default();
    for connection in s.lines().filter(|s| !s.is_empty()) {
        let connection: Vec<&str> = connection.split('-').collect();
        let left = connection[0];
        let right = connection[1];

        res.add_connection(left, right);
    }
    res
}

fn part1(s: &str) -> usize {
    let cave = get_input(s);
    cave.explore_paths("start", "end").len()
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
}
