fn main() {
    let input = include_str!("../input");
    
    let num_columns = input.lines().nth(1).unwrap().chars().count(); // Could be optimized
    let num_lines = input.lines().count();
    let mut counter_array : Vec<i32> = vec![0; num_columns];

    for line in input.lines() {
        for (idx, char) in line.chars().enumerate() {
            counter_array[idx] +=  char as i32 - 48; // 0 in ASCII is 48
        }
    }

    let mut gamma_rate : i32 =  0;
    for (idx, cnt) in counter_array.iter().enumerate()  {
        gamma_rate += ((cnt * 2 > (num_lines as i32)) as i32)  * 2_i32.pow((num_columns - 1 - idx) as u32);
    }

    let result  = gamma_rate * (2_i32.pow(num_columns as u32) - 1 - gamma_rate);

    println!("{}", result);
}