use std::cmp::max;
use std::collections::HashMap;
use std::fs;

fn move_player(pos:u8, mv:u8) -> u8 {
    ((pos + mv - 1) % 10) + 1
}

fn recursive_game(score1:u8, pos1:u8, score2:u8, pos2:u8, p1_turn:bool, cache:&mut HashMap<(u8, u8, u8, u8, bool), (usize, usize)>) -> (usize, usize) {
    if cache.contains_key(&(score1, pos1, score2, pos2, p1_turn)) {
        *cache.get(&(score1, pos1, score2, pos2, p1_turn)).unwrap()
    } else {
        let new_entry = if score1 >= 21 {
            (1, 0)
        } else if score2 >= 21 {
            (0, 1)
        } else {
            // player rolls the dice 3 times and create an universe for each roll results
            let mut count_wins1:usize = 0;
            let mut count_wins2:usize = 0;
            for i in 1..4 {
                for j in 1..4 {
                    for k in 1..4 {
                        let tmp = if p1_turn {
                            let new_pos1 = move_player(pos1, i + j + k);
                            recursive_game(score1 + new_pos1, new_pos1, score2, pos2, false, cache)
                        } else {
                            let new_pos2 = move_player(pos2, i + j + k);
                            recursive_game(score1, pos1, score2 + new_pos2, new_pos2, true, cache)
                        };
                        count_wins1 += tmp.0;
                        count_wins2 += tmp.1;
                    }
                }
            }
            (count_wins1, count_wins2)
        };
        cache.insert((score1, pos1, score2, pos2, p1_turn), new_entry);
        new_entry
    }
}

fn cout_wins(player1_init_pos:u8, player2_init_pos:u8) -> (usize, usize) {
    let mut cache:HashMap<(u8, u8, u8, u8, bool), (usize, usize)> = HashMap::new();
    recursive_game(0, player1_init_pos, 0, player2_init_pos, true, &mut cache)
}

pub fn solution() -> usize {
    let text = fs::read_to_string("src/day21/input.txt")
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = text.trim().split('\n').collect();
    let player1_init_pos = lines[0].chars().last().unwrap().to_digit(10).unwrap() as u8;
    let player2_init_pos = lines[1].chars().last().unwrap().to_digit(10).unwrap() as u8;
    let (count_wins1, count_wins2) = cout_wins(player1_init_pos, player2_init_pos);
    println!("player 1 wins in {} universes", count_wins1);
    println!("player 2 wins in {} universes", count_wins2);
    max(count_wins1, count_wins2)
}