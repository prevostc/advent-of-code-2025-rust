use std::ops::RangeInclusive;

advent_of_code::solution!(5);

type IdRange = RangeInclusive<u64>;

fn parse_range(s: &str) -> IdRange {
    let (start, end) = s.split_once('-').unwrap();
    start.parse::<u64>().unwrap()..=end.parse::<u64>().unwrap()
}

fn parse_input(input: &str) -> (Vec<IdRange>, impl Iterator<Item = u64>) {
    let mut lines = input.lines();
    let mut ranges: Vec<IdRange> = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(parse_range)
        .collect();
    ranges.sort_by(|a, b| a.start().cmp(b.start()));

    let numbers = lines
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<u64>().unwrap());
    (ranges, numbers)
}

#[inline(never)]
pub fn part_one(input: &str) -> Option<u64> {
    let (ranges, numbers) = parse_input(input);

    let count = numbers
        .filter(|&number| {
            ranges
                .iter()
                .rev()
                .filter(|range| number > *range.start())
                .any(|range| range.contains(&number))
        })
        .count() as u64;

    Some(count)
}

#[inline(never)]
pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();

    let mut ranges = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(parse_range)
        .collect::<Vec<IdRange>>();

    ranges.sort_by(|a, b| a.start().cmp(b.start()));

    let (count, _) = ranges.into_iter().fold((0, 0), |(count, last_end), range| {
        let start = last_end.max(*range.start());
        let end = last_end.max(*range.end());
        (count + end - start + 1, end + 1)
    });

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
