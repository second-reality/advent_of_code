use std::ops::Range;

pub fn sort_srt(string: &str) -> String {
    let mut chars_test:Vec<char> = string.chars().collect();
    chars_test.sort();
    return chars_test.into_iter().collect();

}

pub fn sub_sum(i:usize, j:usize, tab:&Vec<i32>) -> i32 {
    return (&tab[i..j]).iter().sum::<i32>();
}

pub fn array_count_set_bits256() -> Vec<u32> {
    let mut array:Vec<u32> = vec![0; 256];
    for i in 0..256 {
        array[i] = (i & 1) as u32 + array[i / 2];
    }
    return array;
}

pub fn parse_matrix_of_digits(lines:Vec<&str>) -> Vec<u32> {
    lines.into_iter().
        flat_map(|x| x.chars().map(|c| c.to_digit(10).unwrap()))
        .collect()
}

pub fn neighbors_matrix2d_no_diag(index:usize, n_columns:usize, n_lines:usize) -> Vec<usize> {
    let n_columns_i=n_columns as i32;
    let n_lines_i = n_lines as i32;
    let index_i = index as i32;
    let (i, j) = (index_i / n_columns_i, index_i % n_columns_i);
    let mut res:Vec<usize> = Vec::new();
    for d in (-1..2).step_by(2) {
        // check neighbors up and down
        if (0..n_lines_i).contains(&(i + d)) {
            res.push((index_i + d* n_columns_i) as usize);
        }
        // check neighbors left and right
        if (0..n_columns_i).contains(&(j + d)) {
            res.push((index_i + d) as usize);
        }
    }
    res
}

pub fn path_from_prev_list(start:usize, end:usize, prev:Vec<usize>) -> Vec<usize> {
    let mut path:Vec<usize> = Vec::new();
    let mut cur = end;
    while cur != start {
        path.push(cur);
        cur = prev[cur];
    }
    path.push(start);
    path.reverse();
    path
}

pub fn intersect_1d(range1:Range<i32>, range2:Range<i32>) -> bool {
    let len1 = (range1.end - range1.start).abs();
    let len2 = (range2.end - range2.start).abs();
    let (bigger_range, other_range) = if len1 > len2 {
        (range1, range2)
    } else {
        (range2, range1)
    };
    return bigger_range.contains(&other_range.start) || bigger_range.contains(&other_range.end);
}

pub fn get_char(string:&str, index:usize) -> char {
    string[index..(index +1)].chars().last().unwrap()
}