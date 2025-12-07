use std::collections::VecDeque;

use mygrid::{direction::ALL_AROUND, grid::Grid, point::Point};

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

#[inline(never)]
pub fn part_one(input: &str) -> Option<u64> {
    let grid = Grid::new_from_str(input, |c| c as u8);
    let res = grid
        .iter_item_and_position()
        .filter(|(_, c)| **c == b'@')
        .filter(|(p, _)| is_removable(&grid, *p))
        .count();
    Some(res as u64)
}

#[inline(never)]
pub fn part_two(input: &str) -> Option<u64> {
    let grid = Grid::new_from_str(input, |c| c as u8);
    let mut q = VecDeque::with_capacity(grid.width * grid.height);
    let mut grid_count = Grid::new(grid.width, grid.height, 0_isize);
    for (p, c) in grid.iter_item_and_position() {
        if *c == b'@' {
            let count = ALL_AROUND
                .iter()
                .filter(|&d| Some(&b'@') == grid.get_item(p + *d))
                .count() as isize;
            grid_count[p] = count;
            if count > 0 && count < 4 {
                q.push_back(p);
            }
        }
    }

    let mut total_removed: u64 = 0;
    while let Some(p) = q.pop_front() {
        if grid_count[p] < 0 {
            continue;
        }

        grid_count[p] = -1;
        total_removed += 1;
        // update neighbors counts
        for d in ALL_AROUND {
            let p_d = p + d;
            if let Some(count) = grid_count.get_item_mut(p_d) {
                *count -= 1;
                if *count > 0 && *count < 4 {
                    q.push_back(p_d);
                }
            }
        }
    }
    Some(total_removed)
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
