advent_of_code::solution!(4);

fn transpose<T: Clone>(grid: &[Vec<T>]) -> Vec<Vec<T>> {
    (0..grid[0].len())
        .map(|i| {
            grid.iter()
                .map(|inner| inner[i].clone())
                .collect::<Vec<T>>()
        })
        .collect()
}

#[derive(Debug)]
enum Direction {
    Regular,
    Reverse,
}

fn diagonal_transpose<T: Clone>(grid: &[Vec<T>], direction: Direction) -> Vec<Vec<T>> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut result: Vec<Vec<T>> = vec![vec![]; rows + cols - 1];

    for (i, row) in grid.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            let diagonal_index = if let Direction::Regular = direction {
                i + j
            } else {
                rows - 1 - i + j
            };
            result[diagonal_index].push(col.clone());
        }
    }

    result
}

type Grid = Vec<Vec<char>>;

pub fn word_search(grid: &Grid) -> u64 {
    swipe_twice(grid)
        + swipe_twice(&transpose(grid))
        + swipe_twice(&diagonal_transpose(grid, Direction::Regular))
        + swipe_twice(&diagonal_transpose(grid, Direction::Reverse))
}

fn swipe_twice(grid: &Grid) -> u64 {
    west_to_east(grid) + east_to_west(grid)
}

fn west_to_east(grid: &Grid) -> u64 {
    grid.iter()
        .map(|row| {
            row.windows(4)
                .filter(|window| window == &['X', 'M', 'A', 'S'])
                .count() as u64
        })
        .sum()
}

fn east_to_west(grid: &Grid) -> u64 {
    grid.iter()
        .map(|row| {
            row.windows(4)
                .rev()
                .filter(|window| window == &['S', 'A', 'M', 'X'])
                .count() as u64
        })
        .sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = input.lines().map(|line| line.chars().collect()).collect();

    Some(word_search(&grid))
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
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
