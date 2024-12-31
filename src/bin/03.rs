use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    sequence::{delimited, preceded},
    IResult, Parser,
};

advent_of_code::solution!(3);

fn parse_mul(input: &str) -> IResult<&str, (u64, u64)> {
    if !input.starts_with("mul(") {
        return Err(nom::Err::Incomplete(nom::Needed::Unknown));
    }

    let (input, x) = preceded(tag("mul("), digit1.map(|i: &str| i.parse().unwrap()))(input)?;
    let (input, y) =
        delimited(tag(","), digit1.map(|i: &str| i.parse().unwrap()), tag(")"))(input)?;

    Ok((input, (x, y)))
}

fn parse_muls(mut input: &str) -> Vec<(u64, u64)> {
    let mut muls = Vec::new();

    while !input.is_empty() {
        if let Ok((s, mul)) = parse_mul(input) {
            muls.push(mul);
            input = s;
        } else {
            input = &input[1..];
        }
    }

    muls
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(parse_muls(input).into_iter().map(|(x, y)| x * y).sum())
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
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
