use core::panic;
use std::iter::repeat;

use itertools::Itertools;

advent_of_code::solution!(9);

fn parse_disk(input: &str) -> Vec<Option<u64>> {
    input
        .trim()
        .chars()
        .map(|digit| digit.to_digit(10).expect("to be a number") as usize)
        .enumerate()
        .flat_map(|(i, amount)| match i % 2 {
            0 => repeat(Some((i / 2) as u64)).take(amount),
            1 => repeat(None).take(amount),
            _ => unreachable!(),
        })
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut disk = parse_disk(input);

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
    let mut disk = parse_disk(input);
    let files = input
        .trim()
        .chars()
        .enumerate()
        .filter_map(|(i, digit)| {
            if i % 2 == 0 {
                Some(digit.to_digit(10).expect("to be a number") as u64)
            } else {
                None
            }
        })
        .collect_vec();
    let mut free_spaces = input
        .trim()
        .chars()
        .enumerate()
        .filter_map(|(i, digit)| {
            if i % 2 == 0 {
                None
            } else {
                Some(digit.to_digit(10).expect("to be a number") as u64)
            }
        })
        .collect_vec();
    let free_spaces_original = free_spaces.clone();

    for (i, file) in files.iter().enumerate().rev() {
        for j in 0..free_spaces.len() {
            if i == j {
                break;
            }

            if file > &free_spaces[j] {
                continue;
            }

            let file_skips = files.iter().take(j + 1).sum::<u64>();
            let free_space_skips = free_spaces.iter().take(j).sum::<u64>();

            let free_space_index = disk
                .iter()
                .enumerate()
                .skip((file_skips + free_space_skips) as usize)
                .find(|(_, data)| data.is_none())
                .map(|(i, _)| i)
                .expect("to be empty");

            let file_skips = files.iter().take(i).sum::<u64>();
            let free_space_skips = free_spaces_original.iter().take(i).sum::<u64>();

            let file_index = (file_skips + free_space_skips) as usize;

            for (free_space, file) in (free_space_index..free_space_index + *file as usize)
                .zip(file_index..file_index + *file as usize)
            {
                if let Some(data) = disk[file].take() {
                    let _ = disk[free_space].insert(data);
                } else {
                    panic!("accessing empty memory");
                }
            }

            free_spaces[j] -= file;

            break;
        }
    }

    Some(
        disk.into_iter()
            .enumerate()
            .map(|(i, id)| i as u64 * id.unwrap_or(0))
            .sum(),
    )
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
        assert_eq!(result, Some(2858));
    }
}
