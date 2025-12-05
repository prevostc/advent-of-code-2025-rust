use std::ops::RangeInclusive;

advent_of_code::solution!(5);

#[derive(Debug, Clone, PartialEq, Eq)]
struct IdRange(RangeInclusive<u64>);
impl IdRange {
    fn from_str(s: &str) -> Self {
        let (start, end) = s.split_once('-').unwrap();
        IdRange(start.parse::<u64>().unwrap()..=end.parse::<u64>().unwrap())
    }

    fn overlaps(&self, other: &IdRange) -> bool {
        !(self.0.end() < other.0.start() || self.0.start() > other.0.end())
    }

    fn merge(&self, other: &IdRange) -> IdRange {
        IdRange((*self.0.start().min(other.0.start()))..=(*self.0.end().max(other.0.end())))
    }

    fn contains(&self, number: u64) -> bool {
        self.0.contains(&number)
    }

    fn is_less_than(&self, other: &IdRange) -> bool {
        self.0.start() < other.0.start()
    }
}

#[derive(Debug)]
struct RangeSet(Vec<IdRange>);

impl RangeSet {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn merge_with(&mut self, range: IdRange) {
        let idx = self
            .0
            .iter()
            .position(|r| r.overlaps(&range) || range.is_less_than(r));

        match idx {
            Some(idx) if self.0[idx].overlaps(&range) => {
                self.0[idx] = self.0[idx].merge(&range);
                while idx + 1 < self.0.len() && self.0[idx].overlaps(&self.0[idx + 1]) {
                    self.0[idx] = self.0[idx].merge(&self.0[idx + 1]);
                    self.0.remove(idx + 1);
                }
            }
            Some(idx) => self.0.insert(idx, range),
            None => self.0.push(range),
        }
    }

    fn len(&self) -> u64 {
        self.0.iter().map(|r| r.0.clone().count()).sum::<usize>() as u64
    }
}

impl RangeSet {
    fn collect<I: IntoIterator<Item = IdRange>>(iter: I) -> Self {
        let mut set = RangeSet::new();
        for range in iter {
            set.merge_with(range);
        }
        set
    }
}

fn parse_input(input: &str) -> (Vec<IdRange>, impl Iterator<Item = u64> + '_) {
    let mut lines = input.lines();

    let ranges: Vec<IdRange> = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(IdRange::from_str)
        .collect();

    let numbers = lines
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<u64>().unwrap());

    (ranges, numbers)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (ranges, numbers) = parse_input(input);

    let res = numbers
        .filter(|&number| ranges.iter().any(|range| range.contains(number)))
        .count() as u64;

    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (ranges, _) = parse_input(input);
    let range_set = RangeSet::collect(ranges);
    Some(range_set.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_range_parses_and_contains() {
        let range = IdRange::from_str("5-10");
        assert!(range.contains(5));
        assert!(range.contains(7));
        assert!(range.contains(10));
        assert!(!range.contains(4));
        assert!(!range.contains(11));
    }

    #[test]
    fn id_range_detects_overlap() {
        let a = IdRange::from_str("3-8");
        let b = IdRange::from_str("6-12");
        let c = IdRange::from_str("9-15");

        assert!(a.overlaps(&b), "ranges share values");
        assert!(b.overlaps(&c), "ranges share boundary value");
        assert!(!a.overlaps(&c), "transitive overlap through shared values");
    }

    #[test]
    fn id_range_detects_not_overlapping() {
        let a = IdRange::from_str("1-3");
        let b = IdRange::from_str("5-7");

        assert!(!a.overlaps(&b), "ranges do not share values");
        assert!(!b.overlaps(&a), "ranges do not share values");
    }

    fn bounds(set: &RangeSet) -> Vec<(u64, u64)> {
        set.0.iter().map(|r| (*r.0.start(), *r.0.end())).collect()
    }

    #[test]
    fn merge_with_inserts_disjoint_sorted() {
        let mut set = RangeSet::new();
        set.merge_with(IdRange::from_str("5-7"));
        set.merge_with(IdRange::from_str("1-3"));

        assert_eq!(bounds(&set), vec![(1, 3), (5, 7)]);
        assert_eq!(set.len(), 6);
    }

    #[test]
    fn merge_with_merges_same_start() {
        let mut set = RangeSet::new();
        set.merge_with(IdRange::from_str("2-5"));
        set.merge_with(IdRange::from_str("2-8"));

        assert_eq!(bounds(&set), vec![(2, 8)]);
        assert_eq!(set.len(), 7);
    }

    #[test]
    fn merge_with_merges_overlapping_previous_range() {
        let mut set = RangeSet::new();
        set.merge_with(IdRange::from_str("5-8"));
        set.merge_with(IdRange::from_str("2-6"));

        assert_eq!(bounds(&set), vec![(2, 8)]);
        assert_eq!(set.len(), 7);
    }

    #[test]
    fn merge_with_larger_range_maintains_order() {
        let mut set = RangeSet::new();
        set.merge_with(IdRange::from_str("10-14"));
        set.merge_with(IdRange::from_str("1-3"));

        assert_eq!(bounds(&set), vec![(1, 3), (10, 14)]);
        assert_eq!(set.len(), 8);

        let mut set = RangeSet::new();
        set.merge_with(IdRange::from_str("1-3"));
        set.merge_with(IdRange::from_str("10-14"));

        assert_eq!(bounds(&set), vec![(1, 3), (10, 14)]);
        assert_eq!(set.len(), 8);
    }

    #[test]
    fn merge_combines_existing_ranges() {
        let mut set = RangeSet::new();
        set.merge_with(IdRange::from_str("1-3"));
        set.merge_with(IdRange::from_str("5-7"));
        set.merge_with(IdRange::from_str("3-5"));

        assert_eq!(bounds(&set), vec![(1, 7)]);
        assert_eq!(set.len(), 7);
    }

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
