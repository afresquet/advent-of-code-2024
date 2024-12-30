use std::{
    collections::HashMap,
    ops::{BitAnd, BitOr, BitXor},
};

advent_of_code::solution!(2);

#[derive(Debug)]
struct Gate<'a> {
    a: &'a str,
    b: &'a str,
    c: &'a str,
    gate_type: GateType,
}

impl<'a> From<&'a str> for Gate<'a> {
    fn from(value: &'a str) -> Self {
        let mut split = value.split_whitespace();

        let a = split.next().unwrap();
        let gate_type = match split.next().unwrap() {
            "AND" => GateType::And,
            "OR" => GateType::Or,
            "XOR" => GateType::Xor,
            _ => unreachable!(),
        };
        let b = split.next().unwrap();
        let _ = split.next().unwrap();
        let c = split.next().unwrap();

        Self { a, b, c, gate_type }
    }
}

#[derive(Debug)]
enum GateType {
    And,
    Or,
    Xor,
}

pub fn part_one(input: &str) -> Option<u64> {
    let (wires, gates) = input
        .split_once("\n\n")
        .expect("to have correct formatting");
    let mut wires = wires
        .lines()
        .map(|line| {
            let (wire, value) = line.split_once(": ").expect("to have correct formatting");
            (wire, value.parse::<u8>().expect("to be a bit"))
        })
        .collect::<HashMap<_, _>>();
    let gates = gates.lines().map(|line| line.into()).collect::<Vec<Gate>>();

    let mut remaining = gates.len();
    while remaining > 0 {
        for gate in &gates {
            if wires.contains_key(gate.c) {
                continue;
            }
            let a = match wires.get(gate.a) {
                Some(a) => a,
                None => continue,
            };
            let b = match wires.get(gate.b) {
                Some(b) => b,
                None => continue,
            };
            let value = match gate.gate_type {
                GateType::And => a.bitand(b),
                GateType::Or => a.bitor(b),
                GateType::Xor => a.bitxor(b),
            };
            wires.insert(gate.c, value);
            remaining -= 1;
        }
    }

    let mut z_wires = wires
        .iter()
        .filter(|(wire, _)| wire.starts_with("z"))
        .collect::<Vec<_>>();
    z_wires.sort_by_key(|wire| wire.0);
    let binary = z_wires
        .iter()
        .rev()
        .map(|(_, value)| value.to_string())
        .collect::<String>();

    u64::from_str_radix(&binary, 2).ok()
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_short_example() {
        let result = part_one(
            "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02",
        );
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2024));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
