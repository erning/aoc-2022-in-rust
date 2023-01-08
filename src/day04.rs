fn parse_input(input: &str) -> Vec<[i32; 4]> {
    input
        .lines()
        .map(|v| {
            v.split(&['-', ','])
                .map(|v| v.parse().unwrap())
                .collect::<Vec<i32>>()
                .try_into()
                .unwrap()
        })
        .collect()
}

pub fn part_one(input: &str) -> usize {
    parse_input(input)
        .iter()
        .filter(|[a, b, c, d]| (a >= c && b <= d) || (c >= a && d <= b))
        .count()
}

pub fn part_two(input: &str) -> usize {
    parse_input(input)
        .iter()
        .filter(|[a, b, c, d]| (a <= c && b >= c) || (c <= a && d >= a))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(4);
        assert_eq!(part_one(&input), 2);
        assert_eq!(part_two(&input), 4);
    }
}
