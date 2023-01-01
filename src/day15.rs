fn merge_segments(segments: &[(i32, i32)]) -> Vec<(i32, i32)> {
    fn merge(a: (i32, i32), b: (i32, i32)) -> Option<(i32, i32)> {
        assert!(a.0 <= a.1 && b.0 <= b.1);
        assert!(a.0 <= b.0);
        if a.1 + 1 < b.0 {
            None
        } else if a.1 >= b.1 {
            Some(a)
        } else {
            Some((a.0, b.1))
        }
    }

    let mut merged: Vec<(i32, i32)> = vec![];
    let mut a = segments[0];
    for &b in &segments[1..] {
        if let Some(v) = merge(a, b) {
            a = v;
        } else {
            merged.push(a);
            a = b;
        }
    }
    merged.push(a);
    merged
}

fn find_segments_at_row(
    sensors: &[(i32, i32, i32)],
    row: i32,
) -> Vec<(i32, i32)> {
    let mut segments: Vec<(i32, i32)> = Vec::new();
    for &(x, y, d) in sensors.iter() {
        let distance_to_row = (row - y).abs();
        let offset = d - distance_to_row;
        if offset < 0 {
            continue;
        }
        let (a, b) = (x - offset, x + offset);
        segments.push((a, b));
    }
    segments.sort_unstable();
    segments = merge_segments(&segments);
    segments
}

fn parse_input(input: &str) -> Vec<(i32, i32, i32)> {
    input
        .lines()
        .map(|v| {
            let v = v
                .split(['=', ',', ':'])
                .enumerate()
                .filter(|(i, _)| [1, 3, 5, 7].contains(i))
                .map(|(_, v)| v.parse().unwrap())
                .collect::<Vec<i32>>();
            (v[0], v[1], (v[2] - v[0]).abs() + (v[3] - v[1]).abs())
        })
        .collect()
}

pub fn part_one(input: &str) -> i32 {
    let sensors = parse_input(input);

    let row = if sensors.len() == 14 { 10 } else { 2000000 };
    let segments = find_segments_at_row(&sensors, row);
    segments.into_iter().map(|(a, b)| b - a).sum()
}

pub fn part_two(input: &str) -> i64 {
    let sensors = parse_input(input);

    let range = if sensors.len() == 14 { 20 } else { 4000000 };
    for y in (0..=range).rev() {
        // trick ;-)
        let segments = find_segments_at_row(&sensors, y);
        if segments.len() <= 1 {
            continue;
        }
        // Find the only possible position for the distress beacon
        // only ONE position :-)
        let (a, b) = (segments[0], segments[1]);
        assert!(a.1 + 2 == b.0);
        let x = a.1 + 1;
        return x as i64 * 4000000 + y as i64;
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(15);
        assert_eq!(part_one(&input), 26);
        assert_eq!(part_two(&input), 56000011);
    }
}
