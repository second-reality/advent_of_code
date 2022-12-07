fn duplicates<T: Eq>(vector: &Vec<T>) -> Option<usize> {
    let n = vector.len();
    (0..n).find(|i| ((i + 1)..n).any(|j| vector[*i] == vector[j]))
}

fn find_uniques_n(input: String, n: usize) -> usize {
    let (cur4chars, next_chars) = input.split_at(n);
    let mut cur4chars: Vec<char> = cur4chars.chars().collect();
    let mut next_chars = next_chars.chars();
    let mut cur = n;
    while let Some(i) = duplicates(&cur4chars) {
        for x in (0..=i).rev() {
            cur4chars.remove(x);
            cur4chars.push(next_chars.next().unwrap());
            cur += 1;
        }
    }
    cur
}
pub fn part1(input: String) -> usize {
    find_uniques_n(input, 4)
}

pub fn part2(input: String) -> usize {
    find_uniques_n(input, 14)
}

pub const EXPECTED1: usize = 11;
pub const EXPECTED2: usize = 26;
