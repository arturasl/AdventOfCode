use anyhow::{ensure, Context, Ok, Result};
use std::io::{self, BufRead};
use std::thread;

#[derive(Debug)]
enum Data {
    Literal(i64),
    Other(Vec<Packet>),
}

#[derive(Debug)]
struct Packet {
    len: usize,
    version: i64,
    type_id: i64,
    data: Data,
}

fn to_num(bits: &[u8]) -> i64 {
    let mut num: i64 = 0;
    for b in bits {
        num = (num << 1) | (*b as i64);
    }
    num
}

fn parse(bits: &[u8]) -> Result<Packet> {
    let mut start: usize = 0;
    let version = to_num(&bits[start..start + 3]);
    start += 3;
    let type_id = to_num(&bits[start..start + 3]);
    start += 3;

    let data = if type_id == 4 {
        let mut collected: Vec<u8> = Vec::new();
        loop {
            let first_bit = bits[start];
            collected.extend(&bits[start + 1..start + 5]);
            start += 5;
            if first_bit == 0 {
                break;
            }
        }

        Data::Literal(to_num(&collected))
    } else {
        let first_bit = bits[start];
        start += 1;
        let mut other: Vec<Packet> = Vec::new();

        match first_bit {
            0 => {
                let mut len_in_bits = to_num(&bits[start..start + 15]);
                start += 15;
                while len_in_bits != 0 {
                    let parsed = parse(&bits[start..])?;
                    len_in_bits -= parsed.len as i64;
                    start += parsed.len;
                    other.push(parsed);
                    ensure!(len_in_bits >= 0);
                }
            }
            1 => {
                let len_in_packets = to_num(&bits[start..start + 11]);
                start += 11;
                for _ in 0..len_in_packets {
                    let parsed = parse(&bits[start..])?;
                    start += parsed.len;
                    other.push(parsed);
                }
            }
            _ => ensure!(false),
        };
        Data::Other(other)
    };

    Ok(Packet {
        len: start,
        version,
        type_id,
        data,
    })
}

fn sum_version_numbesr(packet: &Packet) -> i64 {
    let mut result: i64 = packet.version;
    match &packet.data {
        Data::Literal(..) => {}
        Data::Other(o) => {
            for other_packet in o {
                result += sum_version_numbesr(other_packet);
            }
        }
    }
    result
}

fn run() -> Result<()> {
    for maybe_line in io::stdin().lock().lines() {
        let line = maybe_line?.trim().to_owned();
        if line.is_empty() {
            continue;
        }

        let mut bits: Vec<u8> = Vec::new();
        for ch in line.chars() {
            let num = ch.to_digit(16).context("")?;
            for i in (0..4).rev() {
                bits.push(((num & (1 << i)) >> i) as u8);
            }
        }
        let parsed = parse(&bits)?;
        ensure!(bits.iter().skip(parsed.len).all(|b| *b == 0));
        println!("{:?}", sum_version_numbesr(&parsed));
    }
    Ok(())
}

fn main() -> Result<()> {
    thread::Builder::new()
        .stack_size(20 * 1024 * 1024)
        .spawn(run)?
        .join()
        .unwrap()
}
