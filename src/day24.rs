use std::collections::BinaryHeap;
use std::collections::HashSet;

type Position = (usize, usize); // (x,y)
type Blizzard = (Position, usize); // (x, y), direction
const DIRS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn parse_input(input: &str) -> (Vec<Blizzard>, usize, usize) {
    let blizzards: Vec<Blizzard> = input
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars()
                .enumerate()
                .filter_map(|(x, c)| match c {
                    '>' => Some(((x, y), 0)),
                    'v' => Some(((x, y), 1)),
                    '<' => Some(((x, y), 2)),
                    '^' => Some(((x, y), 3)),
                    _ => None,
                })
                .collect::<Vec<Blizzard>>()
        })
        .collect();

    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    (blizzards, width, height)
}

fn timed_blizzards(
    mut blizzards: Vec<Blizzard>,
    width: usize,
    height: usize,
) -> Vec<HashSet<Position>> {
    (0..(width - 2) * (height - 2))
        .map(|_| {
            let v = HashSet::from_iter(blizzards.iter().map(|&(p, _)| p));
            // next blizzards
            for ((x, y), d) in blizzards.iter_mut() {
                let (dx, dy) = DIRS[*d];
                *x = (*x as i32 + dx) as usize;
                if *x == 0 {
                    *x = width - 2;
                } else if *x == width - 1 {
                    *x = 1;
                }
                *y = (*y as i32 + dy) as usize;
                if *y == 0 {
                    *y = height - 2;
                } else if *y == height - 1 {
                    *y = 1;
                }
            }
            v
        })
        .collect()
}

fn next_time(
    t: usize,
    p0: Position,
    p1: Position,
    timed: &[HashSet<Position>],
) -> Vec<usize> {
    let mut times: Vec<usize> = Vec::new();
    let n = timed.len();
    for i in 1..timed.len() {
        let t0 = (t + i - 1) % n;
        if timed[t0].contains(&p0) {
            break;
        }
        let t1 = (t + i) % n;
        if !timed[t1].contains(&p1) {
            times.push(t + i);
        }
    }
    times
}

fn distance(a: Position, b: Position) -> usize {
    ((a.0 as i32 - b.0 as i32).abs() + (a.1 as i32 - b.1 as i32).abs())
        as usize
}

fn search(
    start: Position,
    time: usize,
    target: Position,
    width: usize,
    height: usize,
    timed: &[HashSet<Position>],
) -> usize {
    let heuristic = |p: Position| -(distance(p, target) as i32);

    let mut visited: HashSet<(Position, usize)> = HashSet::new();
    let mut queue: BinaryHeap<(i32, Position, usize)> = BinaryHeap::new();
    queue.push((heuristic(start), start, time));

    while let Some((_, (x, y), t)) = queue.pop() {
        if (x, y) == target {
            return t;
        }

        if !visited.insert(((x, y), t)) {
            continue;
        }

        for (dx, dy) in DIRS {
            let (nx, ny) = (x as i32 + dx, y as i32 + dy);
            if nx <= 0
                || nx >= width as i32 - 1
                || ny <= 0
                || ny >= height as i32 - 1
            {
                continue;
            }
            let p = (nx as usize, ny as usize);
            for nt in next_time(t, (x, y), p, timed) {
                queue.push((heuristic(p) - (nt + t) as i32, p, nt));
            }
        }
    }

    0
}

pub fn part_one(input: &str) -> usize {
    let (blizzards, width, height) = parse_input(input);
    let timed = timed_blizzards(blizzards, width, height);

    let start = (1, 0);
    let target = (width - 2, height - 2);

    search(start, 0, target, width, height, &timed) + 1
}

pub fn part_two(input: &str) -> usize {
    let (blizzards, width, height) = parse_input(input);
    let timed = timed_blizzards(blizzards, width, height);

    let a = (1, 0);
    let b = (width - 2, height - 2);
    let c = (width - 2, height - 1);
    let d = (1, 1);

    let t = search(a, 0, b, width, height, &timed) + 1;
    let t = search(c, t, d, width, height, &timed) + 1;
    search(a, t, b, width, height, &timed) + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(24);
        assert_eq!(part_one(&input), 18);
        assert_eq!(part_two(&input), 54);
    }
}
