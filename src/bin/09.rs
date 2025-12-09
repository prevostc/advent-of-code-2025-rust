use std::cmp::Reverse;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

advent_of_code::solution!(9);

fn parse_input(input: &str) -> Vec<(i64, i64)> {
    input
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(x, y)| (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap()))
        .collect::<Vec<_>>()
}

#[inline(always)]
fn compute_area(p1: &(i64, i64), p2: &(i64, i64)) -> i64 {
    let width = (p2.0 - p1.0).abs() + 1;
    let height = (p2.1 - p1.1).abs() + 1;
    width * height
}

#[inline(never)]
pub fn part_one(input: &str) -> Option<i64> {
    let pos = parse_input(input);

    // brute force largest rectangle
    let mut max_area = 0;
    for i in 0..pos.len() {
        for j in 0..pos.len() {
            let area = compute_area(&pos[i], &pos[j]);
            max_area = max_area.max(area);
        }
    }

    Some(max_area)
}

#[inline(always)]
fn bbox(p1: &(i64, i64), p2: &(i64, i64)) -> [i64; 4] {
    [
        p1.0.min(p2.0), // xmin
        p1.0.max(p2.0), // xmax
        p1.1.min(p2.1), // ymin
        p1.1.max(p2.1), // ymax
    ]
}

#[inline(never)]
pub fn part_two(input: &str) -> Option<i64> {
    let mut pos = parse_input(input);

    // compute all the possible rectangles and sort by area descending
    let mut rectangles = Vec::with_capacity(pos.len() * pos.len());
    for i in 0..pos.len() {
        for j in 0..pos.len() {
            let area = compute_area(&pos[i], &pos[j]);
            rectangles.push((pos[i], pos[j], area));
        }
    }
    // rectangles.sort_by_key(|(_, _, area)| Reverse(*area));

    pos.push(pos[0]); // make it a closed polygon

    // pairs of points are lines, find the largest rectangle that doesn't intersect with a line
    let max_area = rectangles
        .par_iter()
        .map(|(p1, p2, area)| {
            let [bxmin, bxmax, bymin, bymax] = bbox(p1, p2);
            for w in pos.windows(2) {
                let [lxmin, lxmax, lymin, lymax] = bbox(&w[0], &w[1]);
                if lxmin == lxmax {
                    // is a vertical line
                    if lxmin > bxmin && lxmax < bxmax {
                        // vertical line is inside bbox vertical bounds
                        if !(lymax <= bymin || lymin >= bymax) {
                            // vertical line intersects with bbox
                            return None;
                        }
                    }
                }

                if lymin == lymax {
                    // is a horizontal line
                    if lymin > bymin && lymax < bymax {
                        // horizontal line is inside bbox horizontal bounds
                        if !(lxmax <= bxmin || lxmin >= bxmax) {
                            // horizontal line intersects with bbox
                            return None;
                        }
                    }
                }
            }

            Some(*area)
        })
        .filter_map(|area| area)
        .max()?;

    Some(max_area)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
