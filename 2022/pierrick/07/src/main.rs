use std::collections::{HashMap, HashSet};

enum INode {
    Directory(Directory),
    File(File),
}

type Path = Vec<String>;

struct Directory {
    path: Path,
}

#[derive(PartialEq, Eq, Hash)]
struct File {
    path: Path,
    size: usize,
}

struct FileSystem {
    nodes: HashMap<Path, INode>,
    cwd: Path,
}

impl INode {
    fn as_dir(&self) -> Option<&Directory> {
        if let INode::Directory(dir) = self {
            Some(dir)
        } else {
            None
        }
    }

    fn as_file(&self) -> Option<&File> {
        if let INode::File(file) = self {
            Some(file)
        } else {
            None
        }
    }
}

impl FileSystem {
    fn new() -> Self {
        FileSystem {
            nodes: HashMap::new(),
            cwd: vec![],
        }
    }

    fn mkdir(&mut self, path: &String) {
        let path = self.canonicalize(path);
        self.nodes
            .insert(path.clone(), INode::Directory(Directory { path }));
    }

    fn create_file(&mut self, path: &String, size: usize) {
        let path = self.canonicalize(path);
        self.nodes
            .insert(path.clone(), INode::File(File { path, size }));
    }

    fn chdir(&mut self, path: &String) {
        self.mkdir(path);
        let path = self.canonicalize(path);
        self.cwd = path;
    }

    fn canonicalize(&self, path: &String) -> Path {
        let components: Path = path
            .split('/')
            .filter(|c| !c.is_empty())
            .map(String::from)
            .collect();
        if path.starts_with('/') {
            components
        } else if path == ".." {
            self.cwd.split_last().unwrap().1.to_vec()
        } else {
            [self.cwd.clone(), components].concat()
        }
    }

    fn find_all_files_in(&self, path: &Path) -> HashSet<&File> {
        self.nodes
            .values()
            .filter_map(|n| n.as_file())
            .filter(|f| f.path.starts_with(path))
            .collect()
    }

    fn get_size(&self, path: &Path) -> usize {
        let entry = self.nodes.get(path).unwrap();
        match entry {
            INode::File(file) => file.size,
            INode::Directory(dir) => self
                .find_all_files_in(&dir.path)
                .iter()
                .map(|f| self.get_size(&f.path))
                .sum(),
        }
    }
}

type Input = FileSystem;

fn input(input_str: &str) -> Input {
    let mut fs = FileSystem::new();
    for line in input_str.lines() {
        if line.starts_with("$ cd ") {
            let target = line.replace("$ cd ", "");
            fs.chdir(&target);
        } else if line.starts_with("$ ls") {
            // nothing
        } else if line.starts_with("dir") {
            let dir = line.replace("dir ", "");
            fs.mkdir(&dir);
        } else if line.chars().next().unwrap().is_ascii_digit() {
            let mut entry = line.split(' ');
            let size = entry.next().unwrap().parse::<usize>().unwrap();
            let name = entry.next().unwrap();
            fs.create_file(&String::from(name), size);
        } else {
            unreachable!();
        }
    }
    fs
}

fn part1(fs: &Input) -> usize {
    fs.nodes
        .values()
        .filter_map(|node| node.as_dir())
        .map(|dir| fs.get_size(&dir.path))
        .filter(|&size| size <= 100000)
        .sum()
}

fn part2(fs: &Input) -> usize {
    let total_size = fs.get_size(&vec![]);
    let free_space = 70000000 - total_size;
    let to_clean = 30000000 - free_space;

    fs.nodes
        .values()
        .filter_map(|node| node.as_dir())
        .map(|d| fs.get_size(&d.path))
        .filter(|&size| size >= to_clean)
        .min()
        .unwrap()
}

fn main() {
    let test_str = include_str!("../test");
    let t = input(test_str);
    assert_eq!(part1(&t), 95437);
    assert_eq!(part2(&t), 24933642);
    let input_str = include_str!("../input");
    let i = input(input_str);
    println!("{}", part1(&i));
    println!("{}", part2(&i));
}
