use std::fs;


fn roll3dice(turn:usize) -> usize {
    3 * (3 * turn  - 1)
}

fn move_player(pos:usize, mv:usize) -> usize {
    ((pos + mv - 1) % 10) + 1
}

pub fn solution() -> usize {
    let text = fs::read_to_string("src/day21/input.txt")
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = text.trim().split('\n').collect();
    let player1_init_pos = lines[0].chars().last().unwrap().to_digit(10).unwrap() as usize;
    let player2_init_pos = lines[1].chars().last().unwrap().to_digit(10).unwrap() as usize;

    let mut player1_score:usize = 0;
    let mut player2_score:usize = 0;
    let mut player1_pos = player1_init_pos;
    let mut player2_pos = player2_init_pos;

    let mut turn:usize = 1;
    let mut count_rolls:usize = 0;

    loop {
        player1_pos = move_player(player1_pos, roll3dice(turn));
        player1_score += player1_pos;
        turn += 1;
        count_rolls += 3;

        if player1_score >= 1000 {
            return player2_score * count_rolls;
        }

        player2_pos = move_player(player2_pos, roll3dice(turn));

        player2_score += player2_pos;
        turn += 1;
        count_rolls += 3;

        if player2_score >= 1000 {
            return player1_score * count_rolls;
        }
    }
}