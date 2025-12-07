advent_of_code::solution!(7);

fn solve_safe(input: &str) -> (u64, u64) {
    // one of 2 rows is guaranteed to be empty
    let mut lines = input.lines().step_by(2);
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
                // input data is guaranteed to not have ^ at the edges
                b'^' => {
                    next_beams[col - 1] += beam;
                    next_beams[col + 1] += beam;
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

fn solve_safe_col_skip(input: &str) -> (u64, u64) {
    // one of 2 rows is guaranteed to be empty
    let mut lines = input.lines().step_by(2);
    let first_line = lines.next().unwrap();
    let width = first_line.len();
    let start = first_line.find('S').unwrap();

    let mut splits = 0;
    let mut beams = vec![0; width];
    let mut next_beams = vec![0; width];
    beams[start] = 1;

    let mut col_skip = start as isize - 1;
    while let Some(line) = lines.next() {
        let line = line.as_bytes();
        for (col, &beam) in beams
            .iter()
            .enumerate()
            .skip(col_skip as usize)
            .filter(|&(_, &beam)| beam > 0)
        {
            match line[col] {
                // input data is guaranteed to not have ^ at the edges
                b'^' => {
                    next_beams[col - 1] += beam;
                    next_beams[col + 1] += beam;
                    splits += 1;
                }
                b'.' => {
                    next_beams[col] += beam;
                }
                _ => unreachable!(),
            }
        }

        col_skip -= 1;
        (beams, next_beams) = (next_beams, beams);
        next_beams.fill(0);
    }

    let timelines = beams.iter().sum::<u64>();
    (splits, timelines)
}

#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe fn solve_single_pass(input: &str) -> (usize, usize) {
    let bytes = input.as_bytes();
    let len = bytes.len();
    let mut idx: usize = 0;
    let mut start: usize = 0;
    loop {
        idx += 1;
        match *bytes.get_unchecked(idx) {
            b'S' => start = idx,
            b'\n' => break,
            _ => {}
        }
    }

    let width = idx;
    let mut splits: usize = 0;
    let mut beams = vec![0_usize; width];
    let mut next_beams = vec![0_usize; width];

    let mut col = start;
    *beams.get_unchecked_mut(col) = 1;
    idx += start + 1;
    // skip row 2
    idx += width + 1;

    while idx < len {
        let c = *bytes.get_unchecked(idx);
        idx += 1;

        if c == b'\n' {
            col = 0;
            idx += width + 1 /* one row is guaranteed to be empty */;
            std::mem::swap(&mut beams, &mut next_beams);
            next_beams.fill(0);
            continue;
        }

        let beam = *beams.get_unchecked(col);
        if beam != 0 {
            match c {
                b'^' => {
                    // input data is guaranteed to not have ^ at the edges
                    *next_beams.get_unchecked_mut(col - 1) += beam;
                    *next_beams.get_unchecked_mut(col + 1) += beam;
                    splits += 1;
                }
                b'.' => {
                    *next_beams.get_unchecked_mut(col) += beam;
                }
                _ => (),
            }
        }
        col += 1;
    }

    let timelines = beams.iter().sum::<usize>();

    (splits, timelines)
}

#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe fn solve_bounded_tree_single_pass(input: &str) -> (usize, usize) {
    let bytes = input.as_bytes();
    let len = bytes.len();
    let mut idx: usize = 0;
    let mut start: usize = 0;
    loop {
        idx += 1;
        match *bytes.get_unchecked(idx) {
            b'S' => start = idx,
            b'\n' => break,
            _ => {}
        }
    }

    let width = idx;
    let mut splits: usize = 0;
    let mut beams = vec![0_usize; width];
    let mut next_beams = vec![0_usize; width];

    let mut col = start;
    *beams.get_unchecked_mut(col) = 1;
    idx += start + 1;
    // skip row 2
    idx += width + 1;

    let mut col_skip: isize = start as isize - 1;

    while idx < len {
        let c = *bytes.get_unchecked(idx);
        idx += 1;

        if c == b'\n' {
            idx += width + 1 /* one row is guaranteed to be empty */;
            idx += col_skip as usize;
            col = col_skip as usize;
            col_skip -= 1;
            std::mem::swap(&mut beams, &mut next_beams);
            next_beams.fill(0);
            continue;
        }

        let beam = *beams.get_unchecked(col);
        if beam != 0 {
            match c {
                b'^' => {
                    // input data is guaranteed to not have ^ at the edges
                    *next_beams.get_unchecked_mut(col - 1) += beam;
                    *next_beams.get_unchecked_mut(col + 1) += beam;
                    splits += 1;
                }
                b'.' => {
                    *next_beams.get_unchecked_mut(col) += beam;
                }
                _ => (),
            }
        }
        col += 1;
    }

    let timelines = beams.iter().sum::<usize>();

    (splits, timelines)
}

#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe fn solve_bounded_tree_single_pass_raw_ptr(input: &str) -> (usize, usize) {
    let len = input.len();
    let init_ptr = input.as_ptr();

    let mut ptr = init_ptr;
    let stop = init_ptr.add(len);
    let mut col: usize = 0;
    loop {
        ptr = ptr.add(1);
        match *ptr {
            b'S' => {
                col = ptr as usize - init_ptr as usize;
            }
            b'\n' => break,
            _ => {}
        }
    }

    let width = ptr as usize - init_ptr as usize;
    let mut splits: usize = 0;
    let mut beams = vec![0_usize; width];
    let mut next_beams = vec![0_usize; width];

    *beams.get_unchecked_mut(col) = 1;
    let mut col_skip: isize = col as isize - 1;
    ptr = ptr.wrapping_add(1 /* skip \n */);
    ptr = ptr.wrapping_add(width + 1 /* skip row 2 */);
    ptr = ptr.wrapping_add(col_skip as usize);

    while ptr < stop {
        ptr = ptr.wrapping_add(1);
        let c = *ptr;

        if c == b'\n' {
            ptr = ptr.wrapping_add(width + 1 /* one row is guaranteed to be empty */);
            ptr = ptr.wrapping_add(col_skip as usize);
            col = col_skip as usize;
            col_skip -= 1;
            std::mem::swap(&mut beams, &mut next_beams);
            next_beams.fill(0);
            continue;
        }

        let beam = *beams.get_unchecked(col);
        if beam != 0 {
            match c {
                b'^' => {
                    // input data is guaranteed to not have ^ at the edges
                    *next_beams.get_unchecked_mut(col - 1) += beam;
                    *next_beams.get_unchecked_mut(col + 1) += beam;
                    splits += 1;
                }
                b'.' => {
                    *next_beams.get_unchecked_mut(col) += beam;
                }
                _ => (),
            }
        }
        col += 1;
    }

    let timelines = beams.iter().sum::<usize>();

    (splits, timelines)
}

#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe fn solve_bounded_tree_single_pass_one_beams_array(input: &str) -> (usize, usize) {
    let bytes = input.as_bytes();
    let len = bytes.len();
    let mut idx: usize = 0;
    let mut start: usize = 0;
    loop {
        idx += 1;
        match *bytes.get_unchecked(idx) {
            b'S' => start = idx,
            b'\n' => break,
            _ => {}
        }
    }

    let width = idx;
    let mut splits: usize = 0;
    // build beams so that idx also indexes into the beams array
    // and we don't need to remember "col"
    let mut beams = vec![0_usize; len / 2];

    idx += 1; // skip \n
    idx += width + 1; // skip 1 row
    idx += start; // skip to start 
    let mut beam_idx = idx % (width + 1);
    *beams.get_unchecked_mut(beam_idx) = 1;

    let mut col_skip: isize = start as isize - 1;

    while idx < len {
        let c = *bytes.get_unchecked(idx);
        idx += 1;

        if c == b'\n' {
            idx += width + 1 /* one row is guaranteed to be empty */;
            idx += col_skip as usize;
            beam_idx += col_skip as usize;
            beam_idx += 1;
            col_skip -= 1;
            continue;
        }

        let beam = *beams.get_unchecked(beam_idx);
        beam_idx += 1;

        if beam != 0 {
            match c {
                b'^' => {
                    // input data is guaranteed to not have ^ at the edges
                    *beams.get_unchecked_mut(beam_idx + width - 1) += beam;
                    *beams.get_unchecked_mut(beam_idx + width + 1) += beam;
                    splits += 1;
                }
                b'.' => {
                    *beams.get_unchecked_mut(beam_idx + width) += beam;
                }
                _ => (),
            }
        }
    }

    let timelines = beams[(beams.len() - width - 1)..].iter().sum::<usize>();

    (splits, timelines)
}

#[inline(never)]
pub fn part_one(input: &str) -> Option<u64> {
    let (splits, _) = unsafe { solve_bounded_tree_single_pass(input) };
    Some(splits as u64)
}

#[inline(never)]
pub fn part_two(input: &str) -> Option<u64> {
    let (_, timelines) = unsafe { solve_bounded_tree_single_pass(input) };
    Some(timelines as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe() {
        let result = solve_safe(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, (21, 40));
    }

    #[test]
    fn test_safe_col_skip() {
        let result = solve_safe_col_skip(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, (21, 40));
    }

    #[test]
    fn test_single_pass() {
        let result =
            unsafe { solve_single_pass(&advent_of_code::template::read_file("examples", DAY)) };
        assert_eq!(result, (21, 40));
    }

    #[test]
    fn test_bounded_tree_single_pass() {
        let result = unsafe {
            solve_bounded_tree_single_pass(&advent_of_code::template::read_file("examples", DAY))
        };
        assert_eq!(result, (21, 40));
    }

    #[test]
    fn test_bounded_tree_single_pass_one_beams_array() {
        let result = unsafe {
            solve_bounded_tree_single_pass_one_beams_array(&advent_of_code::template::read_file(
                "examples", DAY,
            ))
        };
        assert_eq!(result, (21, 40));
    }

    #[test]
    fn test_bounded_tree_single_pass_raw_ptr() {
        let result = unsafe {
            solve_bounded_tree_single_pass_raw_ptr(&advent_of_code::template::read_file(
                "examples", DAY,
            ))
        };
        assert_eq!(result, (21, 40));
    }
}
