advent_of_code::solution!(8);

#[inline(always)]
fn squared_distance(p1: Point, p2: Point) -> i64 {
    (p1.0 - p2.0) * (p1.0 - p2.0) + (p1.1 - p2.1) * (p1.1 - p2.1) + (p1.2 - p2.2) * (p1.2 - p2.2)
}

type Point = (i64, i64, i64);

pub fn parse_input(input: &str) -> (Vec<Point>, Vec<(usize, usize, i64)>) {
    let points: Vec<Point> = input
        .lines()
        .map(|line| {
            let mut r = line.split(',');
            (
                r.next().unwrap().parse::<i64>().unwrap(),
                r.next().unwrap().parse::<i64>().unwrap(),
                r.next().unwrap().parse::<i64>().unwrap(),
            )
        })
        .collect::<Vec<_>>();
    let mut distances = Vec::with_capacity(points.len() * (points.len() - 1));

    for (i, &p1) in points.iter().enumerate() {
        for (j, &p2) in points.iter().enumerate().skip(i + 1) {
            distances.push((i, j, squared_distance(p1, p2)));
        }
    }

    distances.sort_unstable_by_key(|&(.., dst)| dst);
    (points, distances)
}

fn solve_p1<const CONNECTIONS: usize, const COUNT_THRESHOLD: usize>(input: &str) -> u64 {
    let (_, distances) = parse_input(input);

    let mut next_circuit_idx = 0;
    let mut circuits = vec![None; input.len()];

    for &(idx, closest_idx, _) in distances[..CONNECTIONS].iter() {
        match (circuits[idx], circuits[closest_idx]) {
            (Some(circuit_idx), Some(other_circuit_idx)) if circuit_idx == other_circuit_idx => {
                continue;
            }
            (Some(circuit_idx), Some(other_circuit_idx)) => {
                for j in 0..input.len() {
                    if circuits[j] == Some(other_circuit_idx) {
                        circuits[j] = Some(circuit_idx);
                    }
                }
            }
            (None, None) => {
                circuits[idx] = Some(next_circuit_idx);
                circuits[closest_idx] = Some(next_circuit_idx);
                next_circuit_idx += 1;
            }
            (Some(circuit_idx), None) => {
                circuits[closest_idx] = Some(circuit_idx);
            }
            (None, Some(circuit_idx)) => {
                circuits[idx] = Some(circuit_idx);
            }
        }
    }

    // TODO: compute this while building the circuits
    use std::collections::HashMap;
    let mut counts = HashMap::new();
    for b in circuits.iter().filter_map(|b| *b) {
        *counts.entry(b).or_insert(0u64) += 1;
    }
    let mut counts = counts
        .into_iter()
        .map(|(_, count)| count)
        .collect::<Vec<_>>();
    counts.sort_unstable();

    counts[counts.len() - COUNT_THRESHOLD..]
        .iter()
        .product::<u64>()
}

#[inline(never)]
pub fn part_one(input: &str) -> Option<u64> {
    Some(solve_p1::<1000, 3>(input))
}

fn solve_p2(input: &str) -> u64 {
    let (input, distances) = parse_input(input);

    let mut next_circuit_idx = 0;
    let mut circuits = vec![None; input.len()];
    let mut last_connection = (0, 0);

    for &(idx, closest_idx, _) in distances.iter() {
        match (circuits[idx], circuits[closest_idx]) {
            (Some(circuit_idx), Some(other_circuit_idx)) if circuit_idx == other_circuit_idx => {
                continue;
            }
            (Some(circuit_idx), Some(other_circuit_idx)) => {
                last_connection = (idx, closest_idx);
                for j in 0..input.len() {
                    if circuits[j] == Some(other_circuit_idx) {
                        circuits[j] = Some(circuit_idx);
                    }
                }
            }
            (None, None) => {
                circuits[idx] = Some(next_circuit_idx);
                circuits[closest_idx] = Some(next_circuit_idx);
                next_circuit_idx += 1;
                last_connection = (idx, closest_idx);
            }
            (Some(circuit_idx), None) => {
                circuits[closest_idx] = Some(circuit_idx);
                last_connection = (idx, closest_idx);
            }
            (None, Some(circuit_idx)) => {
                circuits[idx] = Some(circuit_idx);
                last_connection = (idx, closest_idx);
            }
        }
    }

    let (x1, _, _) = input[last_connection.0];
    let (x2, _, _) = input[last_connection.1];
    return (x1 * x2) as u64;
}

#[inline(never)]
pub fn part_two(input: &str) -> Option<u64> {
    Some(solve_p2(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = solve_p1::<10, 3>(&input);
        assert_eq!(result, 40);
    }

    #[test]
    fn test_part_two() {
        let result = solve_p2(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, 25272);
    }
}
