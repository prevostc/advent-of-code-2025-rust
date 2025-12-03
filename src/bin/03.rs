advent_of_code::solution!(3);

fn stream_input(input: &str) -> impl Iterator<Item = Vec<u8>> + '_ {
    input.lines().map(|line| {
        line.chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect()
    })
}

fn solve<const N: usize>(line: &[u8]) -> u64 {
    let mut result = 0u64;
    let mut last_idx = 0usize;

    for pos in 0..N {
        let end = line.len() - (N - pos - 1);
        let mut max_digit = 0u8;
        let mut max_idx = last_idx;
        for (i, &digit) in line[last_idx..end].iter().enumerate() {
            if digit > max_digit {
                max_digit = digit;
                max_idx = last_idx + i;
            }
        }

        result = result * 10 + max_digit as u64;
        last_idx = max_idx + 1;
    }

    result
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(stream_input(input).map(|line| solve::<2>(&line)).sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(stream_input(input).map(|line| solve::<12>(&line)).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
