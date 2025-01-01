use std::{cmp::Ordering, collections::HashMap};

advent_of_code::solution!(5);

fn parse_input(input: &str) -> (HashMap<u64, Vec<u64>>, Vec<Vec<u64>>) {
    let (rules, updates) = input
        .split_once("\n\n")
        .expect("to have correct formatting");

    let rules = {
        let mut map = HashMap::new();

        let rules_iter = rules.lines().map(|line| {
            let (x, y) = line.split_once('|').expect("to have correct formatting");
            (
                x.parse().expect("to be a number"),
                y.parse().expect("to be a number"),
            )
        });

        for (x, y) in rules_iter {
            map.entry(x)
                .and_modify(|v: &mut Vec<u64>| v.push(y))
                .or_insert_with(|| vec![y]);
        }

        map
    };

    let updates = updates
        .lines()
        .map(|line| {
            line.split(',')
                .map(|x| x.parse().expect("to be a number"))
                .collect()
        })
        .collect();

    (rules, updates)
}

fn check_update_order(update: &[u64], rules: &HashMap<u64, Vec<u64>>) -> bool {
    update
        .iter()
        .enumerate()
        .all(|(j, page)| match rules.get(page) {
            Some(rule) => update.iter().take(j).all(|page| !rule.contains(page)),
            None => true,
        })
}

pub fn part_one(input: &str) -> Option<u64> {
    let (rules, updates) = parse_input(input);

    let middle_pages_sum = updates
        .into_iter()
        .filter(|update| check_update_order(update, &rules))
        .map(|update| {
            update
                .get(update.len() / 2)
                .expect("is not empty")
                .to_owned()
        })
        .sum();

    Some(middle_pages_sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (rules, updates) = parse_input(input);

    let middle_pages_sum = updates
        .into_iter()
        .filter(|update| !check_update_order(update, &rules))
        .map(|mut update| {
            update.sort_by(|a, b| match rules.get(a) {
                Some(rule) => {
                    if rule.contains(b) {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                }
                None => Ordering::Equal,
            });

            update
                .get(update.len() / 2)
                .expect("is not empty")
                .to_owned()
        })
        .sum();

    Some(middle_pages_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
