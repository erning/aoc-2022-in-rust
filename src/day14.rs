use std::collections::HashSet;

fn parse_input(input: &str) -> Vec<Vec<(i32, i32)>> {
    input
        .lines()
        .map(|v| {
            v.split("->")
                .map(|v| v.trim())
                .map(|v| {
                    let mut iter = v.split(',');
                    let x = iter.next().unwrap();
                    let y = iter.next().unwrap();
                    (x.parse().unwrap(), y.parse().unwrap())
                })
                .collect::<Vec<(i32, i32)>>()
        })
        .collect::<Vec<Vec<(i32, i32)>>>()
}

fn tiles(rock_paths: &[Vec<(i32, i32)>]) -> HashSet<(i32, i32)> {
    let mut tiles: HashSet<(i32, i32)> = HashSet::new();
    for path in rock_paths.iter() {
        let (mut x0, mut y0) = path[0];
        for &(x, y) in path[1..].iter() {
            if x == x0 {
                for i in y.min(y0)..=y.max(y0) {
                    tiles.insert((x, i));
                }
            }
            if y == y0 {
                for i in x.min(x0)..=x.max(x0) {
                    tiles.insert((i, y));
                }
            }
            (x0, y0) = (x, y)
        }
    }
    tiles
}

fn bound(rock_paths: &[Vec<(i32, i32)>]) -> (i32, i32, i32, i32) {
    let (mut x1, mut y1) = (i32::MAX, i32::MAX);
    let (mut x2, mut y2) = (i32::MIN, i32::MIN);
    for path in rock_paths.iter() {
        for &(x, y) in path {
            x1 = x1.min(x);
            y1 = y1.min(y);
            x2 = x2.max(x);
            y2 = y2.max(y);
        }
    }
    (x1, y1, x2, y2)
}

pub fn part_one(input: &str) -> i32 {
    fn pour_sand(
        mut x: i32,
        mut y: i32,
        tiles: &mut HashSet<(i32, i32)>,
        depth: i32,
    ) -> Option<(i32, i32)> {
        while y < depth {
            if !tiles.contains(&(x, y + 1)) {
                y += 1;
                continue;
            }
            if !tiles.contains(&(x - 1, y + 1)) {
                x -= 1;
                y += 1;
                continue;
            }
            if !tiles.contains(&(x + 1, y + 1)) {
                x += 1;
                y += 1;
                continue;
            }
            tiles.insert((x, y));
            return Some((x, y));
        }
        None
    }

    let rock_paths = parse_input(input);
    let mut tiles = tiles(&rock_paths);
    let (_, _, _, depth) = bound(&rock_paths);
    let mut count = 0;
    while pour_sand(500, 0, &mut tiles, depth).is_some() {
        count += 1
    }
    count
}

pub fn part_two(input: &str) -> i32 {
    fn pour_sand(
        mut x: i32,
        mut y: i32,
        tiles: &mut HashSet<(i32, i32)>,
        depth: i32,
    ) -> Option<(i32, i32)> {
        while y < depth + 1 {
            if !tiles.contains(&(x, y + 1)) {
                y += 1;
                continue;
            }
            if !tiles.contains(&(x - 1, y + 1)) {
                x -= 1;
                y += 1;
                continue;
            }
            if !tiles.contains(&(x + 1, y + 1)) {
                x += 1;
                y += 1;
                continue;
            }
            break;
        }
        if tiles.insert((x, y)) {
            Some((x, y))
        } else {
            None
        }
    }

    let rock_paths = parse_input(input);
    let mut tiles = tiles(&rock_paths);
    let (_, _, _, depth) = bound(&rock_paths);
    let mut count = 0;
    while pour_sand(500, 0, &mut tiles, depth).is_some() {
        count += 1
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(14);
        assert_eq!(part_one(&input), 24);
        assert_eq!(part_two(&input), 93);
    }
}
