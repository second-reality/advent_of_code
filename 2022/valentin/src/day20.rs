fn parse(input: String) -> Vec<(usize, isize)> {
    input
        .trim()
        .split('\n')
        .enumerate()
        .map(|(i, line)| (i, line.parse().unwrap()))
        .collect()
}

fn compute_result(sequence: Vec<(usize, isize)>) -> usize {
    let root = sequence.iter().position(|x| x.1 == 0).unwrap();
    let g1 = (root + 1000) % sequence.len();
    let g2 = (root + 2000) % sequence.len();
    let g3 = (root + 3000) % sequence.len();
    (sequence[g1].1 + sequence[g2].1 + sequence[g3].1) as usize
}

fn one_mixing_tour(sequence: &mut Vec<(usize, isize)>) {
    for original_index in 0..sequence.len() {
        let index = sequence.iter().position(|x| x.0 == original_index).unwrap();
        let value = sequence[index].1;

        let new_index = index as isize + value;
        let new_index = new_index.rem_euclid(sequence.len() as isize - 1);
        let to_add = sequence.remove(index);
        sequence.insert(new_index as usize, to_add);
    }
}

pub fn part1(input: String) -> usize {
    let mut sequence = parse(input);
    one_mixing_tour(&mut sequence);
    compute_result(sequence)
}

pub fn part2(input: String) -> usize {
    let key = 811589153;
    let mut sequence = parse(input)
        .into_iter()
        .map(|(index, value)| (index, value * key))
        .collect();
    for _ in 0..10 {
        one_mixing_tour(&mut sequence);
    }
    compute_result(sequence)
}

pub const EXPECTED1: usize = 3;
pub const EXPECTED2: usize = 1623178306;
