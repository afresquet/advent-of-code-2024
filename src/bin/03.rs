use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

advent_of_code::solution!(3);

fn parse_mul(input: &str) -> IResult<&str, (u64, u64)> {
    delimited(
        tag("mul("),
        separated_pair(
            digit1.map(|i: &str| i.parse().unwrap()),
            tag(","),
            digit1.map(|i: &str| i.parse().unwrap()),
        ),
        tag(")"),
    )(input)
}

#[derive(Debug)]
enum Enabled {
    True,
    False,
}

fn parse_muls(mut input: &str, instructions: Enabled) -> Vec<(u64, u64)> {
    let mut muls = Vec::new();
    let mut enabled = Enabled::True;

    while !input.is_empty() {
        match enabled {
            Enabled::True => {
                if let Enabled::True = instructions {
                    if input.starts_with("don't()") {
                        enabled = Enabled::False;
                        input = &input["don't()".len()..];
                        continue;
                    }
                }

                if let Ok((s, mul)) = parse_mul(input) {
                    muls.push(mul);
                    input = s;
                } else {
                    input = &input[input.chars().next().unwrap().len_utf8()..];
                }
            }
            Enabled::False => {
                if input.starts_with("do()") {
                    enabled = Enabled::True;
                    input = &input["do()".len()..];
                } else {
                    input = &input[input.chars().next().unwrap().len_utf8()..];
                }
            }
        }
    }

    muls
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        parse_muls(input, Enabled::False)
            .into_iter()
            .map(|(x, y)| x * y)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        parse_muls(input, Enabled::True)
            .into_iter()
            .map(|(x, y)| x * y)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
        assert_eq!(result, Some(48));
    }

    #[test]
    fn from_reddit() {
        let result = part_two(
            "mul(1,2)don't()badstuffmul(30,10)

stillbadmul(2,100)do()mul(2,4)",
        );
        assert_eq!(result, Some(10));
    }
}
