use std::any::Any;
use std::fs;
use crate::day16::part1::{convert_to_binary_from_hex, Packet};

impl Packet {
    fn evaluate_as_expr(&self) -> usize {
        if self.type_id == 4 {
            self.literal.unwrap()
        } else {
            let expressions: Vec<usize> = self.sub_packets.as_ref()
                .unwrap()
                .iter()
                .map(|sub_packet| sub_packet.evaluate_as_expr())
                .collect();
            match self.type_id {
                0 => expressions.iter().sum(),
                1 => expressions.iter().product(),
                2 => *expressions.iter().min().unwrap(),
                3 => *expressions.iter().max().unwrap(),
                5 => {
                    assert_eq!(expressions.len(), 2);
                    if expressions[0] > expressions[1] { 1 } else { 0 }
                }
                6 => {
                    assert_eq!(expressions.len(), 2);
                    if expressions[0] < expressions[1] { 1 } else { 0 }
                }
                7 => {
                    assert_eq!(expressions.len(), 2);
                    if expressions[0] == expressions[1] { 1 } else { 0 }
                }
                _ => {
                    println!("WTF it's not supposed to happen");
                    usize::MAX
                }
            }
        }
    }
}

pub fn solution() -> usize {
    let text = fs::read_to_string("src/day16/input.txt")
        .expect("Something went wrong reading the file");
    let hex_input = text.trim();
    // let hex_input = "9C0141080250320F1802104A08";
    let bin_input = convert_to_binary_from_hex(hex_input);
    let bin_input = bin_input.as_str();

    let packet = Packet::from_binary(bin_input);
    packet.evaluate_as_expr()
}