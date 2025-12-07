use mygrid::{grid::Grid, point::Point};

advent_of_code::solution!(7);

fn solve(input: &str) -> (u64, u64) {
    let grid = Grid::new_from_str(input, |c| c);
    let start_col = grid.width / 2;

    let mut beams = vec![0u64; grid.width];
    beams[start_col] = 1;
    let mut splits = 0;

    for row in 1..(grid.height - 1) {
        let mut new_beams = vec![0u64; grid.width];
        for col in 0..grid.width {
            if beams[col] == 0 {
                continue;
            }
            match grid[Point::new_usize(row + 1, col)] {
                '^' => {
                    if col > 0 {
                        new_beams[col - 1] += beams[col];
                    }
                    if col + 1 < grid.width {
                        new_beams[col + 1] += beams[col];
                    }
                    splits += 1;
                }
                '.' => {
                    new_beams[col] += beams[col];
                }
                _ => unreachable!(),
            }
        }
        beams = new_beams;
    }

    let timelines = beams.iter().sum::<u64>();
    (splits, timelines)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (splits, _) = solve(input);
    Some(splits)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, timelines) = solve(input);
    Some(timelines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
