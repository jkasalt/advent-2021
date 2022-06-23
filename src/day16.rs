use std::ops::Deref;

use bitvec::prelude::*;

pub fn hex_to_bin(input: &str) -> BitVec {
    input
        .trim()
        .chars()
        .flat_map(|ch| match ch {
            '0' => bitvec![0, 0, 0, 0],
            '1' => bitvec![0, 0, 0, 1],
            '2' => bitvec![0, 0, 1, 0],
            '3' => bitvec![0, 0, 1, 1],
            '4' => bitvec![0, 1, 0, 0],
            '5' => bitvec![0, 1, 0, 1],
            '6' => bitvec![0, 1, 1, 0],
            '7' => bitvec![0, 1, 1, 1],
            '8' => bitvec![1, 0, 0, 0],
            '9' => bitvec![1, 0, 0, 1],
            'A' => bitvec![1, 0, 1, 0],
            'B' => bitvec![1, 0, 1, 1],
            'C' => bitvec![1, 1, 0, 0],
            'D' => bitvec![1, 1, 0, 1],
            'E' => bitvec![1, 1, 1, 0],
            'F' => bitvec![1, 1, 1, 1],
            x => panic!("Unexpected input {}", x),
        })
        .collect()
}

pub fn as_number<I, K>(iter: I) -> u64
where
    I: Iterator<Item = K> + std::iter::DoubleEndedIterator,
    K: Deref<Target = bool>,
{
    iter.rev().enumerate().fold(0, |acc, (i, bit)| {
        acc + 2_u64.pow(i.try_into().unwrap()) * *bit as u64
    })
}

#[derive(Debug)]
struct Packet {
    version: u32,
    type_id: u32,
    sub_packets: Vec<Packet>,
    value: Option<u64>,
}

impl Packet {
    fn new(input: &BitSlice, head: &mut usize) -> Self {
        let version = as_number(input[*head..*head + 3].iter())
            .try_into()
            .unwrap();
        *head += 3;
        let type_id = as_number(input[*head..*head + 3].iter())
            .try_into()
            .unwrap();
        *head += 3;

        if type_id == 4 {
            let mut value = BitVec::<Lsb0>::new();
            for chunk in input[*head..].chunks(5) {
                value.extend_from_bitslice(&chunk[1..]);
                *head += 5;
                if !chunk[0] {
                    break;
                }
            }
            Self {
                version,
                type_id,
                sub_packets: vec![],
                value: Some(as_number(value.iter())),
            }
        } else {
            let mut sub_packets = Vec::new();
            // Depending on length type ID
            if !input[*head] {
                *head += 1;
                let length = as_number(input[*head..*head + 15].iter()) as usize;
                *head += 15;
                let mark = *head;
                while *head < mark + length {
                    let packet = Packet::new(input, head);
                    sub_packets.push(packet);
                }
            } else {
                *head += 1;
                let num_packets = as_number(input[*head..*head + 11].iter()) as usize;
                *head += 11;
                while sub_packets.len() < num_packets {
                    sub_packets.push(Packet::new(input, head));
                }
            }
            Self {
                version,
                type_id,
                sub_packets,
                value: None,
            }
        }
    }

    fn sum(&self) -> u128 {
        self.version as u128 + self.sub_packets.iter().map(|p| p.sum()).sum::<u128>()
    }

    fn evaluate(&self) -> u128 {
        let mut sub_values = self.sub_packets.iter().map(|p| p.evaluate());
        match self.type_id {
            0 => sub_values.sum(),
            1 => sub_values.product(),
            2 => sub_values.min().unwrap(),
            3 => sub_values.max().unwrap(),
            4 => self.value.unwrap() as u128,
            5 => (sub_values.next().unwrap() > sub_values.next().unwrap()) as u128,
            6 => (sub_values.next().unwrap() < sub_values.next().unwrap()) as u128,
            7 => (sub_values.next().unwrap() == sub_values.next().unwrap()) as u128,
            _ => unreachable!(),
        }
    }
}

pub fn first(input: &BitSlice) -> u128 {
    let packet = Packet::new(input, &mut 0);
    packet.sum()
}

pub fn second(input: &BitSlice) -> u128 {
    let packet = Packet::new(input, &mut 0);
    packet.evaluate()
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test_case("8A004A801A8002F478" => 16; "Sample 1")]
    #[test_case("620080001611562C8802118E34" => 12; "Sample 2")]
    #[test_case("C0015000016115A2E0802F182340" => 23; "Sample 3")]
    #[test_case("A0016C880162017C3686B18A3D4780" => 31; "Sample 4")]
    fn one(input: &str) -> u128 {
        let input = hex_to_bin(input);
        let a = Packet::new(&input, &mut 0);
        a.sum()
    }

    #[test_case("C200B40A82" => 3; "Sample 1")]
    #[test_case("04005AC33890" => 54; "Sample 2")]
    #[test_case("880086C3E88112" => 7; "Sample 3")]
    #[test_case("CE00C43D881120" => 9; "Sample 4")]
    #[test_case("D8005AC2A8F0" => 1; "Sample 5")]
    #[test_case("F600BC2D8F" => 0; "Sample 6")]
    #[test_case("9C005AC2F8F0" => 0; "Sample 7")]
    #[test_case("9C0141080250320F1802104A08" => 1; "Sample 8")]
    fn two(input: &str) -> u128 {
        let input = hex_to_bin(input);
        let a = Packet::new(&input, &mut 0);
        a.evaluate()
    }
}
