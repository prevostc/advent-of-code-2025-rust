#![feature(slice_partition_dedup)]

use std::collections::VecDeque;

use rayon::iter::{ParallelBridge, ParallelIterator};
use rustc_hash::FxHashSet;
use z3::{Optimize, SatResult, ast::Int};

advent_of_code::solution!(10);

////////////////////////////////////////////////////////////
/// PART 1
////////////////////////////////////////////////////////////

type Lights = u16;

#[inline(always)]
fn parse_schematics(schematics: &str) -> Lights {
    let sch = &schematics[1..(schematics.len() - 1)]; // skip "[" and "]"
    sch.chars()
        .map(|c| match c {
            '.' => 0,
            '#' => 1,
            _ => unreachable!(),
        })
        .enumerate()
        .fold(0_u16, |acc, (i, x)| acc | (x << (i as u64)))
}

#[inline(always)]
fn parse_button_p1(button: &str) -> Lights {
    button[1..(button.len() - 1)] // skip "(" and ")"
        .split(',')
        .map(|n| n.parse::<u64>().unwrap())
        .fold(0_u16, |acc, x| acc | (1 << x as u16))
}

#[inline(always)]
#[allow(unused)]
fn debug_lights(lights: Lights) {
    println!("{:016b}", lights);
}

#[inline(always)]
fn bfs_p1(sch: Lights, buttons: &[Lights]) -> u64 {
    let mut visited = FxHashSet::with_capacity_and_hasher(1000, Default::default());
    let mut q = VecDeque::with_capacity(1000);
    q.push_back((0, 0));
    while let Some((lights, depth)) = q.pop_front() {
        if lights == sch {
            return depth;
        }
        if visited.contains(&lights) {
            continue;
        }
        visited.insert(lights);
        for button in buttons {
            q.push_back((lights ^ button, depth + 1));
        }
    }

    unreachable!("As per the problem statement, there is always a solution");
}

#[inline(never)]
pub fn part_one(input: &str) -> Option<u64> {
    let res = input
        .lines()
        .take_while(|line| !line.is_empty())
        .par_bridge()
        .map(|line| {
            let mut parts = line.split(' ');
            let sch = parse_schematics(parts.next().unwrap());
            let buttons = parts
                .take_while(|b| b.starts_with('('))
                .map(parse_button_p1)
                .collect::<Vec<_>>();

            bfs_p1(sch, &buttons)
        })
        .sum();

    Some(res)
}

////////////////////////////////////////////////////////////
/// PART 2
////////////////////////////////////////////////////////////
///

const MAX_JOLTS: usize = 12;
type Jolts = [u16; MAX_JOLTS];

#[inline(always)]
fn parse_button_p2(button: &str) -> Jolts {
    let mut jolts = [0; MAX_JOLTS];
    for jolt in button[1..(button.len() - 1)].split(',') {
        let idx = jolt.parse::<u8>().unwrap() as usize;
        jolts[idx] = 1;
    }
    jolts
}

#[inline(always)]
fn parse_counters(counter: &str) -> Jolts {
    let mut jolts = [0; MAX_JOLTS];
    for (i, jolt) in counter[1..(counter.len() - 1)].split(',').enumerate() {
        jolts[i] = jolt.parse::<u16>().unwrap();
    }
    jolts
}

#[inline(always)]
fn solve_z3(counters: Jolts, buttons: &[Jolts]) -> u64 {
    // B: inverse of buttons matrix (MAX_JOLT x N_buttons)
    // C: counters column matrix (MAX_JOLT x 1)
    // X: presses matrix (N_buttons x 1)
    // B * X = C

    let nb = buttons.len();
    let opt = Optimize::new();
    let mut x = Vec::with_capacity(nb);

    for (i, button) in buttons.iter().enumerate() {
        let max_presses = button
            .iter()
            .zip(counters.iter())
            .filter(|&(&b, _)| b == 1)
            .map(|(_, &c)| c)
            .min()
            .unwrap_or(0);

        let xi = Int::fresh_const(&format!("x_{}", i));
        opt.assert(&xi.ge(0));
        opt.assert(&xi.le(max_presses));
        x.push(xi);
    }

    // express matrix multiplication B * X = C
    // B and C are known, X is unknown
    for (i, counter) in counters.iter().enumerate() {
        let mut expr = Int::from(0);
        for (j, button) in buttons.iter().enumerate() {
            let bi = button[i];
            let xi = &x[j];

            if bi == 0 {
                continue;
            }

            expr += xi * bi;
        }
        opt.assert(&expr.eq(*counter));
    }

    let total = x.iter().fold(Int::from(0), |acc, xi| acc + xi);
    opt.minimize(&total);

    // solve
    match opt.check(&[]) {
        SatResult::Sat => {
            let model = opt.get_model().unwrap();
            let min_total = model.eval(&total, true).unwrap().as_i64().unwrap() as u64;
            min_total
        }
        _ => panic!("unsat or unknown"),
    }
}

#[inline(never)]
pub fn part_two(input: &str) -> Option<u64> {
    let res = input
        .lines()
        .take_while(|line| !line.is_empty())
        .par_bridge()
        .map(|line| {
            let mut parts = line.split(' ').skip(1).collect::<Vec<_>>(); // skip lights
            let counters = parse_counters(parts.remove(parts.len() - 1));
            let buttons = parts.into_iter().map(parse_button_p2).collect::<Vec<_>>();

            let res = solve_z3(counters, &buttons);
            res
        })
        .sum();

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }
}
