use std::cmp::Ordering;
use std::iter::Peekable;
use std::str::Chars;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    Value(u32),
    List(Vec<Packet>),
}

impl FromStr for Packet {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_value(chars: &mut Peekable<Chars>) -> Packet {
            let mut value = 0;
            while let Some(c) = chars.next_if(|&c| c.is_ascii_digit()) {
                value = value * 10 + c.to_digit(10).unwrap();
            }
            Packet::Value(value)
        }

        fn parse_list(chars: &mut Peekable<Chars>) -> Packet {
            let mut list: Vec<Packet> = vec![];
            while let Some(c) = chars.peek() {
                match c {
                    '0'..='9' => {
                        list.push(parse_value(chars));
                    }
                    '[' => {
                        chars.next();
                        list.push(parse_list(chars));
                    }
                    ']' => {
                        chars.next();
                        break;
                    }
                    ',' => {
                        chars.next();
                    }
                    _ => {
                        panic!("unknow packet");
                    }
                }
            }
            Packet::List(list)
        }

        let mut chars = s.chars().peekable();
        if let Some(c) = chars.next() {
            if c == '[' {
                return Ok(parse_list(&mut chars));
            }
        }
        Err("should be a list".to_string())
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Packet::Value(a), Packet::Value(b)) => a.partial_cmp(b),
            (Packet::List(a), Packet::List(b)) => {
                let mut iter_a = a.iter();
                let mut iter_b = b.iter();
                loop {
                    match (iter_a.next(), iter_b.next()) {
                        (None, None) => break Some(Ordering::Equal),
                        (None, Some(_)) => break Some(Ordering::Less),
                        (Some(_), None) => break Some(Ordering::Greater),
                        (Some(a), Some(b)) => match a.partial_cmp(b) {
                            Some(Ordering::Equal) => continue,
                            c => break c,
                        },
                    }
                }
            }
            (Packet::Value(a), Packet::List(_)) => {
                Packet::List(vec![Packet::Value(*a)]).partial_cmp(other)
            }
            (Packet::List(_), Packet::Value(b)) => {
                self.partial_cmp(&Packet::List(vec![Packet::Value(*b)]))
            }
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn parse_input(input: &str) -> Vec<Packet> {
    input
        .lines()
        .filter_map(|v| Packet::from_str(v).ok())
        .collect()
}

pub fn part_one(input: &str) -> usize {
    parse_input(input)[..]
        .chunks(2)
        .enumerate()
        .filter(|&(_, v)| v[0] < v[1])
        .map(|(i, _)| i + 1)
        .sum()
}

pub fn part_two(input: &str) -> usize {
    let mut packets = parse_input(input);

    let p1 = Packet::from_str("[[2]]").unwrap();
    let p2 = Packet::from_str("[[6]]").unwrap();

    packets.push(p1.clone());
    packets.push(p2.clone());
    packets.sort_unstable();

    packets
        .iter()
        .enumerate()
        .filter(|&(_, p)| p == &p1 || p == &p2)
        .map(|(i, _)| i + 1)
        .product()

    // let mut a = 0;
    // let mut b = 0;
    // for (i, p) in packets.iter().enumerate() {
    //     if p == &p1 {
    //         a = i + 1;
    //     } else if p == &p2 {
    //         b = i + 1;
    //         break;
    //     }
    // }
    // a * b
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(13);
        assert_eq!(part_one(&input), 13);
        assert_eq!(part_two(&input), 140);
    }
}
