advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
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
    None
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
        assert_eq!(result, None);
    }
}
