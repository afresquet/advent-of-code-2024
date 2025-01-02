use std::collections::{HashMap, HashSet};

use itertools::Itertools;

advent_of_code::solution!(8);

fn parse_antennas(input: &str) -> HashMap<char, Vec<(isize, isize)>> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(x, character)| match character {
                    '.' => None,
                    antenna => Some((antenna, (x as isize, y as isize))),
                })
                .collect::<Vec<_>>()
        })
        .fold(HashMap::default(), |mut acc, (antenna, position)| {
            acc.entry(antenna)
                .and_modify(|array| array.push(position))
                .or_insert_with(|| vec![position]);
            acc
        })
}

pub fn part_one(input: &str) -> Option<u64> {
    let antennas = parse_antennas(input);

    let height = input.lines().count();
    let width = input.lines().next().expect("to have a row").len();

    let antinodes = antennas
        .values()
        .flat_map(|nodes| {
            nodes
                .iter()
                .flat_map(|node| {
                    nodes
                        .iter()
                        .filter_map(|inner_node| {
                            let direction_x = inner_node.0 - node.0;
                            let direction_y = inner_node.1 - node.1;

                            if direction_x == 0 && direction_y == 0 {
                                return None;
                            };

                            let x = inner_node.0 + direction_x;
                            let y = inner_node.1 + direction_y;

                            if x.is_negative()
                                || x >= width as isize
                                || y.is_negative()
                                || y >= height as isize
                            {
                                None
                            } else {
                                Some((x, y))
                            }
                        })
                        .collect_vec()
                })
                .collect_vec()
        })
        .collect::<HashSet<_>>();

    Some(antinodes.len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
