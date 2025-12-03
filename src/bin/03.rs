use std::cmp::max;

advent_of_code::solution!(3);

fn stream_input(input: &str) -> impl Iterator<Item = Vec<u8>> {
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        })
}

fn solve<const N: usize>(line: Vec<u8>) -> u64 {
    let mut digits = [(0, 0); N]; // (digit value, index in line)
    for i in 0..N {
        let i_idx = line.len() - (N - i);
        digits[i] = (line[i_idx], i_idx);
    }

    for n in 0..N {
        // find max digit in the available range
        let (mut n_max, n_idx) = digits[n];
        let min_idx = max(n, if n > 0 { digits[n - 1].1 + 1 } else { 0 });
        let max_idx = n_idx - 1;
        for i in (min_idx..=max_idx).rev() {
            let next_digit = line[i];
            if next_digit >= n_max {
                digits[n] = (next_digit, i);
                n_max = next_digit;
            }
        }
    }

    let res = digits
        .into_iter()
        .fold(0, |acc, (d, _)| acc * 10 + d as u64);
    res
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(stream_input(input).map(|line| solve::<2>(line)).sum())
}
pub fn part_two(input: &str) -> Option<u64> {
    Some(stream_input(input).map(|line| solve::<12>(line)).sum())
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
