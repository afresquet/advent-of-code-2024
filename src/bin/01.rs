use std::str::FromStr;

advent_of_code::solution!(1);

#[derive(Debug, PartialEq, Eq)]
enum Schematic {
    Lock([u8; 5]),
    Key([u8; 5]),
}

impl Schematic {
    pub fn heights(&self) -> &[u8; 5] {
        match self {
            Schematic::Lock(h) => h,
            Schematic::Key(h) => h,
        }
    }
}

impl FromStr for Schematic {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (lock, lines): (bool, Box<dyn Iterator<Item = &str>>) = if s.starts_with("#####") {
            (true, Box::new(s.lines().skip(1)))
        } else {
            let count = s.lines().count();
            (false, Box::new(s.lines().take(count - 1)))
        };

        let mut heights = [0; 5];

        for line in lines {
            for (i, c) in line.chars().enumerate() {
                if c == '#' {
                    heights[i] += 1;
                }
            }
        }

        if lock {
            Ok(Schematic::Lock(heights))
        } else {
            Ok(Schematic::Key(heights))
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let (locks, keys) = input
        .split("\n\n")
        .map(|s| s.parse::<Schematic>().unwrap())
        .fold((vec![], vec![]), |mut acc, s| {
            match s {
                Schematic::Lock(_) => acc.0.push(s),
                Schematic::Key(_) => acc.1.push(s),
            };
            acc
        });

    let fit_amount = locks
        .iter()
        .map(|lock| {
            keys.iter()
                .filter(|key| {
                    let key_heights = key.heights();

                    lock.heights()
                        .iter()
                        .enumerate()
                        .all(|(i, h)| h + key_heights[i] <= 5)
                })
                .count() as u64
        })
        .sum();

    Some(fit_amount)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lock_from_schematic() {
        let input = "#####
.####
.####
.####
.#.#.
.#...
.....";

        assert_eq!(input.parse(), Ok(Schematic::Lock([0, 5, 3, 4, 3])))
    }

    #[test]
    fn key_from_schematic() {
        let input = ".....
#....
#....
#...#
#.#.#
#.###
#####";

        assert_eq!(input.parse(), Ok(Schematic::Key([5, 0, 2, 1, 3])))
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
