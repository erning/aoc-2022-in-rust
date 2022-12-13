use std::{cmp::Ordering, iter::Peekable, str::Chars};

#[derive(Debug)]
enum Packet {
    Value(u32),
    List(Vec<Packet>),
}

fn parse_packet(line: &str) -> Option<Packet> {
    fn parse_value(chars: &mut Peekable<Chars>) -> Packet {
        let mut value = 0;
        while let Some(c) = chars.next_if(|&c| c.is_digit(10)) {
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
                _ => {
                    chars.next();
                }
            }
        }
        Packet::List(list)
    }

    let mut chars = line.chars().peekable();
    if let Some(c) = chars.next() {
        if c == '[' {
            return Some(parse_list(&mut chars));
        }
    }
    None
}

fn compare_packet(a: &Packet, b: &Packet) -> Ordering {
    match (a, b) {
        (Packet::Value(a), Packet::Value(b)) => a.cmp(b),
        (Packet::List(a), Packet::List(b)) => {
            let mut iter_a = a.iter();
            let mut iter_b = b.iter();
            loop {
                let a = iter_a.next();
                let b = iter_b.next();
                if a.is_none() && b.is_none() {
                    break Ordering::Equal;
                }
                if a.is_none() {
                    break Ordering::Less;
                }
                if b.is_none() {
                    break Ordering::Greater;
                }
                let c = compare_packet(a.unwrap(), b.unwrap());
                if c != Ordering::Equal {
                    break c;
                }
            }
        }
        (Packet::Value(a), Packet::List(_)) => {
            compare_packet(&Packet::List(vec![Packet::Value(*a)]), b)
        }
        (Packet::List(_), Packet::Value(b)) => {
            compare_packet(a, &Packet::List(vec![Packet::Value(*b)]))
        }
    }
}

fn parse_input(input: &str) -> Vec<Packet> {
    input.lines().filter_map(|v| parse_packet(v)).collect()
}

pub fn part_one(input: &str) -> usize {
    parse_input(input)[..]
        .chunks(2)
        .enumerate()
        .filter(|(_, v)| compare_packet(&v[0], &v[1]) == Ordering::Less)
        .map(|(i, _)| i + 1)
        .sum()
}

pub fn part_two(input: &str) -> usize {
    let mut packets = parse_input(input);
    packets.push(parse_packet("[[2]]").unwrap());
    packets.push(parse_packet("[[6]]").unwrap());

    packets.sort_by(|a, b| compare_packet(a, b));

    let a = parse_packet("[[2]]").unwrap();
    let b = parse_packet("[[6]]").unwrap();

    let mut v = 1;
    for (i, p) in packets.iter().enumerate() {
        if compare_packet(&a, &p) == Ordering::Equal {
            v *= i + 1;
        }
        if compare_packet(&b, &p) == Ordering::Equal {
            v *= i + 1;
        }
    }

    v
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
