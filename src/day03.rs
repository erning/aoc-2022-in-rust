fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|v| {
            v.bytes()
                .map(|v| match v {
                    b'a'..=b'z' => v - b'a' + 1,
                    b'A'..=b'Z' => v - b'A' + 27,
                    _ => 0,
                })
                .collect()
        })
        .collect()
}

pub fn part_one(input: &str) -> i32 {
    parse_input(input)
        .iter()
        .map(|v| {
            let n = v.len() / 2;
            let mut occupied = [false; 52];
            v[..n].iter().for_each(|&i| occupied[i as usize - 1] = true);
            v[n..].iter().find(|&&i| occupied[i as usize - 1]).unwrap()
        })
        .map(|&v| v as i32)
        .sum()
}

pub fn part_two(input: &str) -> i32 {
    parse_input(input)
        .chunks(3)
        .map(|v| {
            let mut occupied = [(false, false); 52];
            v[0].iter().for_each(|&i| occupied[i as usize - 1].0 = true);
            v[1].iter().for_each(|&i| occupied[i as usize - 1].1 = true);
            v[2].iter()
                .find(|&&i| occupied[i as usize - 1] == (true, true))
                .unwrap()
        })
        .map(|&v| v as i32)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(3);
        assert_eq!(part_one(&input), 157);
        assert_eq!(part_two(&input), 70);
    }
}
