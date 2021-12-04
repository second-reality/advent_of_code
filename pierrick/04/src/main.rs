struct BingoBoard {
    rows_not_marked: Vec<Vec<i32>>,
    cols_not_marked: Vec<Vec<i32>>,
}

impl BingoBoard {
    fn new(numbers: &[Vec<i32>]) -> BingoBoard {
        let board_size = numbers[0].len();

        let mut rows: Vec<Vec<i32>> = vec![];
        let mut cols: Vec<Vec<i32>> = vec![];
        for row in numbers {
            assert!(row.len() == board_size);
            rows.push(row.clone());
        }
        for j in 0..board_size {
            let mut col = vec![];
            #[allow(clippy::needless_range_loop)]
            for i in 0..board_size {
                col.push(numbers[i][j]);
            }
            assert!(col.len() == board_size);
            cols.push(col);
        }

        BingoBoard {
            rows_not_marked: rows,
            cols_not_marked: cols,
        }
    }

    fn play(&mut self, number: i32) {
        let filter_one = |vec: &mut Vec<i32>| vec.retain(|&x| x != number);
        self.rows_not_marked.iter_mut().for_each(filter_one);
        self.cols_not_marked.iter_mut().for_each(filter_one);
    }

    fn won(&self) -> bool {
        let is_empty = |vec: &Vec<Vec<i32>>| vec.iter().any(|x| x.is_empty());
        is_empty(&self.rows_not_marked) || is_empty(&self.cols_not_marked)
    }

    fn sum_of_not_marked(&self) -> i32 {
        let mut sum = 0;
        for row in &self.rows_not_marked {
            for col in row {
                sum += col;
            }
        }
        sum
    }
}

fn get_input() -> (Vec<i32>, Vec<BingoBoard>) {
    let input = include_str!("../input.txt");
    let numbers = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    let mut bingo: Vec<Vec<Vec<i32>>> = vec![];
    for l in input.lines().skip(1) {
        if l.is_empty() {
            bingo.push(vec![]);
        } else {
            let line_numbers: Vec<i32> = l.split_whitespace().map(|x| x.parse().unwrap()).collect();
            bingo.last_mut().unwrap().push(line_numbers);
        }
    }

    let boards = bingo.iter().map(|x| BingoBoard::new(x)).collect();
    (numbers, boards)
}

fn play_boards(numbers: &[i32], mut boards: Vec<BingoBoard>) -> Vec<i32>
{
    let mut res = vec![];
    for &n in numbers.iter() {
        for b in boards.iter_mut() {
            b.play(n);
            if b.won() {
                res.push(n * b.sum_of_not_marked());
            }
        }
        boards.retain(|b| !b.won());
    }

    res
}

fn main() {
    let (numbers, boards) = get_input();
    let results = play_boards(&numbers, boards);
    println!("{}", results[0]);
    println!("{}", results.last().unwrap());
}
