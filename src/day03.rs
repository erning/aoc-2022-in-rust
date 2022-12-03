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
    let mut sum = 0_i32;
    let rucksacks = parse_input(input);
    for rucksack in rucksacks {
        let mut items: [u8; 52] = [0; 52];
        let len = rucksack.len() / 2;
        for v in rucksack[..len].iter() {
            items[(*v - 1) as usize] += 1;
        }
        for v in rucksack[len..].iter() {
            if items[(*v - 1) as usize] > 0 {
                sum += *v as i32;
                break;
            }
        }
    }
    sum
}

pub fn part_two(input: &str) -> i32 {
    let mut sum = 0_i32;
    let rucksacks = parse_input(input);
    let mut iter = rucksacks.iter();
    while let (Some(a), Some(b), Some(c)) =
        (iter.next(), iter.next(), iter.next())
    {
        let mut common_items: [u8; 52] = [0; 52];
        let mut items: [u8; 52] = [0; 52];
        for v in a.iter() {
            items[(*v - 1) as usize] += 1;
        }
        for v in b.iter() {
            if items[(*v - 1) as usize] > 0 {
                common_items[(*v - 1) as usize] += 1;
            }
        }
        for v in c.iter() {
            if common_items[(*v - 1) as usize] > 0 {
                sum += *v as i32;
                break;
            }
        }
    }
    sum
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
