use std::collections::HashMap;

advent_of_code::solution!(1);

fn parse_lists(input: &str) -> (Vec<u64>, Vec<u64>) {
    let mut left = vec![];
    let mut right = vec![];
    for line in input.lines() {
        let mut values = line.split_whitespace();
        left.push(
            values
                .next()
                .and_then(|value| value.parse::<u64>().ok())
                .unwrap(),
        );
        right.push(
            values
                .next()
                .and_then(|value| value.parse::<u64>().ok())
                .unwrap(),
        );
    }
    (left, right)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (mut left, mut right) = parse_lists(input);
    left.sort();
    right.sort();

    Some(
        left.into_iter()
            .zip(right)
            .map(|(l, r)| l.abs_diff(r))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let (left, right) = parse_lists(input);
    let right = {
        let mut map = HashMap::new();
        for value in right {
            map.entry(value)
                .and_modify(|v: &mut u64| *v += 1)
                .or_insert(1);
        }
        map
    };

    Some(
        left.into_iter()
            .map(|value| value * right.get(&value).unwrap_or(&0))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
