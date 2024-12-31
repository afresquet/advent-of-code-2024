advent_of_code::solution!(2);

enum Direction {
    Unknown(u64),
    Increasing(u64),
    Decreasing(u64),
}

pub fn part_one(input: &str) -> Option<u64> {
    let reports = input.lines().map(|line| {
        line.split_whitespace()
            .map(|val| val.parse::<u64>().unwrap())
            .collect::<Vec<_>>()
    });

    let safe = reports
        .filter(|levels| {
            let mut levels_iter = levels.iter();
            let mut direction = Direction::Unknown(*levels_iter.next().expect("has first item"));

            for level in levels_iter {
                match direction {
                    Direction::Unknown(value) if (1..=3).contains(&value.abs_diff(*level)) => {
                        if value < *level {
                            direction = Direction::Increasing(*level);
                        } else {
                            direction = Direction::Decreasing(*level);
                        }
                    }
                    Direction::Increasing(value)
                        if level > &value && (1..=3).contains(&value.abs_diff(*level)) =>
                    {
                        direction = Direction::Increasing(*level);
                    }
                    Direction::Decreasing(value)
                        if level < &value && (1..=3).contains(&value.abs_diff(*level)) =>
                    {
                        direction = Direction::Decreasing(*level);
                    }
                    _ => return false,
                }
            }

            true
        })
        .count();

    Some(safe as u64)
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
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
