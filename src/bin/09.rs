use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

advent_of_code::solution!(9);

////////////////////////////////////////////////////////////////
///  Helper functions
////////////////////////////////////////////////////////////////

#[inline(always)]
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

#[inline(always)]
fn bbox(p1: &(i64, i64), p2: &(i64, i64)) -> [i64; 4] {
    [
        p1.0.min(p2.0), // xmin
        p1.0.max(p2.0), // xmax
        p1.1.min(p2.1), // ymin
        p1.1.max(p2.1), // ymax
    ]
}

#[inline(always)]
fn bbox_intersects_line(&[bxmin, bxmax, bymin, bymax]: &[i64; 4], line: &[(i64, i64); 2]) -> bool {
    let [lxmin, lxmax, lymin, lymax] = bbox(&line[0], &line[1]);
    if lxmin == lxmax {
        // is a vertical line
        if lxmin > bxmin && lxmax < bxmax {
            // vertical line is inside bbox vertical bounds
            if !(lymax <= bymin || lymin >= bymax) {
                // vertical line intersects with bbox
                return true;
            }
        }
    }

    if lymin == lymax {
        // is a horizontal line
        if lymin > bymin && lymax < bymax {
            // horizontal line is inside bbox horizontal bounds
            if !(lxmax <= bxmin || lxmin >= bxmax) {
                // horizontal line intersects with bbox
                return true;
            }
        }
    }

    false
}

////////////////////////////////////////////////////////////////
///  Brute force solutions
///     Use the brute force approach to solve the problem
///     Works for all inputs but is a tad slow (~20ms for part 2)
////////////////////////////////////////////////////////////////

pub fn part_one_brute(input: &str) -> Option<i64> {
    let pos = parse_input(input);

    let mut max_area = 0;
    for i in 0..pos.len() {
        for j in 0..pos.len() {
            let area = compute_area(&pos[i], &pos[j]);
            max_area = max_area.max(area);
        }
    }

    Some(max_area)
}

pub fn part_two_brute(input: &str) -> Option<i64> {
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
        .map(|(p1, p2, area)| (bbox(p1, p2), area))
        .filter(|&([bxmin, bxmax, bymin, bymax], _)| {
            !pos.windows(2).any(|w| {
                bbox_intersects_line(
                    &[bxmin, bxmax, bymin, bymax],
                    &[(w[0].0, w[0].1), (w[1].0, w[1].1)],
                )
            })
        })
        .map(|(_, area)| *area)
        .max()?;

    Some(max_area)
}

////////////////////////////////////////////////////////////////
///  Cheated solutions
///     Use the shape of the input to make the solution faster
////////////////////////////////////////////////////////////////

#[inline(always)]
fn build_cheat_index<F>(pos: &[(i64, i64)], get_cutoff: F) -> [Vec<&(i64, i64)>; 4]
where
    F: Fn(i64, i64) -> (i64, i64),
{
    let capacity = pos.len();
    let [xmin, xmax, ymin, ymax] = pos.iter().fold(
        [i64::MAX, i64::MIN, i64::MAX, i64::MIN],
        |[xmin, xmax, ymin, ymax], (x, y)| [xmin.min(*x), xmax.max(*x), ymin.min(*y), ymax.max(*y)],
    );
    let [xmid, ymid] = [(xmin + xmax) / 2, (ymin + ymax) / 2];
    let (xcutoff, ycutoff) = get_cutoff(xmax, ymax);

    let mut index = [
        Vec::with_capacity(capacity), // top left
        Vec::with_capacity(capacity), // top right
        Vec::with_capacity(capacity), // bottom left
        Vec::with_capacity(capacity), // bottom right
    ];

    for p in pos
        .iter()
        // remove points too close to the center, empirical cutoff
        .filter(|p| (p.0 - xmid).abs() > xcutoff && (p.1 - ymid).abs() > ycutoff)
    {
        let idx = (p.0 > xmid) as usize * 2 + (p.1 > ymid) as usize;
        index[idx].push(p);
    }
    index
}

pub fn part_one_cheated(input: &str) -> Option<i64> {
    let pos = parse_input(input);
    let corners = build_cheat_index(&pos, |xmax, ymax| (xmax / 10, ymax / 10));

    // the input draws a big circle with an horizontal cut off
    // only look at points in opposite corners of the map
    let mut max_area = 0;

    // explore top left and bottom right corners
    for a in corners[0].iter() {
        for b in corners[3].iter() {
            let area = compute_area(a, b);
            max_area = max_area.max(area);
        }
    }
    // explore top right and bottom left corners
    for a in corners[1].iter() {
        for b in corners[2].iter() {
            let area = compute_area(a, b);
            max_area = max_area.max(area);
        }
    }

    Some(max_area)
}

pub fn part_two_cheated(input: &str) -> Option<i64> {
    let pos = parse_input(input);
    let corners = build_cheat_index(&pos, |_, _| (20000, 0));

    let mut rectangles = Vec::with_capacity(corners[0].len() * corners[1].len());
    for a in corners[0].iter() {
        for b in corners[2].iter() {
            let area = compute_area(a, b);
            rectangles.push((a, b, area, [0, 2]));
        }
    }

    for a in corners[1].iter() {
        for b in corners[3].iter() {
            let area = compute_area(a, b);
            rectangles.push((a, b, area, [1, 3]));
        }
    }

    // pairs of points are lines, find the largest rectangle that doesn't intersect with a line
    let max_area = rectangles
        .par_iter()
        .map(|(p1, p2, area, idxs)| (bbox(p1, p2), area, idxs))
        .filter(|&([bxmin, bxmax, bymin, bymax], _, idxs)| {
            !corners[idxs[0]]
                .windows(2)
                .chain(corners[idxs[1]].windows(2))
                .any(|w| {
                    bbox_intersects_line(
                        &[bxmin, bxmax, bymin, bymax],
                        &[(w[0].0, w[0].1), (w[1].0, w[1].1)],
                    )
                })
        })
        .map(|(_, area, _)| *area)
        .max()?;

    Some(max_area)
}

////////////////////////////////////////////////////////////////
///  Entry point
////////////////////////////////////////////////////////////////

#[inline(never)]
pub fn part_one(input: &str) -> Option<i64> {
    part_one_cheated(input)
}

#[inline(never)]
pub fn part_two(input: &str) -> Option<i64> {
    part_two_cheated(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_brute(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two_brute(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
