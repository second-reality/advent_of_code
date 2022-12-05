type Item = char;

struct Move {
    how_many: usize,
    from: usize,
    to: usize,
}

#[derive(Clone)]
struct Stack {
    content: Vec<Item>,
}

#[derive(Clone)]
struct Cargo {
    stacks: Vec<Stack>,
}

struct Input {
    cargo: Cargo,
    moves: Vec<Move>,
}

impl Move {
    fn new(s: &str) -> Self {
        let mut s = String::from(s);
        // 'move 7 from 3 to 9' -> '7 3 9'
        s = s.replace("move ", "");
        s = s.replace(" from ", " ");
        s = s.replace(" to ", " ");
        let mut s = s.split(' ');
        let how_many = s.next().unwrap().parse::<usize>().unwrap();
        let from = s.next().unwrap().parse::<usize>().unwrap();
        let to = s.next().unwrap().parse::<usize>().unwrap();
        Move { how_many, from, to }
    }
}

impl Cargo {
    fn new(stacks: &str) -> Self {
        let stacks = stacks.lines().map(Stack::new).collect();
        Cargo { stacks }
    }

    fn get_mut(&mut self, index: usize) -> &mut Stack {
        // indexes from stacks start at 1, but 0 here
        self.stacks.get_mut(index - 1).unwrap()
    }

    fn stacks_tops(&self) -> String {
        self.stacks.iter().map(|s| s.top()).collect()
    }
}

impl Stack {
    fn new(s: &str) -> Self {
        let mut res = Stack {
            content: Vec::new(),
        };
        for item in s.chars() {
            res.push(item);
        }
        res
    }

    fn push(&mut self, item: Item) {
        self.content.push(item);
    }

    fn pop(&mut self) -> Item {
        self.content.pop().unwrap()
    }

    fn top(&self) -> Item {
        *self.content.last().unwrap()
    }
}

fn input(input_str: &str) -> Input {
    // Input was changed to avoid parsing complicated cargo part.
    // Instead of vertical stacks, content is just on a single line.
    let mut it_input = input_str.split("\n\n");
    let cargo = Cargo::new(it_input.next().unwrap());
    let moves = it_input.next().unwrap().lines().map(Move::new).collect();
    Input { cargo, moves }
}

fn part1(i: &Input) -> String {
    let mut cargo = i.cargo.clone();
    for m in i.moves.iter() {
        for _ in 0..m.how_many {
            let item = cargo.get_mut(m.from).pop();
            cargo.get_mut(m.to).push(item);
        }
    }
    cargo.stacks_tops()
}

fn part2(i: &Input) -> String {
    let mut cargo = i.cargo.clone();
    for m in i.moves.iter() {
        let mut items = Vec::new();
        for _ in 0..m.how_many {
            items.push(cargo.get_mut(m.from).pop());
        }
        for item in items.iter().rev() {
            cargo.get_mut(m.to).push(*item);
        }
    }
    cargo.stacks_tops()
}

fn main() {
    let input_str = include_str!("../input");
    let i = input(input_str);
    println!("{}", part1(&i));
    println!("{}", part2(&i));
}
