use std::iter::repeat;

use itertools::Itertools;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let mut disk = input
        .trim()
        .chars()
        .map(|digit| digit.to_digit(10).expect("to be a number") as usize)
        .enumerate()
        .flat_map(|(i, amount)| match i % 2 {
            0 => repeat(Some((i / 2) as u64)).take(amount),
            1 => repeat(None).take(amount),
            _ => unreachable!(),
        })
        .collect_vec();

    'outer: for i in 0..disk.len() {
        if disk[i].is_some() {
            continue;
        }

        for j in (0..disk.len()).rev() {
            if i == j {
                break 'outer;
            }

            if let Some(id) = disk[j].take() {
                let _ = disk[i].insert(id);

                break;
            }
        }
    }

    Some(
        disk.into_iter()
            .flatten()
            .enumerate()
            .map(|(i, id)| i as u64 * id)
            .sum(),
    )
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
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
