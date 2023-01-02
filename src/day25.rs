fn parse_input(input: &str) -> Vec<i64> {
    input.lines().into_iter().map(to_decimal).collect()
}

fn to_decimal(snafu: &str) -> i64 {
    snafu
        .chars()
        .map(|c| match c {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => panic!(),
        })
        .reduce(|a, b| a * 5 + b)
        .unwrap()
}

fn from_decimal(mut value: i64) -> String {
    let mut snafu: Vec<char> = Vec::new();
    while value > 0 {
        snafu.push(match (value + 2) % 5 - 2 {
            2 => '2',
            1 => '1',
            0 => '0',
            -1 => '-',
            -2 => '=',
            _ => panic!(),
        });
        value = (value + 2) / 5;
    }
    String::from_iter(snafu.into_iter().rev())
}

pub fn part_one(input: &str) -> String {
    let nums = parse_input(input);
    let sum = nums.into_iter().sum();
    from_decimal(sum)
}

pub fn part_two(_: &str) -> String {
    "fifty stars".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(25);
        assert_eq!(part_one(&input), "2=-1=0");
        assert_eq!(part_two(&input), "fifty stars");
    }
}
