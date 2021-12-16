// each u8 is one bit
fn get_input(s: &str) -> Vec<u8> {
    fn one_char(c: char) -> u8 {
        let mut buf = [0; 1];
        let s = c.encode_utf8(&mut buf);
        u8::from_str_radix(s, 16).unwrap()
    }
    s.chars()
        .map(one_char)
        .map(|val| {
            [
                (val & 0b1000) >> 3,
                (val & 0b100) >> 2,
                (val & 0b10) >> 1,
                val & 0b1,
            ]
        })
        .flatten()
        .collect()
}

#[test]
fn test_parse() {
    assert_eq!(get_input("0"), vec![0, 0, 0, 0]);
    assert_eq!(get_input("1"), vec![0, 0, 0, 1]);
    assert_eq!(get_input("A"), vec![1, 0, 1, 0]);
    assert_eq!(get_input("F"), vec![1, 1, 1, 1]);
    assert_eq!(get_input("19"), vec![0, 0, 0, 1, 1, 0, 0, 1]);
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum PacketType {
    LitteralValue,
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    Equal,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum LengthType {
    TotalLength(u16),
    NumberSubPackets(u16),
}

#[derive(Debug, PartialEq, Eq)]
enum Trace {
    PacketVersion(u8),
    PacketType(PacketType),
    LengthType(LengthType),
    LitteralValue(u64),
}

struct Message {
    data: Vec<u8>,
    trace: Vec<Trace>,
}

impl Message {
    fn new(d: &[u8]) -> Self {
        let mut data = d.to_owned();
        data.reverse();
        Message {
            data,
            trace: vec![],
        }
    }

    fn read(&mut self, num_bits: u8) -> u64 {
        let mut res = 0;
        for _ in 0..num_bits {
            res <<= 1;
            res += self.data.pop().unwrap() as u64;
        }
        res
    }

    fn packet_version(&mut self) -> u8 {
        self.read(3) as u8
    }

    fn packet_type(&mut self) -> PacketType {
        match self.read(3) {
            0 => PacketType::Sum,
            1 => PacketType::Product,
            2 => PacketType::Minimum,
            3 => PacketType::Maximum,
            4 => PacketType::LitteralValue,
            5 => PacketType::GreaterThan,
            6 => PacketType::LessThan,
            7 => PacketType::Equal,
            _ => unreachable!()
        }
    }

    fn litteral_value(&mut self) -> u64 {
        let mut res = 0;

        let mut stop = false;

        while !stop {
            stop = self.read(1) == 0;
            let next_bits = self.read(4);
            res <<= 4;
            res += next_bits;
        }
        res
    }

    fn length_type(&mut self) -> LengthType {
        let length_id = self.read(1);
        match length_id {
            0 => LengthType::TotalLength(self.read(15) as u16),
            1 => LengthType::NumberSubPackets(self.read(11) as u16),
            _ => unreachable!(),
        }
    }

    fn end(&self) -> bool {
        self.data.is_empty() || self.data.iter().all(|b| *b == 0)
    }

    fn offset(&self) -> usize {
        self.data.len()
    }

    fn packet(&mut self) -> i64 {
        let packet_version = self.packet_version();
        self.trace.push(Trace::PacketVersion(packet_version));

        let packet_type = self.packet_type();
        self.trace.push(Trace::PacketType(packet_type));

        match packet_type {
            PacketType::LitteralValue => {
                let val = self.litteral_value();
                self.trace.push(Trace::LitteralValue(val));
                val as i64
            }
            _ => {
                let length_type = self.length_type();
                self.trace.push(Trace::LengthType(length_type));
                match length_type {
                    LengthType::TotalLength(len) => {
                        let until = self.offset() - len as usize;
                        while self.offset() > until {
                            self.packet();
                        }
                        assert!(self.offset() == until);
                    }
                    LengthType::NumberSubPackets(num_packets) => {
                        (0..num_packets).for_each(|_| { self.packet(); } )
                    }
                }
                0
            }
        }
    }

    fn parse(&mut self) -> i64 {
        let res = self.packet();
        assert!(self.end());
        res
    }
}

fn parse(data: &str) -> Vec<Trace> {
    let data = get_input(data.trim());
    let mut msg = Message::new(&data);
    msg.parse();
    msg.trace
}

const TEST_LITTERAL: &str = "D2FE28";
const TEST_OPERATOR: &str = "38006F45291200";
const INPUT: &str = include_str!("../input");

fn part1(s: &str) -> usize {
    let trace = parse(s);
    trace
        .iter()
        .map(|e| match e {
            Trace::PacketVersion(version) => *version as usize,
            _ => 0,
        })
        .sum()
}

fn main() {
    assert_eq!(16, part1("8A004A801A8002F478"));
    assert_eq!(12, part1("620080001611562C8802118E34"));
    assert_eq!(23, part1("C0015000016115A2E0802F182340"));
    assert_eq!(31, part1("A0016C880162017C3686B18A3D4780"));
    println!("{}", part1(INPUT));
}
