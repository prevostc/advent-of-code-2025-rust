advent_of_code::solution!(2);

use rayon::prelude::*;
use std::ops::RangeInclusive;

fn stream_input(input: &str) -> impl Iterator<Item = RangeInclusive<usize>> {
    input
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.split('-').map(|s| s.parse::<usize>().unwrap()))
        .map(|mut iter| (iter.next().unwrap(), iter.next().unwrap()))
        .map(|(start, end)| start..=end)
}

pub fn part_one(input: &str) -> Option<u64> {
    let res = stream_input(input)
        .par_bridge()
        .flat_map(|range| range.into_iter())
        .filter(|i| {
            let digits = i.to_string().into_bytes();
            digits.len() % 2 == 0 && digits[0..digits.len() / 2] == digits[digits.len() / 2..]
        })
        .sum::<usize>();
    Some(res as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let res = stream_input(input)
        .flat_map(|range| range.into_iter())
        .par_bridge()
        .filter(|&i| {
            let digits = i.to_string().into_bytes();
            (1..=digits.len() / 2)
                .filter(|&size| digits.len() % size == 0)
                .any(|size| {
                    let mut chunks = digits.chunks(size);
                    chunks
                        .next()
                        .map_or(false, |first| chunks.all(|chunk| chunk == first))
                })
        })
        .sum::<usize>();
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
