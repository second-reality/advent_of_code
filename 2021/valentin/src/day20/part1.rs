use std::fs;

pub fn parse_input(lines:Vec<&str>, n_step:usize, n:usize) -> (Vec<bool>, Vec<Vec<bool>>) {
    let magic_line:Vec<bool> = lines[0].chars().map(|c| c == '#').collect();
    let mut matrix:Vec<Vec<bool>> = (0..(n + 2 * (n_step + 1))).into_iter()
        .map(|i| {
            (0..(n + 2 * (n_step + 1))).into_iter()
                .map(|j| false).collect()
        }).collect();

    let mut index_row = n_step + 1;
    for i in 2..lines.len() {
        let row:Vec<bool> = lines[i].chars().map(|c| c == '#').collect();
        for j in (n_step+ 1)..(n_step + n + 1) {
            matrix[index_row][j] = row[j-n_step - 1];
        }
        index_row += 1;
    }
    (magic_line, matrix)
}

fn compute_output_pixel(i:i32, j:i32, input_image:&Vec<Vec<bool>>, magic_line:&Vec<bool>) -> bool {
    let mut binary_index = String::new();
    for di in -1..2 {
        for dj in -1..2 {
            let new_i = (i + di) as usize;
            let new_j = (j + dj) as usize;
            let bit = if (0..input_image.len()).contains(&new_i) && (0..input_image[0].len()).contains(&new_j) {
                input_image[new_i][new_j]
            } else {
                input_image[0][0]
            };
            binary_index.push(char::from_digit(bit as u32, 2).unwrap());
        }
    }
    let binary_index = usize::from_str_radix(&*binary_index, 2).unwrap();
    magic_line[binary_index]
}

pub fn process_image(input_image:&Vec<Vec<bool>>, magic_line:&Vec<bool>) -> Vec<Vec<bool>> {
    let mut output_image:Vec<Vec<bool>> = input_image.clone();
    for i in 0..output_image.len() {
        for j in 0..output_image[0].len() {
            output_image[i][j] = compute_output_pixel(i as i32, j as i32, input_image, magic_line);
        }
    }
    output_image
}

fn print_image(image:&Vec<Vec<bool>>) {
    println!("------ IMAGE {}x{} -------", image.len(), image[0].len());
    for i in 0..image.len() {
        for j in 0..image[0].len() {
            let c = if image[i][j] { '#' } else {'.'};
            print!("{}", c);
        }
        println!();
    }
}

pub fn count_pixels_on(image:&Vec<Vec<bool>>) -> usize {
    let mut count = 0;
    for i in 1..(image.len()-1) {
        for j in 1..(image[0].len() - 1) {
            count += image[i][j] as usize;
        }
    }
    count
}

pub fn solution() -> usize {
    let text = fs::read_to_string("src/day20/input.txt")
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = text.trim().split('\n').collect();
    let n_steps = 2;
    let (magic_line, mut image) = parse_input(lines, n_steps, 100);
    print_image(&image);
    for i in 0..n_steps {
        image = process_image(&image, &magic_line);
        print_image(&image);
    }
    count_pixels_on(&image)
}