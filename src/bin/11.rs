use std::collections::VecDeque;

advent_of_code::solution!(11);

#[inline(always)]
fn hash(node: &str) -> usize {
    debug_assert_eq!(node.len(), 3);
    let node = node.as_bytes();
    (((node[0] - b'a') as usize) << 10
        | ((node[1] - b'a') as usize) << 5
        | ((node[2] - b'a') as usize)) as usize
}

#[allow(unused)]
#[inline(always)]
fn node_id_to_str(node: usize) -> String {
    let mut s = String::with_capacity(3);
    s.push((((node >> 10) & 0b11111) as u8 + b'a') as char);
    s.push((((node >> 5) & 0b11111) as u8 + b'a') as char);
    s.push(((node & 0b11111) as u8 + b'a') as char);
    s
}

fn parse_input(input: &str) -> (Vec<usize>, Vec<Vec<usize>>) {
    let mut nodes = Vec::with_capacity(1 << 15);
    let mut edges = vec![Vec::with_capacity(10); 1 << 15]; // wasteful in memory but faster than FxHashMap

    for line in input.lines() {
        let (n, e) = line.split_once(':').unwrap();
        let ni = hash(n);
        nodes.push(ni);

        debug_assert!(edges[ni].is_empty());
        edges[ni].extend(e.split_whitespace().map(hash));
    }

    (nodes, edges)
}

fn topological_sort(nodes: &[usize], edges: &[Vec<usize>]) -> Vec<usize> {
    // https://www.geeksforgeeks.org/dsa/topological-sorting-indegree-based-solution/
    let mut in_degree = [0usize; 1 << 15];
    for &node in nodes.iter() {
        for &neighbor in &edges[node] {
            in_degree[neighbor] += 1;
        }
    }

    let mut q: VecDeque<_> = nodes
        .iter()
        .copied()
        .filter(|&n| in_degree[n] == 0)
        .collect();

    // Kahn's algorithm
    let mut ordered = Vec::with_capacity(nodes.len());
    while let Some(node) = q.pop_front() {
        ordered.push(node);
        for &neighbor in &edges[node] {
            in_degree[neighbor] -= 1;
            if in_degree[neighbor] == 0 {
                q.push_back(neighbor);
            }
        }
    }

    ordered
}

fn count_ways(ordered_nodes: &[usize], edges: &[Vec<usize>], start: usize, end: usize) -> u64 {
    let mut ways = vec![0; 1 << 15];
    ways[start] = 1;

    for node in ordered_nodes.iter().skip_while(|&&n| n != start) {
        for &neighbor in &edges[*node] {
            ways[neighbor] += ways[*node];
        }
    }
    ways[end] as u64
}

#[inline(never)]
pub fn part_one(input: &str) -> Option<u64> {
    let (nodes, edges) = parse_input(input);

    let ordered = topological_sort(&nodes, &edges);

    Some(count_ways(&ordered, &edges, hash("you"), hash("out")))
}

#[inline(never)]
pub fn part_two(input: &str) -> Option<u64> {
    let (nodes, edges) = parse_input(input);
    let ordered = topological_sort(&nodes, &edges);

    let [svr, dac, fft, out] = [hash("svr"), hash("dac"), hash("fft"), hash("out")];
    let ways = |from: usize, to: usize| count_ways(&ordered, &edges, from, to);

    match (ways(dac, fft), ways(fft, dac)) {
        (dac_fft, 0) => return Some(ways(svr, dac) * dac_fft * ways(fft, out)),
        (0, fft_dac) => return Some(ways(svr, fft) * fft_dac * ways(dac, out)),
        (_, _) => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(
            "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
",
        );
        assert_eq!(result, Some(2));
    }
}
