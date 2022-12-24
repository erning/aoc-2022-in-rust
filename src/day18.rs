use std::collections::HashSet;

const DIRS: [(i8, i8, i8); 6] = [
    (1, 0, 0),
    (-1, 0, 0),
    (0, 1, 0),
    (0, -1, 0),
    (0, 0, 1),
    (0, 0, -1),
];

fn parse_input(input: &str) -> Vec<(i8, i8, i8)> {
    input
        .lines()
        .map(|v| {
            v.split(',')
                .map(|v| v.parse::<i8>().unwrap())
                .collect::<Vec<i8>>()
        })
        .map(|v| (v[0], v[1], v[2]))
        .collect()
}

fn surface(cubeset: &HashSet<(i8, i8, i8)>) -> usize {
    let mut n = cubeset.len() * 6;
    for &(x, y, z) in cubeset.iter() {
        for (dx, dy, dz) in DIRS {
            if cubeset.contains(&(x + dx, y + dy, z + dz)) {
                n -= 1;
            }
        }
    }
    n
}

pub fn part_one(input: &str) -> usize {
    let lava = parse_input(input);
    let lava: HashSet<_> = lava.into_iter().collect();
    surface(&lava)
}

pub fn part_two(input: &str) -> usize {
    let lava = parse_input(input);
    let x1 = lava.iter().map(|v| v.0).min().unwrap() - 1;
    let x2 = lava.iter().map(|v| v.0).max().unwrap() + 1;
    let y1 = lava.iter().map(|v| v.1).min().unwrap() - 1;
    let y2 = lava.iter().map(|v| v.1).max().unwrap() + 1;
    let z1 = lava.iter().map(|v| v.2).min().unwrap() - 1;
    let z2 = lava.iter().map(|v| v.2).max().unwrap() + 1;
    let lava: HashSet<_> = lava.into_iter().collect();

    let mut queue: Vec<_> = vec![(x1, y1, z1)];
    let mut steam: HashSet<_> = vec![(x1, y1, z1)].into_iter().collect();
    while let Some((x, y, z)) = queue.pop() {
        for (dx, dy, dz) in DIRS {
            let k = (x + dx, y + dy, z + dz);
            if k.0 < x1 || k.0 > x2 {
                continue;
            }
            if k.1 < y1 || k.1 > x2 {
                continue;
            }
            if k.2 < z1 || k.2 > z2 {
                continue;
            }
            if steam.contains(&k) || lava.contains(&k) {
                continue;
            }
            steam.insert(k);
            queue.push(k);
        }
    }

    let n = surface(&steam);
    let a = (x2 - x1 + 1) as usize;
    let b = (y2 - y1 + 1) as usize;
    let c = (z2 - z1 + 1) as usize;

    n - (a * b + b * c + a * c) * 2
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(18);
        assert_eq!(part_one(&input), 64);
        assert_eq!(part_two(&input), 58);
    }
}
