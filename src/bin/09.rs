use std::cmp::Reverse;

advent_of_code::solution!(9);

#[inline(never)]
pub fn part_one(input: &str) -> Option<i64> {
    let pos = input
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(x, y)| (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap()))
        .collect::<Vec<_>>();

    // brute force largest rectangle
    let mut max_area = 0;
    for i in 0..pos.len() {
        for j in 0..pos.len() {
            let width = (pos[j].0 - pos[i].0).abs() + 1;
            let height = (pos[j].1 - pos[i].1).abs() + 1;
            let area = width * height;
            max_area = max_area.max(area);
        }
    }

    Some(max_area)
}

#[inline(never)]
pub fn part_two(input: &str) -> Option<i64> {
    let pos = input
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(x, y)| (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap()))
        .collect::<Vec<_>>();

    // compute all the possible rectangles and sort by area descending
    let mut rectangles = Vec::with_capacity(pos.len() * pos.len());
    for i in 0..pos.len() {
        for j in 0..pos.len() {
            let width = (pos[j].0 - pos[i].0).abs() + 1;
            let height = (pos[j].1 - pos[i].1).abs() + 1;
            let area = width * height;
            rectangles.push((pos[i], pos[j], area));
        }
    }
    rectangles.sort_by_key(|(_, _, area)| Reverse(*area));

    #[inline(always)]
    fn bbox(p1: &(i64, i64), p2: &(i64, i64)) -> [i64; 4] {
        [
            p1.0.min(p2.0), // xmin
            p1.0.max(p2.0), // xmax
            p1.1.min(p2.1), // ymin
            p1.1.max(p2.1), // ymax
        ]
    }

    // pairs of points are lines, find the largest rectangle that doesn't intersect with a line
    'outer: for (p1, p2, area) in rectangles.iter() {
        let [bxmin, bxmax, bymin, bymax] = bbox(p1, p2);
        let mut prev_p = &pos[pos.len() - 1];
        for p in pos.iter() {
            let [lxmin, lxmax, lymin, lymax] = bbox(prev_p, p);
            if lxmin == lxmax && lxmin > bxmin && lxmax < bxmax {
                if !(lymax <= bymin || lymin >= bymax) {
                    continue 'outer;
                }
            } else if lymin == lymax && lymin > bymin && lymax < bymax {
                if !(lxmax <= bxmin || lxmin >= bxmax) {
                    continue 'outer;
                }
            }
            prev_p = p;
        }

        return Some(*area);
    }
    None
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
