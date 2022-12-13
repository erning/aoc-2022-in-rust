use std::collections::HashSet;
use std::collections::VecDeque;

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
fn search<F1, F2>(
    heightmap: &Vec<Vec<u8>>,
    s: (usize, usize),
    is_finish: F1,
    is_movable: F2,
) -> Option<i32>
where
    F1: Fn((usize, usize)) -> bool,
    F2: Fn((usize, usize), (usize, usize)) -> bool,
{
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut queue: VecDeque<(i32, (usize, usize))> = VecDeque::new();
    queue.push_back((0, s));
    let h = heightmap.len() as i32;
    let w = heightmap[0].len() as i32;
    while let Some((step, p)) = queue.pop_front() {
        if is_finish(p) {
            return Some(step);
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
            if is_movable(p, np) {
                queue.push_back((step + 1, np));
            }
        }
    }
    None
}

pub fn part_one(input: &str) -> i32 {
    let parsed = parse_input(input);
    search(
        &parsed.heightmap,
        parsed.start,
        |p| p == parsed.end,
        |f, t| {
            parsed.heightmap[t.1][t.0] as i32
                - parsed.heightmap[f.1][f.0] as i32
                <= 1
        },
    )
    .unwrap()
}

pub fn part_two(input: &str) -> i32 {
    let parsed = parse_input(input);
    search(
        &parsed.heightmap,
        parsed.end,
        |p| parsed.heightmap[p.1][p.0] == b'a',
        |f, t| {
            parsed.heightmap[f.1][f.0] as i32
                - parsed.heightmap[t.1][t.0] as i32
                <= 1
        },
    )
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
