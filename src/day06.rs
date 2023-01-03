fn find_marker(input: &str, distinct: usize) -> usize {
    let mut a = 0;
    let mut b = 0;
    let mut occupied = [false; 256];

    let bytes = input.as_bytes();
    while b < bytes.len() && b - a < distinct {
        let c = bytes[b] as usize;
        if occupied[c] {
            occupied[bytes[a] as usize] = false;
            a += 1;
            continue;
        }
        occupied[c] = true;
        b += 1;
    }
    b
}

pub fn part_one(input: &str) -> usize {
    find_marker(input, 4)
}

pub fn part_two(input: &str) -> usize {
    find_marker(input, 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(part_one("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(part_one("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(part_one("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(part_one("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(part_one("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);

        assert_eq!(part_two("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(part_two("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(part_two("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(part_two("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(part_two("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
