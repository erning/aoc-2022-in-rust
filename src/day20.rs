fn parse_input(input: &str) -> Vec<(i64, usize, usize)> {
    // (value, prev-index, next-index)
    let n = input.lines().count();
    input
        .lines()
        .map(|v| v.parse::<i64>().unwrap())
        .enumerate()
        .map(|(i, v)| (v, (n + i - 1) % n, (i + 1) % n))
        .collect()
}

#[allow(dead_code)]
fn show_list(list: &Vec<(i64, usize, usize)>, i: usize) {
    let n = list.len();
    let mut i = i % n;
    for _ in 0..n {
        print!("{},", list[i].0);
        i = list[i].2;
    }
    println!();
}

fn mix(list: &mut Vec<(i64, usize, usize)>) -> usize {
    let n = list.len();
    let mut zero = n;
    for i in 0..n {
        let (v, a, b) = list[i];
        if v == 0 {
            zero = i;
            continue;
        }
        // minus one to skip the removed current node
        let m = v % (n as i64 - 1);
        if m == 0 {
            continue;
        }

        // remove from list
        list[a].2 = b;
        list[b].1 = a;
        // move m steps
        let mut j = i;
        if v < 0 {
            for _ in 0..1 - m {
                j = list[j].1
            }
        } else {
            for _ in 0..m {
                j = list[j].2;
            }
        }
        // insert after j
        let (_, _, b) = list[j];
        list[b].1 = i;
        list[j].2 = i;
        list[i].1 = j;
        list[i].2 = b;
    }
    zero
}

pub fn part_one(input: &str) -> i64 {
    let mut list = parse_input(input);
    let mut i = mix(&mut list);
    let mut sum = 0;
    for _ in 0..3 {
        for _ in 0..1000 {
            i = list[i].2;
        }
        sum += list[i].0;
    }
    sum
}

pub fn part_two(input: &str) -> i64 {
    let mut list = parse_input(input);
    // apply the decryption key
    for v in list.iter_mut() {
        v.0 *= 811589153;
    }
    for _ in 0..9 {
        mix(&mut list);
    }
    let mut i = mix(&mut list);
    let mut sum = 0;
    for _ in 0..3 {
        for _ in 0..1000 {
            i = list[i].2;
        }
        sum += list[i].0;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(20);
        assert_eq!(part_one(&input), 3);
        assert_eq!(part_two(&input), 1623178306);
    }
}
