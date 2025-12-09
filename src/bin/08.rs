#![feature(binary_heap_into_iter_sorted)]

use kdtree::KdTree;
use kdtree::distance::squared_euclidean;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

advent_of_code::solution!(8);

fn parse_input(input: &str) -> (Vec<[f64; 3]>, impl Iterator<Item = (usize, usize)>) {
    let points = input
        .lines()
        .map(|line| {
            let mut r = line.split(',');
            [
                r.next().unwrap().parse::<u64>().unwrap() as f64,
                r.next().unwrap().parse::<u64>().unwrap() as f64,
                r.next().unwrap().parse::<u64>().unwrap() as f64,
            ]
        })
        .collect::<Vec<_>>();

    let mut kdtree = KdTree::new(3);
    for (i, p) in points.iter().enumerate() {
        kdtree.add(p, i).unwrap();
    }

    let entries = BinaryHeap::from_iter(points.iter().enumerate().flat_map(|(i, &p1)| {
        kdtree
            .iter_nearest_mut(&p1, &squared_euclidean)
            .unwrap()
            .skip(1) // skip self
            .take(10) // empirical cutoff
            .map(|(d, ni)| Reverse((d as u64, i, *ni)))
            .filter(|r| r.0.1 > r.0.2) // skip duplicates
            .collect::<Vec<_>>()
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
        // .take(5 * points.len()) // empirical cutoff
        .filter(|&(a, b)| matches!(dsu.union(a, b), aph_disjoint_set::UnionResult::Success))
        .last()
        .unwrap();

    return points[a][0] as u64 * points[b][0] as u64;
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
