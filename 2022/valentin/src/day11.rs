enum Operation {
    Square,
    Multiply(usize),
    Add(usize),
}
impl Operation {
    fn new(line: &str) -> Self {
        use Operation::*;
        let mut tmp_op = line.split(' ').skip(3);
        let op = tmp_op.next().unwrap();
        let value = tmp_op.next().unwrap();
        let num = value.parse::<usize>();
        if op == "+" {
            Add(num.unwrap())
        } else {
            match num {
                Ok(n) => Multiply(n),
                _ => Square,
            }
        }
    }
}
struct Monkey {
    items: Vec<usize>,
    op: Operation,
    test_div: usize,
    id_true: usize,
    id_false: usize,
    count_inspect: usize,
}

impl Monkey {
    fn new(s: &str) -> Option<Self> {
        // 6 lines
        let mut lines = s.split('\n').skip(1).map(|c| c.split(": ").last().unwrap());
        let items: Vec<usize> = lines
            .next()?
            .split(", ")
            .map(|i| i.parse().unwrap())
            .collect();
        let op = Operation::new(lines.next()?);
        let test_div: usize = lines.next()?.split(' ').last()?.parse().unwrap();
        let id_true: usize = lines.next()?.split(' ').last()?.parse().unwrap();
        let id_false: usize = lines.next()?.split(' ').last()?.parse().unwrap();
        Some(Monkey {
            items,
            op,
            test_div,
            id_true,
            id_false,
            count_inspect: 0,
        })
    }
    fn test(&self, value: usize) -> usize {
        if value % self.test_div == 0 {
            self.id_true
        } else {
            self.id_false
        }
    }
    fn apply_operation(&self, old: usize) -> usize {
        use Operation::*;
        match self.op {
            Square => old * old,
            Multiply(n) => old * n,
            Add(n) => old + n,
        }
    }
    fn inspect(&mut self) -> Option<usize> {
        let res = self.items.pop();
        if res.is_some() {
            self.count_inspect += 1;
        }
        res
    }

    fn catch(&mut self, item: usize) {
        self.items.push(item)
    }

    fn transactions(&mut self, modulo: Option<usize>) -> Vec<(usize, usize)> {
        let mut res: Vec<(usize, usize)> = Vec::new();
        while let Some(mut item) = self.inspect() {
            item = self.apply_operation(item);
            item = match modulo {
                None => item / 3,
                Some(n) => item % n,
            };
            let to = self.test(item);
            res.push((to, item))
        }
        res
    }
}

fn bullygame(rounds: usize, is_part_one: bool, input: String) -> usize {
    let mut monkeys: Vec<Monkey> = input
        .trim()
        .split("\n\n")
        .map(|s| Monkey::new(s).unwrap())
        .collect();
    let n = monkeys.len();
    let modulo = if is_part_one {
        None
    } else {
        Some(monkeys.iter().map(|m| m.test_div).product())
    };
    for _r in 0..rounds {
        // play 1 round of game
        for i in 0..n {
            let yeets = monkeys[i].transactions(modulo);
            yeets
                .into_iter()
                .for_each(|(to, item)| monkeys[to].catch(item));
        }
    }
    let mut scores: Vec<usize> = monkeys.iter().map(|m| m.count_inspect).collect();
    scores.sort_by(|a, b| b.cmp(a));
    scores.iter().take(2).product()
}
pub fn part1(input: String) -> usize {
    bullygame(20, true, input)
}

pub fn part2(input: String) -> usize {
    bullygame(10_000, false, input)
}

pub const EXPECTED1: usize = 10_605;
pub const EXPECTED2: usize = 2_713_310_158;
