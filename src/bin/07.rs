use itertools::Itertools;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let calibration_equations = input
        .lines()
        .map(|line| {
            let (test_value, numbers) = line.split_once(": ").expect("to have correct formatting");
            (
                test_value.parse::<u64>().expect("to be a number"),
                numbers
                    .split_whitespace()
                    .map(|n| n.parse::<u64>().expect("to be a number"))
                    .collect::<Vec<_>>(),
            )
        })
        .filter_map(|(test_value, numbers)| {
            let chars = ['+', '*'];
            let combinations = (0..numbers.len() - 1)
                .map(|_| chars.iter())
                .multi_cartesian_product()
                .map(|combo| combo.into_iter().copied().collect::<Vec<_>>());

            for combination in combinations {
                let mut combination_iter = combination.iter();

                let value = numbers
                    .iter()
                    .cloned()
                    .reduce(|acc, cur| match combination_iter.next() {
                        Some('+') => acc + cur,
                        Some('*') => acc * cur,
                        _ => unreachable!(),
                    })
                    .expect("to work");

                if value == test_value {
                    return Some(test_value);
                }
            }

            None
        });

    Some(calibration_equations.sum())
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
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
