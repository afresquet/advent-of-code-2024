use std::collections::HashSet;

advent_of_code::solution!(6);

type Map = Vec<Vec<char>>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
    direction: Direction,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Position {
    pub fn from_map(map: &Map) -> Option<Self> {
        map.iter().enumerate().find_map(|(y, row)| {
            row.iter().enumerate().find_map(|(x, pos)| match pos {
                '^' => Some(Position::up(x, y)),
                'v' => Some(Position::down(x, y)),
                '<' => Some(Position::left(x, y)),
                '>' => Some(Position::right(x, y)),
                _ => None,
            })
        })
    }

    pub fn up(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            direction: Direction::Up,
        }
    }

    pub fn down(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            direction: Direction::Down,
        }
    }

    pub fn left(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            direction: Direction::Left,
        }
    }

    pub fn right(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            direction: Direction::Right,
        }
    }

    pub fn step(&self, map: &Map) -> Result<Self, ()> {
        match self.direction {
            Direction::Up => match map
                .get(self.y.checked_sub(1).unwrap_or(usize::MAX))
                .and_then(|row| row.get(self.x))
            {
                Some(&'#') => Self::step(&Self::right(self.x, self.y), map),
                Some(_) => Ok(Self::up(self.x, self.y - 1)),
                None => Err(()),
            },
            Direction::Down => match map.get(self.y + 1).and_then(|row| row.get(self.x)) {
                Some(&'#') => Self::step(&Self::left(self.x, self.y), map),
                Some(_) => Ok(Self::down(self.x, self.y + 1)),
                None => Err(()),
            },
            Direction::Left => match map
                .get(self.y)
                .and_then(|row| row.get(self.x.checked_sub(1).unwrap_or(usize::MAX)))
            {
                Some(&'#') => Self::step(&Self::up(self.x, self.y), map),
                Some(_) => Ok(Self::left(self.x - 1, self.y)),
                None => Err(()),
            },
            Direction::Right => match map.get(self.y).and_then(|row| row.get(self.x + 1)) {
                Some(&'#') => Self::step(&Self::down(self.x, self.y), map),
                Some(_) => Ok(Self::right(self.x + 1, self.y)),
                None => Err(()),
            },
        }
    }

    pub fn to_coords(&self) -> (usize, usize) {
        (self.x, self.y)
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let map: Map = input.lines().map(|line| line.chars().collect()).collect();
    let mut position = Position::from_map(&map).expect("to have starting position");
    let mut positions = HashSet::new();

    loop {
        positions.insert(position.to_coords());
        match position.step(&map) {
            Ok(pos) => position = pos,
            Err(_) => break,
        }
    }

    Some(positions.len() as u64)
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
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
