use mygrid::{direction::ALL_AROUND, grid::Grid, point::Point};
use rayon::iter::ParallelIterator;

advent_of_code::solution!(4);

#[inline(always)]
fn is_removable(grid: &Grid<u8>, p: Point) -> bool {
    let mut count = 0;
    for d in ALL_AROUND {
        if Some(&b'@') == grid.get_item(p + d) {
            count += 1;
        }
        if count >= 4 {
            return false;
        }
    }
    true
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = Grid::new_from_str(input, |c| c as u8);
    let res = grid
        .par_iter_item_and_position()
        .filter(|(_, c)| **c == b'@')
        .filter(|(p, _)| is_removable(&grid, *p))
        .count();
    Some(res as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = Grid::new_from_str(input, |c| c as u8);

    let mut all_rolls = grid
        .iter_item_and_position()
        .filter(|(_, c)| **c == b'@')
        .map(|(p, _)| (false, p))
        .collect::<Vec<_>>();

    let mut removed_one = true;
    let mut count = 0;
    while removed_one {
        removed_one = false;
        for (removed, p) in all_rolls.iter_mut() {
            if !*removed && is_removable(&grid, *p) {
                grid[*p] = b'.';
                *removed = true;
                removed_one = true;
                count += 1;
            }
        }
    }
    Some(count as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
