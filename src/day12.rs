use std::collections::{BinaryHeap, HashSet};

#[derive(Debug)]
struct ParsedInput {
    heightmap: Vec<Vec<u8>>,
    start: (usize, usize),
    end: (usize, usize),
}

fn parse_input(input: &str) -> ParsedInput {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let heightmap = input
        .lines()
        .enumerate()
        .map(|(y, v)| {
            v.bytes()
                .enumerate()
                .map(|(x, v)| match v {
                    b'S' => {
                        start = (x, y);
                        b'a'
                    }
                    b'E' => {
                        end = (x, y);
                        b'z'
                    }
                    _ => v,
                })
                .collect()
        })
        .collect();
    ParsedInput {
        heightmap,
        start,
        end,
    }
}

fn dijkstra(
    heightmap: &Vec<Vec<u8>>,
    e: (usize, usize),
    visited: &mut HashSet<(usize, usize)>,
    queue: &mut BinaryHeap<(i32, (usize, usize))>,
) -> Option<i32> {
    let h = heightmap.len() as i32;
    let w = heightmap[0].len() as i32;
    while let Some((step, p)) = queue.pop() {
        if p == e {
            return Some(-step);
        }
        if !visited.insert(p) {
            continue;
        }
        for (dx, dy) in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
            let (nx, ny) = (p.0 as i32 + dx, p.1 as i32 + dy);
            if nx < 0 || nx >= w || ny < 0 || ny >= h {
                continue;
            }
            let np = (nx as usize, ny as usize);
            if visited.contains(&np) {
                continue;
            }
            if heightmap[np.1][np.0] as i32 - heightmap[p.1][p.0] as i32 > 1 {
                continue;
            }
            queue.push((step - 1, np));
        }
    }
    None
}

pub fn part_one(input: &str) -> i32 {
    let parsed = parse_input(input);
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut queue: BinaryHeap<(i32, (usize, usize))> = BinaryHeap::new();
    queue.push((0, parsed.start));
    dijkstra(&parsed.heightmap, parsed.end, &mut visited, &mut queue).unwrap()
}

pub fn part_two(input: &str) -> i32 {
    let parsed = parse_input(input);
    let mut starts: Vec<(usize, usize)> = Vec::new();
    for (y, row) in parsed.heightmap.iter().enumerate() {
        for (x, v) in row.iter().enumerate() {
            if v == &b'a' {
                starts.push((x, y))
            }
        }
    }
    starts
        .into_iter()
        .filter_map(|s| {
            let mut visited: HashSet<(usize, usize)> = HashSet::new();
            let mut queue: BinaryHeap<(i32, (usize, usize))> =
                BinaryHeap::new();
            queue.push((0, s));
            dijkstra(&parsed.heightmap, parsed.end, &mut visited, &mut queue)
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(12);
        assert_eq!(part_one(&input), 31);
        assert_eq!(part_two(&input), 29);
    }
}
