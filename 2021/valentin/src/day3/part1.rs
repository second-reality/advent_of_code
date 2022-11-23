use std::fs;
use crate::helper::array_count_set_bits256;

pub fn solution() -> u32 {
    let text = fs::read_to_string("src/day3/input.txt")
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = text.trim().split('\n').collect();
    const N_COLUMNS: usize = 12;
    const N_LINES: usize = 1000;
    assert_eq!(N_LINES, lines.len());

    let mut counts: [u32; N_COLUMNS] = [0; N_COLUMNS];
    let count_set_bits256 = array_count_set_bits256();

    for i in (0..(N_LINES)).step_by(8) {
        let cur_8_lines = &lines[i..(i + 8)];
        for j in 0..N_COLUMNS {
            // parse sub String of size 8 in column j
            let bin_str = cur_8_lines.iter().map(|s| s.chars().nth(j).unwrap()).collect::<String>();
            let one_byte = u8::from_str_radix(&*bin_str, 2).unwrap();
            counts[j] += count_set_bits256[one_byte as usize];
        }
    }
    println!("{:?}", counts);

    let final_bin_str = counts.into_iter()
        .map(|x|
        if x > (N_LINES / 2) as u32 { "1" } else { "0" })
        .collect::<Vec<&str>>()
        .join("");

    let final_bin_str_not = counts.into_iter()
        .map(|x|
            if x > (N_LINES / 2) as u32 { "0" } else { "1" })
        .collect::<Vec<&str>>()
        .join("");

    println!("final str bin {}", final_bin_str);
    println!("f str bin not {}", final_bin_str_not);

    let final_int = u32::from_str_radix(&*final_bin_str, 2).unwrap();
    let final_int_not = u32::from_str_radix(&*final_bin_str_not, 2).unwrap();

    return final_int_not * final_int;
}