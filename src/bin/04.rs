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

fn find_crosses<const N: usize>(grid: &Grid<char>, chars: &[char; N]) -> u64 {
    let len = grid.first().expect("is not empty").len() - 1;

    let grid = diagonal_transpose(grid, Direction::Regular);

    grid.iter()
        .enumerate()
        .map(|(i, row)| {
            row.windows(N)
                .enumerate()
                .filter_map(|(j, window)| {
                    if window != chars {
                        return None;
                    };

                    let up_col = j + 2.min(0.max(i.saturating_sub(len)));
                    let down_col = j + (0.max(2.min(len.saturating_sub(i))));

                    let up = grid.get(i - 2).and_then(|r| r.get(up_col))?;
                    let down = grid.get(i + 2).and_then(|r| r.get(down_col))?;

                    ((up == &'M' && down == &'S') || (up == &'S' && down == &'M')).then_some(true)
                })
                .count() as u64
        })
        .sum()
}

fn word_search_cross_mas(grid: &Grid<char>) -> u64 {
    find_crosses(grid, &['M', 'A', 'S']) + find_crosses(grid, &['S', 'A', 'M'])
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
