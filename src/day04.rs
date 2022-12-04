fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|v| v.split(&['-', ',']).map(|v| v.parse().unwrap()).collect())
        .collect()
}

pub fn part_one(input: &str) -> usize {
    parse_input(input)
        .iter()
        .filter(|v| {
            (v[0] >= v[2] && v[1] <= v[3]) || (v[2] >= v[0] && v[3] <= v[1])
        })
        .count()
}

pub fn part_two(input: &str) -> usize {
    parse_input(input)
        .iter()
        .filter(|v| {
            (v[0] <= v[2] && v[1] >= v[2]) || (v[2] <= v[0] && v[3] >= v[0])
        })
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
