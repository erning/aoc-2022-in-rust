use std::collections::HashSet;
use std::collections::VecDeque;

type HeightMap = Vec<Vec<u8>>;
type Position = (usize, usize);

fn parse_input(input: &str) -> (HeightMap, Position, Position) {
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

    (heightmap, start, end)
}

fn search<F1, F2>(
    heightmap: &HeightMap,
    s: Position,
    is_finish: F1,
    is_movable: F2,
) -> Option<i32>
where
    F1: Fn(Position) -> bool,
    F2: Fn(Position, Position) -> bool,
{
    let mut visited: HashSet<Position> = HashSet::new();
    let mut queue: VecDeque<(i32, Position)> = VecDeque::new();
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
    let (heightmap, start, end) = parse_input(input);
    search(
        &heightmap,
        start,
        |p| p == end,
        |f, t| heightmap[t.1][t.0] as i32 - heightmap[f.1][f.0] as i32 <= 1,
    )
    .unwrap()
}

pub fn part_two(input: &str) -> i32 {
    let (heightmap, _, end) = parse_input(input);
    search(
        &heightmap,
        end,
        |p| heightmap[p.1][p.0] == b'a',
        |f, t| heightmap[f.1][f.0] as i32 - heightmap[t.1][t.0] as i32 <= 1,
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
