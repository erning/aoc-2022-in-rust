fn find_marker(input: &str, distinct: usize) -> usize {
    // fn is_unique(bytes: &[u8]) -> bool {
    //     let mut unique: [bool; 256] = [false; 256];
    //     for &c in bytes {
    //         if unique[c as usize] {
    //             return false;
    //         }
    //         unique[c as usize] = true;
    //     }
    //     return true;
    // }
    //
    // for (i, bytes) in input.as_bytes().windows(distinct).enumerate() {
    //     if is_unique(bytes) {
    //         return i + distinct;
    //     }
    // }

    let mut a = 0;
    let mut b = 0;
    let mut unique: [bool; 256] = [false; 256];

    let bytes = input.as_bytes();
    while b < bytes.len() && b - a < distinct {
        let c = bytes[b] as usize;
        if unique[c] {
            let c = bytes[a] as usize;
            unique[c] = false;
            a += 1;
            continue;
        }
        unique[c] = true;
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
