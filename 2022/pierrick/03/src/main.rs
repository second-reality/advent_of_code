use std::cmp;
use std::collections::HashMap;

type Item = char;

struct Compartment {
    items: HashMap<Item, usize>,
}

struct Rucksack {
    c1: Compartment,
    c2: Compartment,
    all_items: Vec<Item>,
}

type Input = Vec<Rucksack>;

impl Compartment {
    fn new(content: &str) -> Compartment {
        let mut items = HashMap::new();
        for item in content.chars() {
            let count = items.entry(item).or_insert(0);
            *count += 1;
        }
        Compartment { items }
    }

    fn find_common_item(&self, other: &Compartment) -> Vec<Item> {
        let mut res = Vec::new();
        for item in self.items.keys() {
            if other.items.contains_key(item) {
                let num_our = self.items.get(item).unwrap();
                let num_their = other.items.get(item).unwrap();
                let num_common = *cmp::min(num_our, num_their);
                for _ in 0..=num_common {
                    res.push(*item);
                }
            }
        }
        res
    }

    fn has_item(&self, item: Item) -> bool {
        self.items.contains_key(&item)
    }
}

fn priority(item: Item) -> usize {
    if item.is_uppercase() {
        item as usize - 'A' as usize + 27
    } else {
        item as usize - 'a' as usize + 1
    }
}

impl Rucksack {
    fn new(content: &str) -> Rucksack {
        let all_items = content.chars().collect();
        let (content1, content2) = content.split_at(content.len() / 2);
        Rucksack {
            c1: Compartment::new(content1),
            c2: Compartment::new(content2),
            all_items,
        }
    }

    fn has_item(&self, item: Item) -> bool {
        self.c1.has_item(item) || self.c2.has_item(item)
    }
}

fn input(input_str: &str) -> Input {
    input_str.lines().map(Rucksack::new).collect()
}

fn part1(i: &Input) -> usize {
    let mut res = 0;
    for r in i {
        let mut common = r.c1.find_common_item(&r.c2);
        // do not count when the same items is shared several times
        common.dedup();
        for item in common.iter() {
            res += priority(*item);
        }
    }
    res
}

fn part2(i: &Input) -> usize {
    let mut res = 0;
    for rucksacks in i.chunks(3) {
        let first = rucksacks.get(0).unwrap();
        let second = rucksacks.get(1).unwrap();
        let third = rucksacks.get(2).unwrap();
        for item in first.all_items.iter() {
            if second.has_item(*item) && third.has_item(*item) {
                res += priority(*item);
                break;
            }
        }
    }
    res
}

fn main() {
    let input_str = include_str!("../input");
    let test_str = include_str!("../test");
    let i = input(input_str);
    assert_eq!(part1(&input(test_str)), 157);
    println!("{}", part1(&i));
    println!("{}", part2(&i));
}
