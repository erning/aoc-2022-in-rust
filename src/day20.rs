// (value, prev-index, next-index)
type List = Vec<(i64, usize, usize)>;

fn parse_input(input: &str) -> List {
    let n = input.lines().count();
    input
        .lines()
        .enumerate()
        .map(|(i, v)| (v.parse().unwrap(), (n + i - 1) % n, (i + 1) % n))
        .collect()
}

#[allow(dead_code)]
fn show_list(list: &List, i: usize) {
    let n = list.len();
    let mut i = i % n;
    for _ in 0..n {
        print!("{},", list[i].0);
        i = list[i].2;
    }
    println!();
}

// do the mix operation and return the index of zero
fn mix(list: &mut List) -> usize {
    let n = list.len();
    let mut zero = n;
    for i in 0..n {
        let (v, a, b) = list[i];
        if v == 0 {
            zero = i;
            continue;
        }
        // remove from list
        list[a].2 = b;
        list[b].1 = a;
        // minus one to skip the removed node
        let m = v % (n as i64 - 1);
        if m == 0 {
            continue;
        }
        // move m steps
        let j = if v < 0 {
            (0..1 - m).fold(i, |k, _| list[k].1)
        } else {
            (0..m).fold(i, |k, _| list[k].2)
        };
        // insert after j
        let (_, _, b) = list[j];
        list[b].1 = i;
        list[j].2 = i;
        list[i].1 = j;
        list[i].2 = b;
    }
    zero
}

// sum the values of 1000th, 2000th, and 3000th numbers after the value 0
fn final_sum(list: &List, mut i: usize) -> i64 {
    (0..3).fold(0, |sum, _| {
        i = (0..1000).fold(i, |j, _| list[j].2);
        sum + list[i].0
    })
}

pub fn part_one(input: &str) -> i64 {
    let mut list = parse_input(input);
    let i = mix(&mut list);
    final_sum(&list, i)
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
    let i = mix(&mut list);
    final_sum(&list, i)
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
