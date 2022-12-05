//const INPUT: &str = include_str!("../test.txt");
const INPUT: &str = include_str!("../input.txt");

type Action = (i32, i32, i32);
type Stack = Vec<String>;

fn read_input() -> Vec<String> {
    INPUT.split('\n').map(str::to_string).collect()
}

fn to_actions(input: &[String]) -> Vec<Action> {
    input
        .iter()
        .filter(|x| x.starts_with("move "))
        .map(|x| {
            let action = x
                .split(' ')
                .map(|x| x.parse::<i32>().unwrap_or(0))
                .collect::<Vec<_>>();
            (action[1], action[3] - 1, action[5] - 1)
        })
        .collect()
}

fn to_stacks(input: &[String]) -> Vec<Stack> {
    let mut stacks: Vec<Stack> = Vec::<Stack>::new();
    for line in input.iter().take_while(|x| x[0..2].ne(" 1")) {
        if stacks.is_empty() {
            for _ in 0..(line.len() + 3) / 4 {
                stacks.push(Stack::new());
            }
        }
        for s in 0..stacks.len() {
            let item = &line[s * 4 + 1..s * 4 + 2];
            if !item.eq(" ") {
                stacks[s].insert(0, String::from(item))
            }
        }
    }
    stacks
}

fn moves(stacks: &mut [Stack], actions: &[Action]) {
    for action in actions.iter() {
        for _ in 0..action.0 {
            let item = stacks[action.1 as usize].pop().unwrap();
            stacks[action.2 as usize].push(item);
        }
    }
}

fn top_stacks(stacks: &[Stack]) -> String {
    let mut top = String::new();
    for stack in stacks.iter() {
        top.push_str(&stack[stack.len() - 1]);
    }
    top
}

fn step1() {
    let input = read_input();
    let actions = to_actions(&input);
    let mut stacks = to_stacks(&input);
    moves(&mut stacks, &actions);
    let top = top_stacks(&stacks);
    println!("step1: {top}");
}

fn moves_ordered(stacks: &mut [Stack], actions: &[Action]) {
    for action in actions.iter() {
        let dst_top = stacks[action.2 as usize].len();
        for _ in 0..action.0 {
            let item = stacks[action.1 as usize].pop().unwrap();
            stacks[action.2 as usize].insert(dst_top, item);
        }
    }
}

fn step2() {
    let input = read_input();
    let actions = to_actions(&input);
    let mut stacks = to_stacks(&input);
    moves_ordered(&mut stacks, &actions);
    let top = top_stacks(&stacks);
    println!("step2: {top}");
}

fn main() {
    step1();
    step2();
}
