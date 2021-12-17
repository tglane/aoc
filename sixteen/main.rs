use std::fs::File;
use std::io::{BufRead, BufReader, Error};

const HEADER_LEN: usize = 6;

struct Packet {
    version: u8,
    id: u8,
    literal: u64,
    sub_packets: Vec<Packet>,
}

impl Packet {
    fn from(bits: &[u8]) -> Self {
        let (packet, _) = build_packet(&bits).unwrap();
        packet
    }

    fn version_sum(&self) -> u64 {
        let mut sum = self.version as u64;
        for sub in self.sub_packets.iter() {
            sum += sub.version_sum();
        }
        sum
    }

    fn evaluate(&self) -> u64 {
        match self.id {
            4 => {
                // Literal
                return self.literal;
            },
            0 => {
                // Sum packet
                let mut sum = 0;
                for packet in self.sub_packets.iter() {
                    sum += packet.evaluate();
                }
                return sum;
            },
            1 => {
                // Product packet
                let mut prod = 1;
                for packet in self.sub_packets.iter() {
                    prod *= packet.evaluate();
                }
                return prod;
            },
            2 => {
                // Minimum packet
                let mut min = u64::MAX;
                for packet in self.sub_packets.iter() {
                    let val = packet.evaluate();
                    min = std::cmp::min(val, min);
                }
                return min;
            },
            3 => {
                // Maximum packet
                let mut max = u64::MIN;
                for packet in self.sub_packets.iter() {
                    let val = packet.evaluate();
                    max = std::cmp::max(val, max);
                }
                return max;
            },
            5 => {
                // Greater-than packet
                return (self.sub_packets[0].evaluate() > self.sub_packets[1].evaluate()) as u64;
            },
            6 => {
                // Less-than packet
                return (self.sub_packets[0].evaluate() < self.sub_packets[1].evaluate()) as u64;
            },
            7 => {
                // Equal-to packet
                return (self.sub_packets[0].evaluate() == self.sub_packets[1].evaluate()) as u64;
            }
            _ => {
                return 0;
            }
        }
    }
}

fn build_packet(bits: &[u8]) -> Result<(Packet, usize), ()> {
    if bits.len() < HEADER_LEN {
        return Err(());
    }

    let version = (bits[0] << 2) + (bits[1] << 1) + bits[2];
    let id = (bits[3] << 2) + (bits[4] << 1) + bits[5];

    let body = &bits[HEADER_LEN..];
    match id {
        4 => {
            // Literal package - Read bundles of 5 bits until the first bit is zero
            let mut literal = Vec::<u8>::new();

            let mut ptr = 0;
            while ptr <= body.len() - 5 {
                // Parse a block of 5 bits where the first bit indicats continuation status
                literal.push(body[ptr + 1]);
                literal.push(body[ptr + 2]);
                literal.push(body[ptr + 3]);
                literal.push(body[ptr + 4]);

                if body[ptr] == 0 {
                    ptr += 5;
                    break;
                }
                ptr += 5;
            }

            let parsed_literal = to_num(&literal);
            return Ok((
                Packet { version, id, literal: parsed_literal, sub_packets: Vec::<Packet>::new() },
                ptr + HEADER_LEN
            ));
        },
        _ => {
            // Operator package containing sub-packages to be parsed separately
            match body[0] {
                0 => {
                    if body.len() < 16 {
                        return Err(());
                    }

                    // Next 15 bits are total length in bits of the sub-packets
                    let sub_bits = to_num(&body[1..16]) as usize;
                    let mut sub_packets = Vec::<Packet>::new();

                    let mut ptr = 16_usize;
                    while ptr < sub_bits + 16 {
                        let (packet, end_bit) = build_packet(&body[ptr..])?;
                        ptr += end_bit;
                        sub_packets.push(packet);
                    }

                    return Ok((Packet { version, id, literal: 0_u64, sub_packets }, ptr + HEADER_LEN));
                },
                1 => {
                    if body.len() < 12 {
                        return Err(());
                    }

                    // Next 11 bits are total number of sub-packets immediately contained
                    let sub_packet_num = to_num(&body[1..12]) as usize;
                    let mut sub_packets = Vec::<Packet>::with_capacity(sub_packet_num);

                    let mut ptr = 12_usize;
                    for _ in 0..sub_packet_num {
                        let (packet, end_bit) = build_packet(&body[ptr..])?;
                        ptr += end_bit;
                        sub_packets.push(packet);
                    }

                    return Ok((Packet { version, id, literal: 0_u64, sub_packets }, ptr + HEADER_LEN));
                },
                _ => {
                    return Err(());
                }
            }

        }
    }
}

fn to_num(bits: &[u8]) -> u64 {
    let mut num = 0_u64;
    for (i, &bit) in bits.iter().rev().enumerate() {
        num += (bit as u64) << i;
    }
    num
}

fn parse_input(filename: &str) -> Result<Vec<u8>, Error> {
    let reader = BufReader::new(File::open(filename)?);
    let line = reader.lines().next().unwrap().unwrap();

    let mut out = Vec::<u8>::with_capacity(line.len() * 4);
    for c in line.chars() {
        let x = c.to_digit(16).unwrap() as u8;
        out.extend([x >> 3, (x >> 2) & 1, (x >> 1) & 1, x & 1]);
    }

    Ok(out)
}

fn one(input: &Vec<u8>) {
    let packet = Packet::from(&input);
    println!("ONE: Version sum: {}", packet.version_sum());
    println!("TWO: Packet evaluates to {}", packet.evaluate());
}

fn main() {
    let input = parse_input("in.txt")
        .expect("Failed to parse input");

    one(&input);
}
