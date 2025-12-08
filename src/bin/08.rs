#![feature(binary_heap_into_iter_sorted)]

use std::cmp::Reverse;
use std::collections::BinaryHeap;
advent_of_code::solution!(8);

type Point = (u64, u64, u64);

#[inline(always)]
fn squared_distance(p1: Point, p2: Point) -> u64 {
    p1.0.abs_diff(p2.0).pow(2) + p1.1.abs_diff(p2.1).pow(2) + p1.2.abs_diff(p2.2).pow(2)
}

fn parse_input(input: &str) -> (Vec<Point>, impl Iterator<Item = (usize, usize)>) {
    let points = input
        .lines()
        .map(|line| {
            let mut r = line.split(',');
            (
                r.next().unwrap().parse::<u64>().unwrap(),
                r.next().unwrap().parse::<u64>().unwrap(),
                r.next().unwrap().parse::<u64>().unwrap(),
            )
        })
        .collect::<Vec<_>>();
    let entries = BinaryHeap::from_iter(points.iter().enumerate().flat_map(|(i, &p1)| {
        points
            .iter()
            .enumerate()
            .skip(i + 1)
            .map(move |(j, &p2)| Reverse((squared_distance(p1, p2), i, j)))
    }));
    (points, entries.into_iter_sorted().map(|r| (r.0.1, r.0.2)))
}

fn solve_p1<const CONNECTIONS: usize>(input: &str) -> u64 {
    let (points, entries) = parse_input(input);

    let mut dsu = aph_disjoint_set::DisjointSetArrayU16::<1000>::new();
    entries.take(CONNECTIONS).for_each(|(a, b)| {
        dsu.union(a, b);
    });

    let mut circuits = (0..points.len())
        .fold(std::collections::HashMap::new(), |mut acc, i| {
            *acc.entry(dsu.get_root(i)).or_default() += 1;
            acc
        })
        .into_values()
        .collect::<Vec<_>>();
    circuits.sort_unstable_by_key(|&circuit| std::cmp::Reverse(circuit));

    return circuits[0..3].iter().product::<u64>();
}

fn solve_p2(input: &str) -> u64 {
    let (points, entries) = parse_input(input);

    let mut dsu = aph_disjoint_set::DisjointSetArrayU16::<1000>::new();
    let (a, b) = entries
        .into_iter()
        .take(5 * points.len()) // empirical cutoff
        .filter(|&(a, b)| matches!(dsu.union(a, b), aph_disjoint_set::UnionResult::Success))
        .last()
        .unwrap();

    return points[a].0 * points[b].0;
}

#[inline(never)]
pub fn part_one(input: &str) -> Option<u64> {
    Some(solve_p1::<1000>(input))
}

#[inline(never)]
pub fn part_two(input: &str) -> Option<u64> {
    Some(solve_p2(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = solve_p1::<10>(&input);
        assert_eq!(result, 40);
    }

    #[test]
    fn test_part_two() {
        let result = solve_p2(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, 25272);
    }
}
