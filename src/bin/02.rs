advent_of_code::solution!(2);

use num::Integer;
use rayon::prelude::*;
use std::ops::RangeInclusive;

const POW10: [u64; 16] = [
    1,
    10,
    100,
    1_000,
    10_000,
    100_000,
    1_000_000,
    10_000_000,
    100_000_000,
    1_000_000_000,
    10_000_000_000,
    100_000_000_000,
    1_000_000_000_000,
    10_000_000_000_000,
    100_000_000_000_000,
    1_000_000_000_000_000,
];

fn stream_input(input: &str) -> impl Iterator<Item = RangeInclusive<u64>> {
    input
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.split('-').map(|s| s.parse::<u64>().unwrap()))
        .map(|mut iter| (iter.next().unwrap(), iter.next().unwrap()))
        .map(|(start, end)| start..=end)
}

#[inline(never)]
pub fn part_one(input: &str) -> Option<u64> {
    let res = stream_input(input)
        .par_bridge()
        .flat_map(|range| range.into_iter())
        .filter(|num| {
            let digits = num.ilog10() + 1;
            let (q, r) = num.div_mod_floor(&POW10[(digits / 2) as usize]);
            q == r
        })
        .sum::<u64>();
    Some(res as u64)
}

#[inline(never)]
pub fn part_two(input: &str) -> Option<u64> {
    let res = stream_input(input)
        .par_bridge()
        .flat_map(|range| range)
        .filter(|&i| {
            let n = i;
            let len = (n.ilog10() + 1) as usize;
            (1..=len / 2).any(|l| {
                len.is_multiple_of(l) && {
                    let shift = POW10[l];
                    let prefix = n / POW10[len - l];
                    let multiplier = (POW10[len] - 1) / (shift - 1); // geometric series
                    prefix * multiplier == n
                }
            })
        })
        .sum::<u64>();
    Some(res as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
