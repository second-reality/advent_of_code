fn read_input() -> Vec<String> {
    //include_str!("../test.txt")
    include_str!("../input.txt")
        .trim()
        .split('\n')
        .map(str::to_string)
        .collect()
}

fn map_byte(byte: u8) -> i32 {
    if byte as i32 >= 'a' as i32 {
        byte as i32 - 'a' as i32 + 1
    } else {
        byte as i32 - 'A' as i32 + 27
    }
}

fn str_to_privec(string: String) -> Vec<i32> {
    let mut vec = [0; 53];
    let bstr = string.as_bytes();
    for i in 0..string.len() {
        vec[map_byte(bstr[i]) as usize] = 1;
    }
    vec.to_vec()
}

fn priority(p1: String, p2: String) -> i32 {
    let v1 = str_to_privec(p1);
    let v2 = str_to_privec(p2);

    let mut val = 0;
    for (i, (a, b)) in v1.iter().zip(v2.iter()).enumerate() {
        if a & b == 1 {
            val += i as i32;
        }
    }
    val
}

fn step1() {
    let input = read_input();
    let pairs = input.iter().map(|x| x.split_at(x.len() / 2));
    let sum = pairs
        .map(|x| priority(x.0.to_string(), x.1.to_string()))
        .sum::<i32>();
    println!("step1: {sum}");
}

fn priority3(p1: String, p2: String, p3: String) -> i32 {
    let v1 = str_to_privec(p1);
    let v2 = str_to_privec(p2);
    let v3 = str_to_privec(p3);

    let mut val = 0;
    for (i, ((a, b), c)) in v1.iter().zip(v2.iter()).zip(v3.iter()).enumerate() {
        if a & b & c == 1 {
            val += i as i32;
        }
    }
    val
}

fn step2() {
    let input = read_input();
    let triples = input
        .iter()
        .zip(input.iter().skip(1))
        .zip(input.iter().skip(2))
        .step_by(3);
    let sum = triples
        .map(|((a, b), c)| priority3(a.to_string(), b.to_string(), c.to_string()))
        .sum::<i32>();
    println!("step2: {sum}");
}

fn main() {
    step1();
    step2();
}
