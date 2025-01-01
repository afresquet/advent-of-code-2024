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

type Grid<T> = Vec<Vec<T>>;

pub fn word_search_xmas(grid: &Grid<char>) -> u64 {
    swipe_twice_xmas(grid)
        + swipe_twice_xmas(&transpose(grid))
        + swipe_twice_xmas(&diagonal_transpose(grid, Direction::Regular))
        + swipe_twice_xmas(&diagonal_transpose(grid, Direction::Reverse))
}

fn swipe_twice_xmas(grid: &Grid<char>) -> u64 {
    left_to_right(grid, &['X', 'M', 'A', 'S']) + right_to_left(grid, &['S', 'A', 'M', 'X'])
}

fn left_to_right<T: PartialEq, const N: usize>(grid: &Grid<T>, chars: &[T; N]) -> u64 {
    grid.iter()
        .map(|row| row.windows(N).filter(|window| window == chars).count() as u64)
        .sum()
}

fn right_to_left<T: PartialEq, const N: usize>(grid: &Grid<T>, chars: &[T; N]) -> u64 {
    grid.iter()
        .map(|row| {
            row.windows(N)
                .rev()
                .filter(|window| window == chars)
                .count() as u64
        })
        .sum()
}

fn word_search_cross_mas(grid: &Grid<char>) -> u64 {
    if grid.len() < 3 || grid.first().expect("is not empty").len() < 3 {
        return 0;
    }

    grid.iter()
        .enumerate()
        .take(grid.len() - 2)
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .take(row.len() - 2)
                .filter(|(j, col)| {
                    let top_left = col;
                    let top_right = row.get(j + 2);
                    let middle = grid.get(i + 1).and_then(|row| row.get(j + 1));
                    let bottom_left = grid.get(i + 2).and_then(|row| row.get(*j));
                    let bottom_right = grid.get(i + 2).and_then(|row| row.get(j + 2));
                    match (top_left, middle, bottom_right) {
                        ('M', Some('A'), Some('S')) | ('S', Some('A'), Some('M')) => {
                            matches!(
                                (top_right, bottom_left),
                                (Some('M'), Some('S')) | (Some('S'), Some('M'))
                            )
                        }
                        _ => false,
                    }
                })
                .count() as u64
        })
        .sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = input.lines().map(|line| line.chars().collect()).collect();

    Some(word_search_xmas(&grid))
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid = input.lines().map(|line| line.chars().collect()).collect();

    Some(word_search_cross_mas(&grid))
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
        assert_eq!(result, Some(9));
    }
}
