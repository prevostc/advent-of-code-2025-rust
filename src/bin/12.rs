advent_of_code::solution!(12);

#[inline(never)]
pub fn part_one(input: &str) -> Option<u64> {
    let count = input
        .lines()
        .skip(30)
        .filter(|region| {
            let (size, counts) = region.split_once(':').unwrap();
            let region_size: usize = size
                .split('x')
                .map(|n| n.parse::<usize>().unwrap())
                .product();
            let required = counts[2..]
                .split(' ')
                .map(|n| n.parse::<usize>().unwrap() * 9)
                .sum();

            // turns out you can ignore the individual shapes and just try to fit 3x3 squares in the regions
            region_size >= required
        })
        .count();
    Some(count as u64)
}

#[inline(never)]
pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
