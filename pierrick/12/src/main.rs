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
            .entry(left)
            .or_insert_with(HashSet::new)
            .insert(right);
    }

    fn is_small_cave(name: &str) -> bool {
        name.chars().next().unwrap().is_lowercase()
    }

    fn explore_paths(
        &self,
        start: &str,
        end: &str,
        visited: &[String],
        can_visit: fn(&str, &[String]) -> bool,
    ) -> HashSet<Vec<String>> {
        let mut visited = visited.to_owned();
        visited.push(String::from(start));

        //println!("from {} to {}", start, end);

        if start == end {
            HashSet::from_iter([visited])
        } else {
            let mut res = HashSet::new();

            for dest in self.paths[start].iter() {
                if can_visit(dest, &visited) {
                    let new_paths = self.explore_paths(dest, end, &visited, can_visit);
                    res.extend(new_paths);
                }
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
        res.add_connection(right, left);
    }
    res
}

fn part1(s: &str) -> usize {
    let cave = get_input(s);

    fn can_visit(dest: &str, visited: &[String]) -> bool {
        if CaveMap::is_small_cave(dest) {
            let was_seen = visited.iter().any(|cave| cave == dest);
            !was_seen
        } else {
            true
        }
    }
    cave.explore_paths("start", "end", &[], can_visit).len()
}

fn part2(s: &str) -> usize {
    let cave = get_input(s);

    fn can_visit(dest: &str, visited: &[String]) -> bool {
        if CaveMap::is_small_cave(dest) {
            let small_was_visited_twice = visited
                .iter()
                .filter(|cave| CaveMap::is_small_cave(cave))
                .map(|cave| visited.iter().filter(|c| *c == cave).count())
                .any(|count| count > 1);
            let num_seen = visited.iter().filter(|cave| *cave == dest).count();

            if small_was_visited_twice || dest == "start" || dest == "end" {
                num_seen == 0
            } else {
                num_seen < 2
            }
        } else {
            true
        }
    }
    cave.explore_paths("start", "end", &[], can_visit).len()
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
