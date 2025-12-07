advent_of_code::solution!(1);

fn stream_input(input: &str) -> impl Iterator<Item = isize> {
    input
        .lines()
        .map(|line| {
            let direction = if line.starts_with('L') { -1 } else { 1 };
            direction * line[1..].parse::<isize>().expect("Invalid number")
        })
        .filter(|&value| value != 0)
}

#[inline(never)]
pub fn part_one(input: &str) -> Option<u64> {
    let (_, res) = stream_input(input).fold((50, 0), |(c, r), v| {
        let n = (c + v % 100).rem_euclid(100);
        (n, r + (n == 0) as u64)
    });
    Some(res)
}

#[inline(never)]
pub fn part_two(input: &str) -> Option<u64> {
    let (_, res) = stream_input(input).fold((50, 0), |(c, r), v| {
        let n = (c + v % 100).rem_euclid(100);
        let z = ((v < 0 && n > c && c != 0) || (v > 0 && n < c && c != 0) || n == 0) as u64;
        (n, r + (v.abs() / 100) as u64 + z)
    });
    Some(res)
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
        assert_eq!(result, Some(6));
    }
}
