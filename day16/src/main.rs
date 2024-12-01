use core::num;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Bit {
    Zero,
    One,
}

pub fn to_bits(c: char) -> [Bit; 4] {
    match c {
        '0' => [Bit::Zero, Bit::Zero, Bit::Zero, Bit::Zero],
        '1' => [Bit::Zero, Bit::Zero, Bit::Zero, Bit::One],
        '2' => [Bit::Zero, Bit::Zero, Bit::One, Bit::Zero],
        '3' => [Bit::Zero, Bit::Zero, Bit::One, Bit::One],

        '4' => [Bit::Zero, Bit::One, Bit::Zero, Bit::Zero],
        '5' => [Bit::Zero, Bit::One, Bit::Zero, Bit::One],
        '6' => [Bit::Zero, Bit::One, Bit::One, Bit::Zero],
        '7' => [Bit::Zero, Bit::One, Bit::One, Bit::One],

        '8' => [Bit::One, Bit::Zero, Bit::Zero, Bit::Zero],
        '9' => [Bit::One, Bit::Zero, Bit::Zero, Bit::One],
        'A' => [Bit::One, Bit::Zero, Bit::One, Bit::Zero],
        'B' => [Bit::One, Bit::Zero, Bit::One, Bit::One],

        'C' => [Bit::One, Bit::One, Bit::Zero, Bit::Zero],
        'D' => [Bit::One, Bit::One, Bit::Zero, Bit::One],
        'E' => [Bit::One, Bit::One, Bit::One, Bit::Zero],
        'F' => [Bit::One, Bit::One, Bit::One, Bit::One],

        _ => unreachable!(),
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Payload {
    Literal(u64),
    Operation {
        type_id: u64,
        packets: Vec<Packet>
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Packet {
    version: u64,
    payload: Payload,
}

impl Packet {
    pub fn total_version(&self) -> u64 {
        self.version + match &self.payload {
            Payload::Literal(_) => 0,
            Payload::Operation { type_id: _, packets } => {
                packets.iter().map(|p| p.total_version()).sum()
            }
        }
    }

    pub fn evaluate(&self) -> u64 {
        match &self.payload {
            Payload::Literal(val) => *val,
            Payload::Operation { type_id, packets } => {
                match type_id {
                    0 => packets.iter().map(|p| p.evaluate()).sum(),
                    1 => packets.iter().map(|p| p.evaluate()).product(),
                    2 => packets.iter().map(|p| p.evaluate()).min().unwrap(),
                    3 => packets.iter().map(|p| p.evaluate()).max().unwrap(),
                    5 => if packets[0].evaluate() > packets[1].evaluate() { 1 } else { 0 },
                    6 => if packets[0].evaluate() < packets[1].evaluate() { 1 } else { 0 },
                    7 => if packets[0].evaluate() == packets[1].evaluate() { 1 } else { 0 },
                    _ => unreachable!(),
                }
            }
        }
    }
}

pub fn to_number(bits: &[Bit]) -> u64 {
    let mut result = 0;
    for (power, bit) in bits.iter().rev().enumerate() {
        if bit == &Bit::One {
            result += 2u64.pow(power as u32);
        }
    }

    result
}

pub fn parse_input(input: &str) -> Packet {
    parse_packet(&mut input.trim().chars().flat_map(to_bits).collect())
}

pub fn parse_packet(input: &mut Vec<Bit>) -> Packet {
    let version_chunk = input.drain(0..=2).collect::<Vec<_>>();
    let type_id_chunk = input.drain(0..=2).collect::<Vec<_>>();

    let version = to_number(&version_chunk);
    let type_id = to_number(&type_id_chunk);

    if type_id == 4 {
        let number = parse_literal(input);
        Packet { version, payload: Payload::Literal(number) }
    } else {
        let length_type_id = *input.drain(0..1).collect::<Vec<_>>().first().unwrap();
        match length_type_id {
            Bit::Zero => {
                let length_of_sub_packets = input.drain(0..=14).collect::<Vec<_>>();
                let length_of_sub_packets = to_number(&length_of_sub_packets) as usize;

                let mut sub_packets_to_parse = input.drain(0..length_of_sub_packets).collect::<Vec<_>>();
                let mut parsed_sub_packets = Vec::new();
                while !sub_packets_to_parse.is_empty() {
                    parsed_sub_packets.push(parse_packet(&mut sub_packets_to_parse));
                }

                Packet { version, payload: Payload::Operation {
                    type_id, packets: parsed_sub_packets 
                }}
            },

            Bit::One => {
                let number_of_sub_packets = input.drain(0..=10).collect::<Vec<_>>();
                let number_of_sub_packets = to_number(&number_of_sub_packets);

                let mut parsed_sub_packets = Vec::new();
                for _ in 1..=number_of_sub_packets {
                    parsed_sub_packets.push(parse_packet(input));
                }

                Packet { version, payload: Payload::Operation {
                    type_id, packets: parsed_sub_packets 
                }}
            }
        }
    }
}

pub fn parse_literal(input: &mut Vec<Bit>) -> u64 {
    let mut next_chunk = input.drain(0..=4).collect::<Vec<_>>();
    let mut bits = Vec::new();
    while next_chunk.first().unwrap() != &Bit::Zero {
        bits.extend_from_slice(&next_chunk[1..=4]);
        next_chunk = input.drain(0..=4).collect();
    }

    bits.extend_from_slice(&next_chunk[1..=4]);
    to_number(&bits)
}

#[test]
pub fn test_to_number() {
    assert_eq!(to_number(&[Bit::One, Bit::Zero, Bit::Zero]), 4);
    assert_eq!(to_number(&[Bit::One, Bit::Zero, Bit::One]), 5);
    assert_eq!(to_number(&[Bit::Zero, Bit::One, Bit::One]), 3);
}

#[test]
pub fn test_parse_packet() {
    assert_eq!(parse_input("D2FE28"), Packet { version: 6, payload: Payload::Literal(2021)});
    assert_eq!(parse_input("38006F45291200"),
        Packet { version: 1, payload: Payload::Operation {
            type_id: 6, packets: vec![
                Packet { version: 6, payload: Payload::Literal(10)},
                Packet { version: 2, payload: Payload::Literal(20)}
            ]
        }});
    assert_eq!(parse_input("EE00D40C823060"),
        Packet { version: 7, payload: Payload::Operation {
            type_id: 3, packets: vec![
                Packet { version: 2, payload: Payload::Literal(1)},
                Packet { version: 4, payload: Payload::Literal(2)},
                Packet { version: 1, payload: Payload::Literal(3)}
            ]
        }});
}

#[test]
pub fn test() {
    assert_eq!(parse_input("8A004A801A8002F478").total_version(), 16);
    assert_eq!(parse_input("620080001611562C8802118E34").total_version(), 12);
    assert_eq!(parse_input("C0015000016115A2E0802F182340").total_version(), 23);
    assert_eq!(parse_input("A0016C880162017C3686B18A3D4780").total_version(), 31);

    assert_eq!(parse_input("C200B40A82").evaluate(), 3);
    assert_eq!(parse_input("04005AC33890").evaluate(), 54);
    assert_eq!(parse_input("880086C3E88112").evaluate(), 7);
    assert_eq!(parse_input("CE00C43D881120").evaluate(), 9);
    assert_eq!(parse_input("D8005AC2A8F0").evaluate(), 1);
    assert_eq!(parse_input("F600BC2D8F").evaluate(), 0);
    assert_eq!(parse_input("9C005AC2F8F0").evaluate(), 0);
    assert_eq!(parse_input("9C0141080250320F1802104A08").evaluate(), 1);
}

fn main() {
    let input = include_str!("../input.txt");
    let packet = parse_input(input);
    println!("Part 1: {}", packet.total_version());
    println!("Part 2: {}", packet.evaluate());
}
