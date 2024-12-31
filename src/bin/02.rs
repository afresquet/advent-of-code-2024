advent_of_code::solution!(2);

#[derive(Debug)]
enum Direction {
    Unknown(u64),
    Increasing(u64),
    Decreasing(u64),
}

#[derive(Debug, Clone, Copy)]
enum Tolerance {
    Yes,
    No,
}

fn remove_at_index<T: Clone>(array: &[T], index: usize) -> Vec<T> {
    array
        .iter()
        .cloned()
        .enumerate()
        .filter_map(move |(i, value)| (index != i).then_some(value))
        .collect()
}

fn is_report_safe(report: &[u64], tolerance: Tolerance) -> bool {
    let mut levels = report.iter();
    let mut direction = Direction::Unknown(*levels.next().expect("has first item"));

    for level in levels {
        match direction {
            Direction::Unknown(value) | Direction::Increasing(value)
                if level > &value && (1..=3).contains(&value.abs_diff(*level)) =>
            {
                direction = Direction::Increasing(*level);
            }
            Direction::Unknown(value) | Direction::Decreasing(value)
                if level < &value && (1..=3).contains(&value.abs_diff(*level)) =>
            {
                direction = Direction::Decreasing(*level);
            }
            _ => {
                if let Tolerance::Yes = tolerance {
                    for index in 0..report.len() {
                        if is_report_safe(&remove_at_index(report, index), Tolerance::No) {
                            return true;
                        }
                    }
                }

                return false;
            }
        }
    }

    true
}

fn get_safe_amount(input: &str, tolerance: Tolerance) -> u64 {
    let reports = input.lines().map(|line| {
        line.split_whitespace()
            .map(|val| val.parse::<u64>().unwrap())
            .collect::<Vec<_>>()
    });

    reports
        .filter(|report| is_report_safe(report, tolerance))
        .count() as u64
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(get_safe_amount(input, Tolerance::No))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(get_safe_amount(input, Tolerance::Yes))
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
        assert_eq!(result, Some(4));
    }

    #[test]
    fn tolerance() {
        let result = part_two(
            "1 5 6 7 8
1 6 2 3 4
1 6 10 11 12
1 4 2 4 5",
        );
        assert_eq!(result, Some(3));
    }
}
