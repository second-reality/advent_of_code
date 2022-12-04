type Range = (i32, i32);
type RangePair = (Range, Range);

fn read_input() -> Vec<RangePair> {
    //include_str!("../test.txt")
    include_str!("../input.txt")
        .trim()
        .split('\n')
        .map(|x| {
            let elves = x
                .split(',')
                .map(|x| {
                    let ints = x
                        .split('-')
                        .map(|x| x.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>();
                    (ints[0], ints[1])
                })
                .collect::<Vec<_>>();
            (elves[0], elves[1])
        })
        .collect()
}

fn first_contains(r1: Range, r2: Range) -> bool {
    r2.0 >= r1.0 && r2.1 <= r1.1
}

fn contains(r1: Range, r2: Range) -> bool {
    first_contains(r1, r2) || first_contains(r2, r1)
}

fn step1() {
    let input = read_input();
    let sum = input.iter().map(|x| contains(x.0, x.1) as i32).sum::<i32>();
    println!("step1: {sum}");
}

fn overlap(r1: Range, r2: Range) -> bool {
    r1.1 >= r2.0 && r1.0 <= r2.1
}

fn step2() {
    let input = read_input();
    let sum = input.iter().map(|x| overlap(x.0, x.1) as i32).sum::<i32>();
    println!("step2: {sum}");
}

fn main() {
    step1();
    step2();
}
