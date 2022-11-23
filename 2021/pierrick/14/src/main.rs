use std::collections::HashMap;

struct Template {
    // AB -> C
    // base: "AB", generate: "AC", "BC"
    base: String,
    generate: [String; 2],
}

impl Template {
    fn new(s: &str) -> Self {
        let (base, added) = s.split_once(" -> ").unwrap();
        let base = base.to_string();
        let added = added.chars().next().unwrap();
        let left = base.chars().next().unwrap();
        let right = base.chars().nth(1).unwrap();
        let generate = [
            [left, added].iter().collect(),
            [added, right].iter().collect(),
        ];
        Self { base, generate }
    }
}

#[derive(Clone, Default, Debug)]
struct Polymer {
    chains: HashMap<String, i64>,
}

impl Polymer {
    fn new(s: &str) -> Self {
        let mut res = Polymer::default();
        let chars: Vec<char> = s.chars().collect();
        let _ = &chars[..].windows(2).for_each(|slice| {
            let s: String = [slice[0], slice[1]].iter().collect();
            res.add_chain(&s, 1)
        });
        let first: String = [chars.first().unwrap()].into_iter().collect();
        let last: String = [chars.last().unwrap()].into_iter().collect();
        res.add_chain(&first, 1);
        res.add_chain(&last, 1);
        res
    }

    fn step(&self, templates: &[Template]) -> Self {
        let mut res = Polymer::default();

        for (chain, &count) in self.chains.iter() {
            match templates.iter().find(|&c| c.base == *chain) {
                Some(t) => t.generate.iter().for_each(|g| res.add_chain(g, count)),
                None => res.add_chain(chain, count),
            }
        }
        res
    }

    fn add_chain(&mut self, chain: &str, quantity: i64) {
        *self.chains.entry(chain.to_string()).or_default() += quantity;
    }

    fn elements(&self) -> HashMap<char, i64> {
        let mut res = HashMap::new();
        for (chain, count) in self.chains.iter() {
            for c in chain.chars() {
                *res.entry(c).or_default() += count;
            }
        }

        res.iter_mut().for_each(|(_, count)| *count /= 2);
        res
    }
}

fn solution(s: &str, steps: usize) -> i64 {
    let (mut polymer, templates) = get_input(s);

    (0..steps).for_each(|_| polymer = polymer.step(&templates));
    let elements = polymer.elements();
    elements.values().max().unwrap() - elements.values().min().unwrap()
}

fn get_input(s: &str) -> (Polymer, Vec<Template>) {
    let polymer = Polymer::new(s.lines().next().unwrap());
    let templates = s.lines().skip(2).map(|s| Template::new(s)).collect();
    (polymer, templates)
}

const TEST_DATA: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

fn main() {
    assert_eq!(1588, solution(TEST_DATA, 10));
    println!("{}", solution(include_str!("../input"), 10));
    println!("{}", solution(include_str!("../input"), 40));
}
