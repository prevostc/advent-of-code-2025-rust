advent_of_code::solution!(7);

fn solve(input: &str) -> (u64, u64) {
    let mut lines = input.lines();
    let first_line = lines.next().unwrap();
    let width = first_line.len();
    let start = first_line.find('S').unwrap();

    let mut splits = 0;
    let mut beams = vec![0; width];
    let mut next_beams = vec![0; width];
    beams[start] = 1;

    while let Some(line) = lines.next() {
        let line = line.as_bytes();
        for (col, &beam) in beams.iter().enumerate().filter(|&(_, &beam)| beam > 0) {
            match line[col] {
                b'^' => {
                    if col > 0 {
                        next_beams[col - 1] += beam;
                    }
                    if col + 1 < width {
                        next_beams[col + 1] += beam;
                    }
                    splits += 1;
                }
                b'.' => {
                    next_beams[col] += beam;
                }
                _ => unreachable!(),
            }
        }
        (beams, next_beams) = (next_beams, beams);
        next_beams.fill(0);
    }

    let timelines = beams.iter().sum::<u64>();
    (splits, timelines)
}

#[inline(never)]
pub fn part_one(input: &str) -> Option<u64> {
    let (splits, _) = solve(input);
    Some(splits)
}

#[inline(never)]
pub fn part_two(input: &str) -> Option<u64> {
    let (_, timelines) = solve(input);
    Some(timelines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
