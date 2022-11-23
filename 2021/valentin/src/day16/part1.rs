use std::fs;
use crate::helper::get_char;

pub fn convert_to_binary_from_hex(hex: &str) -> String {
    hex.chars().map(to_binary).collect()
}

fn to_binary(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    }
}
#[derive(Debug)]
pub struct Packet {
    version:u8,
    pub type_id:u8,
    length:usize,
    pub literal:Option<usize>,
    pub sub_packets:Option<Vec<Box<Packet>>>,
}

fn handle_literal(bin_str:&str, version:u8) -> Packet {
    // search for literal value
    let mut literal_value = String::new();
    let mut index:usize = 6;
    let mut substring = &bin_str[index..(index+5)];
    while substring.starts_with('1') {
        literal_value.push_str(&substring[1..]);
        index += 5;
        substring = &bin_str[index..(index+5)];
    }
    // the last 4 bits are prefixed with a 0 don't forget them
    literal_value.push_str(&substring[1..]);

    let literal_value = usize::from_str_radix(&literal_value, 2).unwrap();
    // ignore the 0s in the end

    index += 5;

    Packet {
        version,
        type_id:4,
        length:index,
        literal: Option::Some(literal_value),
        sub_packets: Option::None
    }
}

impl Packet {
    pub fn from_binary(bin_str:&str) -> Packet {
        let version= u8::from_str_radix(&bin_str[0..3], 2).unwrap();
        let type_id = u8::from_str_radix(&bin_str[3..6], 2).unwrap();
        if type_id == 4 {
            handle_literal(bin_str, version)
        } else {
            // search for operator and sub packets
            let (sub_packets_vec, length) = if get_char(bin_str, 6) == '0' {
                let total_length = usize::from_str_radix(&bin_str[7..(7 + 15)], 2).unwrap();
                let mut res:Vec<Box<Packet>> = Vec::new();
                let mut len:usize = 0;
                while len < total_length {
                    if *(&bin_str[(len + 7 + 15)..].chars().all(|c| c == '0')) {break;}
                    let sub_packet = Packet::from_binary(&bin_str[(len + 7 + 15)..]);
                    len += sub_packet.length;
                    res.push(Box::from(sub_packet));
                }
                (res, (7 + 15 + total_length))
            } else {
                let n_sp = usize::from_str_radix(&bin_str[7..(7 + 11)], 2).unwrap();
                let mut res:Vec<Box<Packet>> = Vec::new();
                let mut len:usize = 7 + 11;
                for _ in 0..n_sp {
                    let sub_packet = Packet::from_binary(&bin_str[len..]);
                    len += sub_packet.length;
                    res.push(Box::from(sub_packet));
                }
                (res, len)
            };

            Packet {
                version,
                type_id,
                length,
                literal: Option::None,
                sub_packets: Option::Some(sub_packets_vec)
            }
        }
    }

    fn sum_all_version_numbers(&self) -> usize {
        let my_version = self.version as usize;
        if self.type_id == 4 {
            my_version
        } else {
            let sub_packets_total_version:usize = self.sub_packets.as_ref()
                .unwrap()
                .iter()
                .map(|sp| sp.sum_all_version_numbers())
                .sum();
            my_version + sub_packets_total_version
        }
    }
}

pub fn solution() -> usize {
    let text = fs::read_to_string("src/day16/input.txt")
        .expect("Something went wrong reading the file");
    let hex_input = text.trim();
    // let hex_input = "A0016C880162017C3686B18A3D4780";
    let bin_input = convert_to_binary_from_hex(hex_input);
    let bin_input = bin_input.as_str();

    let packet = Packet::from_binary(bin_input);

    packet.sum_all_version_numbers()
}