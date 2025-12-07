advent_of_code::solution!(6);
use mygrid::{grid::Grid, point::Point};

#[inline(always)]
fn op_identity(op: char) -> u64 {
    match op {
        '*' => 1,
        '+' => 0,
        _ => unreachable!(),
    }
}

#[inline(always)]
fn op_fold(acc: u64, item: u64, op: char) -> u64 {
    match op {
        '*' => acc * item,
        '+' => acc + item,
        _ => unreachable!(),
    }
}

#[inline(never)]
pub fn part_one(input: &str) -> Option<u64> {
    let mut chars = input.chars();
    let mut data = Vec::with_capacity(1000);
    let mut operations = Vec::with_capacity(1000);
    let mut n = 0;
    while let Some(c) = chars.next() {
        match c {
            '*' | '+' => operations.push(c),
            '0'..='9' => n = n * 10 + c.to_digit(10).unwrap() as u64,
            ' ' | '\n' if n > 0 => {
                data.push(n);
                n = 0
            }
            ' ' | '\n' => (),
            _ => unreachable!(),
        }
    }
    let grid = Grid::from_vec(data, operations.len());
    let total = (0..grid.width)
        .map(|column| {
            let op = operations[column];
            (0..grid.height)
                .map(|row| Point::new_usize(row, column))
                .map(|pos| grid.get_item(pos).unwrap())
                .fold(op_identity(op), |acc, &item| op_fold(acc, item, op))
        })
        .sum();
    Some(total)
}

#[inline(never)]
pub fn part_two(input: &str) -> Option<u64> {
    let mut nums = Vec::with_capacity(1000);
    let mut col_idx = 0;
    let mut chars = input.chars();
    let mut op = '+';

    while let Some(c) = chars.next() {
        match c {
            '*' | '+' => {
                op = c;
                break;
            }
            ' ' if col_idx >= nums.len() => {
                nums.push(0);
            }
            ' ' => (),
            '\n' => {
                col_idx = 0;
                continue;
            }
            '0'..='9' => {
                let n = c.to_digit(10).unwrap() as u64;
                if col_idx < nums.len() {
                    nums[col_idx] = nums[col_idx] * 10 + n;
                } else {
                    nums.push(n);
                }
            }
            _ => unreachable!(),
        }
        col_idx += 1;
    }

    let mut op = op;
    let mut op_idx = 0;
    let mut col = 0;
    let mut total = 0;

    while let Some(c) = chars.next() {
        match c {
            // TODO: dedup code but keep it fast
            '*' | '+' => {
                total += nums[op_idx..col]
                    .iter()
                    .filter(|&x| *x != 0)
                    .fold(op_identity(op), |acc, &x| op_fold(acc, x, op));
                op = c;
                op_idx = col;
            }
            '\n' => {
                total += nums[op_idx..]
                    .iter()
                    .filter(|&x| *x != 0)
                    .fold(op_identity(op), |acc, &x| op_fold(acc, x, op));
                return Some(total);
            }
            ' ' => (),
            _ => unreachable!(),
        }
        col += 1;
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
