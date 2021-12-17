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

    fn get_version_sum(&self) -> u64 {
        let mut sum = 0_u64;
        sum
    }
}

fn build_packet(bits: &[u8]) -> Result<(Packet, usize), ()> {
    println!("Len: {}", bits.len());
    if bits.len() < HEADER_LEN {
        // Return error?
        return Err(());
    }

    let version = (bits[0] << 2) + (bits[1] << 1) + bits[2];
    let id = (bits[3] << 2) + (bits[4] << 1) + bits[5];

    let mut body = &bits[HEADER_LEN..];
    match id {
        4 => {
            // Literal package
            // Read bundles of 5 bits until the first bit is zero
            let mut literal = Vec::<u8>::new();

            let mut ptr = 0;
            println!("body len {:?}", body);
            while ptr + 5 <= body.len() {
                println!("ptr: {}", ptr);
                ptr += 5;
                literal.push(body[ptr + 1]);
                literal.push(body[ptr + 2]);
                literal.push(body[ptr + 3]);
                literal.push(body[ptr + 4]);
                if body[ptr] == 0 {
                    break;
                }
            }

            let parsed_literal = to_num(&literal);
            println!("Literal: {:?} = {}", literal, parsed_literal);
            return Ok((
                Packet { version, id, literal: parsed_literal, sub_packets: Vec::<Packet>::new() },
                ptr + HEADER_LEN
            ));

        },
        _ => {
            // Control package containing sub-packages
            match body[0] {
                0 => {
                    if body.len() < 16 {
                        return Err(());
                    }

                    // Next 15 bits are total length in bits of the sub-packets
                    let sub_bits = to_num(&body[1..16]) as usize;
                    let mut sub_packets = Vec::<Packet>::new();

                    let mut ptr = 0_usize;
                    while ptr < sub_bits {
                        let (packet, end_bit) = build_packet(&body[16+ptr..])?;
                        ptr += end_bit;
                        sub_packets.push(packet);
                    }

                    return Ok((Packet { version, id, literal: 0_u64, sub_packets }, ptr + 16 + HEADER_LEN));
                },
                1 => {
                    if body.len() < 12 {
                        return Err(());
                    }

                    // Next 11 bits are total number of sub-packets immediately contained
                    let sub_packet_num = to_num(&body[1..12]) as usize;
                    let mut sub_packets = Vec::<Packet>::with_capacity(sub_packet_num);

                    let mut ptr = 0_usize;
                    for _ in 0..sub_packet_num {
                        let (packet, end_bit) = build_packet(&body[12+ptr..])?;
                        ptr += end_bit;
                        sub_packets.push(packet);
                    }

                    return Ok((Packet { version, id, literal: 0_u64, sub_packets }, ptr + 12 + HEADER_LEN));
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
    println!("Version: {} - ID: {} - Literal: {}", packet.version, packet.id, packet.literal);
}

fn main() {
    let mut input = parse_input("in.txt")
        .expect("Failed to parse input");

    println!("Input: {:?}", input);

    one(&input);
}
