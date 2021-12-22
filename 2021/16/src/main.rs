use std::{env, fs, path::Path, slice::Iter};

// single wrapper packet
// multiple subpackets
// ignore trailing zeros
// most significant bits first
// packet content from start:
// * 3 bit version (number 0-7)
// * 3 bit type (number 0-7)
// * type 4 is literal value
// * literal values encoded as multiple of 4 bits with leading zeros for padding
// * each four bits have leading 1 (each group is 5 bits) except last group

#[derive(Debug, Clone)]
struct Input(Vec<bool>);

struct Packet {
    version: u64,
    packettype: PacketType,
    content: Content<u64, Vec<Packet>>,
    length: usize,
}

enum Content<T1, T2> {
    Value(T1),
    Subpackets(T2),
}

#[derive(Debug, PartialEq, Eq)]
enum PacketType {
    Literal,
    Sum,
    Mul,
    Min,
    Max,
    Gt,
    Lt,
    Eq,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = args.get(1).expect("Missing file parameter");
    if !Path::new(file).exists() {
        panic!("No such file {}", file);
    }
    let data = fs::read_to_string(file).expect("Something went wrong reading the file");

    let input = parse_input(&data);

    part1(&input);
    part2(&input);
}

fn parse_input(data: &str) -> Input {
    Input(
        data.split("\n")
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .map(|l| {
                l.chars()
                    .map(|s| s.to_digit(16).unwrap() as u8)
                    .collect::<Vec<u8>>()
            })
            .flatten()
            .flat_map(|n| (0..=3).rev().map(move |b| ((n >> b) & 1) == 1))
            .collect(),
    )
}

fn part1(input: &Input) {
    let mut bitstream = input.bitstream();
    let packet = Packet::from(&mut bitstream);

    println!("## Part 1");
    println!("Result: {}", packet.version_sum());
}

fn part2(input: &Input) {
    let mut bitstream = input.bitstream();
    let packet = Packet::from(&mut bitstream);

    println!("## Part 2");
    println!("Result: {}", packet.calculate());
}

impl Input {
    fn bitstream(&self) -> Iter<bool> {
        self.0.iter()
    }
}

impl From<&mut Iter<'_, bool>> for Packet {
    fn from(bitstream: &mut Iter<bool>) -> Self {
        let (version, vlen) = Self::decode_version(bitstream);
        let (packettype, tlen) = Self::decode_type(bitstream);
        let (content, clen) = Self::decode_content(bitstream, &packettype);
        // if let Content::Value(v) = content {
        //     println!("{:?} {}", packettype, v);
        // } else {
        //     println!("{:?}", packettype);
        // }
        Packet {
            version,
            packettype,
            content,
            length: vlen + tlen + clen,
        }
    }
}
impl Packet {
    fn decode_version(bitstream: &mut Iter<bool>) -> (u64, usize) {
        let v = Self::bits_to_num(bitstream, 3);
        (v, 3)
    }

    fn decode_type(bitstream: &mut Iter<bool>) -> (PacketType, usize) {
        let t = Self::bits_to_num(bitstream, 3);
        (
            match t {
                0 => PacketType::Sum,
                1 => PacketType::Mul,
                2 => PacketType::Min,
                3 => PacketType::Max,
                4 => PacketType::Literal,
                5 => PacketType::Gt,
                6 => PacketType::Lt,
                7 => PacketType::Eq,
                _ => panic!("Unknown packet type"),
            },
            3,
        )
    }

    fn bits_to_num(bitstream: &mut Iter<bool>, count: usize) -> u64 {
        let t = bitstream
            .take(count)
            .enumerate()
            .fold(0, |result, (b, is_set)| {
                if *is_set {
                    result + (1 << (count - b - 1))
                } else {
                    result
                }
            });
        t
    }

    fn decode_content(
        bitstream: &mut Iter<bool>,
        packettype: &PacketType,
    ) -> (Content<u64, Vec<Packet>>, usize) {
        match packettype {
            PacketType::Literal => {
                let (v, len) = Self::decode_literal(bitstream);
                (Content::Value(v), len)
            }
            _ => {
                let mode = bitstream.next().unwrap();
                match mode {
                    false => {
                        let len = 16; // mode + length bits
                        let sublen = Self::bits_to_num(bitstream, 15) as usize;
                        let mut subpkgs = vec![];
                        let mut count = 0;
                        while count < sublen {
                            let packet = Packet::from(&mut *bitstream);
                            count += packet.len();
                            subpkgs.push(packet);
                        }
                        (Content::Subpackets(subpkgs), len + count)
                    }
                    true => {
                        let mut len = 12; // mode + length bits
                        let subcount = Self::bits_to_num(bitstream, 11) as usize;
                        let mut subpkgs = vec![];
                        (0..subcount).for_each(|_| {
                            let packet = Packet::from(&mut *bitstream);
                            len += packet.len();
                            subpkgs.push(packet);
                        });
                        (Content::Subpackets(subpkgs), len)
                    }
                }
            }
        }
    }

    fn decode_literal(bitstream: &mut Iter<bool>) -> (u64, usize) {
        let mut result = 0;
        let mut count = 0;
        while let Some(more) = bitstream.next() {
            let n = Self::bits_to_num(bitstream, 4);
            result = (result << 4) + n;
            count += 5;
            if !more {
                break;
            }
        }
        (result, count)
    }

    fn len(&self) -> usize {
        self.length
    }

    fn version_sum(&self) -> u64 {
        match &self.content {
            Content::Value(_) => self.version,
            Content::Subpackets(pkgs) => {
                self.version + pkgs.iter().map(|p| p.version_sum()).sum::<u64>()
            }
        }
    }

    fn calculate(&self) -> u64 {
        match &self.content {
            Content::Value(v) => *v,
            Content::Subpackets(pkgs) => match self.packettype {
                PacketType::Sum => pkgs.iter().map(|p| p.calculate()).sum::<u64>(),
                PacketType::Mul => pkgs
                    .iter()
                    .map(|p| p.calculate())
                    .reduce(|a, b| a * b)
                    .unwrap(),
                PacketType::Min => pkgs.iter().map(|p| p.calculate()).min().unwrap(),
                PacketType::Max => pkgs.iter().map(|p| p.calculate()).max().unwrap(),
                PacketType::Gt => {
                    if pkgs.len() != 2 {
                        panic!("Bad Gt packet, need exactly two subpackets.")
                    }
                    one_or_zero(pkgs[0].calculate() > pkgs[1].calculate())
                }
                PacketType::Lt => {
                    if pkgs.len() != 2 {
                        panic!("Bad Lt packet, need exactly two subpackets.")
                    }
                    one_or_zero(pkgs[0].calculate() < pkgs[1].calculate())
                }
                PacketType::Eq => {
                    if pkgs.len() != 2 {
                        panic!("Bad Eq packet, need exactly two subpackets.")
                    }
                    one_or_zero(pkgs[0].calculate() == pkgs[1].calculate())
                }
                PacketType::Literal => {
                    panic!("Unexpected packet type literal, should have no subpackets")
                }
            },
        }
    }
}

fn one_or_zero(one: bool) -> u64 {
    match one {
        true => 1,
        false => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_example1() {
        let input = parse_input("D2FE28");
        let mut bitstream = input.bitstream();

        let packet = Packet::from(&mut bitstream);
        assert_eq!(packet.version, 6);
        assert_eq!(packet.packettype, PacketType::Literal);

        match packet.content {
            Content::Value(v) => assert_eq!(v, 2021),
            _ => assert!(false),
        }
        assert_eq!(packet.len(), 21);
    }

    #[test]
    fn p1_example2() {
        let input = parse_input("38006F45291200");
        let mut bitstream = input.bitstream();
        let packet = Packet::from(&mut bitstream);
        assert_eq!(packet.version, 1);
        assert_eq!(packet.packettype, PacketType::Lt);
    }

    #[test]
    fn p1_example3() {
        let input = parse_input("EE00D40C823060");
        let mut bitstream = input.bitstream();
        let packet = Packet::from(&mut bitstream);
        assert_eq!(packet.version, 7);
        assert_eq!(packet.packettype, PacketType::Max);
    }

    #[test]
    fn p1_example4() {
        let input = parse_input("8A004A801A8002F478");
        let mut bitstream = input.bitstream();
        let packet = Packet::from(&mut bitstream);

        assert_eq!(packet.version_sum(), 16);
    }

    #[test]
    fn p1_example5() {
        let input = parse_input("620080001611562C8802118E34");
        let mut bitstream = input.bitstream();
        let packet = Packet::from(&mut bitstream);

        assert_eq!(packet.version_sum(), 12);
    }

    #[test]
    fn p1_example6() {
        let input = parse_input("C0015000016115A2E0802F182340");
        let mut bitstream = input.bitstream();
        let packet = Packet::from(&mut bitstream);

        assert_eq!(packet.version_sum(), 23);
    }

    #[test]
    fn p1_example7() {
        let input = parse_input("A0016C880162017C3686B18A3D4780");
        let mut bitstream = input.bitstream();
        let packet = Packet::from(&mut bitstream);

        assert_eq!(packet.version_sum(), 31);
    }

    #[test]
    fn p2_example1() {
        let input = parse_input("C200B40A82");
        let mut bitstream = input.bitstream();
        let packet = Packet::from(&mut bitstream);

        assert_eq!(packet.calculate(), 3);
    }

    #[test]
    fn p2_example2() {
        let input = parse_input("04005AC33890");
        let mut bitstream = input.bitstream();
        let packet = Packet::from(&mut bitstream);

        assert_eq!(packet.calculate(), 54);
    }

    #[test]
    fn p2_example3() {
        let input = parse_input("880086C3E88112");
        let mut bitstream = input.bitstream();
        let packet = Packet::from(&mut bitstream);

        assert_eq!(packet.calculate(), 7);
    }

    #[test]
    fn p2_example4() {
        let input = parse_input("CE00C43D881120");
        let mut bitstream = input.bitstream();
        let packet = Packet::from(&mut bitstream);

        assert_eq!(packet.calculate(), 9);
    }

    #[test]
    fn p2_example5() {
        let input = parse_input("D8005AC2A8F0");
        let mut bitstream = input.bitstream();
        let packet = Packet::from(&mut bitstream);

        assert_eq!(packet.calculate(), 1);
    }

    #[test]
    fn p2_example6() {
        let input = parse_input("F600BC2D8F");
        let mut bitstream = input.bitstream();
        let packet = Packet::from(&mut bitstream);

        assert_eq!(packet.calculate(), 0);
    }

    #[test]
    fn p2_example7() {
        let input = parse_input("9C005AC2F8F0");
        let mut bitstream = input.bitstream();
        let packet = Packet::from(&mut bitstream);

        assert_eq!(packet.calculate(), 0);
    }

    #[test]
    fn p2_example8() {
        let input = parse_input("9C0141080250320F1802104A08");
        let mut bitstream = input.bitstream();
        let packet = Packet::from(&mut bitstream);

        assert_eq!(packet.calculate(), 1);
    }
}
