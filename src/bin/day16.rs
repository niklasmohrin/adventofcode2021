use std::io::{self, BufRead};

use itertools::Itertools;

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let mut bit_stream = line
        .chars()
        .map(|hex| hex.to_digit(16).unwrap())
        .flat_map(|n| [(n >> 3) & 1, (n >> 2) & 1, (n >> 1) & 1, (n >> 0) & 1].map(|v| v as u8));

    fn next_num(bit_iter: impl Iterator<Item = u8>, bits: usize) -> u64 {
        bit_iter.take(bits).fold(0, |acc, cur| 2 * acc + cur as u64)
    }

    fn next_literal(bit_iter: impl Iterator<Item = u8>) -> u64 {
        let chunks = bit_iter.chunks(5);
        let parts = chunks.into_iter().map(|sub_iter| next_num(sub_iter, 5));
        let mut res = 0;
        for v in parts {
            res <<= 4;
            res |= v & 0b1111;
            if v & 0b10000 == 0 {
                break;
            }
        }
        res
    }

    fn evaluate_next_packet(
        mut bit_iter: &mut dyn Iterator<Item = u8>,
        version_sum: &mut u64,
    ) -> u64 {
        let version = next_num(&mut bit_iter, 3);
        *version_sum += version;
        let type_id = next_num(&mut bit_iter, 3);

        if type_id == 4 {
            return next_literal(&mut bit_iter);
        }

        let length_type_id = bit_iter.next().unwrap();
        let subpacket_values = match length_type_id {
            0 => {
                let total_sub_length = next_num(&mut bit_iter, 15) as _;
                let mut subpackets_bits = bit_iter.take(total_sub_length).peekable();
                let mut subpacket_values = Vec::new();
                while subpackets_bits.peek().is_some() {
                    subpacket_values.push(evaluate_next_packet(&mut subpackets_bits, version_sum));
                }
                subpacket_values
            }
            1 => {
                let sub_packet_count = next_num(&mut bit_iter, 11);
                (0..sub_packet_count)
                    .map(|_| evaluate_next_packet(bit_iter, version_sum))
                    .collect()
            }
            _ => panic!(),
        };

        match type_id {
            0 => subpacket_values.into_iter().sum(),
            1 => subpacket_values.into_iter().product(),
            2 => subpacket_values.into_iter().min().unwrap(),
            3 => subpacket_values.into_iter().max().unwrap(),
            5 => (subpacket_values[0] > subpacket_values[1]) as _,
            6 => (subpacket_values[0] < subpacket_values[1]) as _,
            7 => (subpacket_values[0] == subpacket_values[1]) as _,
            _ => panic!(),
        }
    }

    let mut version_sum = 0;
    dbg!(evaluate_next_packet(&mut bit_stream, &mut version_sum));
    dbg!(version_sum);
}
