fn parse_input(input: &str) -> Vec<i32> {
    let mut instructions: Vec<i32> = Vec::new();
    let mut value = 0;
    for c in input.lines().last().unwrap().chars() {
        match c {
            'R' => {
                instructions.push(value);
                instructions.push(1 - 2);
                value = 0;
            }
            'L' => {
                instructions.push(value);
                instructions.push(-1 - 2);
                value = 0;
            }
            _ => {
                value *= 10;
                value += c.to_digit(10).unwrap() as i32;
            }
        }
    }
    instructions.push(value);
    instructions
}

const R: i32 = 0;
const D: i32 = 1;
const L: i32 = 2;
const U: i32 = 3;

struct Board {
    board: Vec<Vec<u8>>,
    rows: Vec<(i32, i32)>,
    cols: Vec<(i32, i32)>,
}

impl Board {
    fn new(input: &str) -> Board {
        let width = input
            .lines()
            .take_while(|v| !v.is_empty())
            .map(|v| v.len())
            .max()
            .unwrap();
        let height = input.lines().count() - 2;

        let board: Vec<Vec<u8>> = input
            .lines()
            .take_while(|v| !v.is_empty())
            .map(|v| {
                v.bytes()
                    .map(|v| match v {
                        b'.' => 1,
                        b'#' => 2,
                        _ => 0,
                    })
                    .collect::<Vec<u8>>()
            })
            .map(|v| {
                let mut v = v;
                v.extend(std::iter::repeat(0).take(width - v.len()));
                v
            })
            .collect();

        let rows: Vec<(i32, i32)> = (0..height)
            .into_iter()
            .map(|y| {
                let a = (0..width)
                    .into_iter()
                    .find(|&x| board[y][x] > 0)
                    .unwrap();
                let b = (0..width)
                    .into_iter()
                    .rev()
                    .find(|&x| board[y][x] > 0)
                    .unwrap();
                (a as i32, b as i32)
            })
            .collect();

        let cols: Vec<(i32, i32)> = (0..width)
            .into_iter()
            .map(|x| {
                let a = (0..height)
                    .into_iter()
                    .find(|&y| board[y][x] > 0)
                    .unwrap();
                let b = (0..height)
                    .into_iter()
                    .rev()
                    .find(|&y| board[y][x] > 0)
                    .unwrap();
                (a as i32, b as i32)
            })
            .collect();

        Board { board, rows, cols }
    }

    fn next(&self, x: &mut i32, y: &mut i32, d: i32) -> bool {
        let (r1, r2) = self.rows[*y as usize];
        let (c1, c2) = self.cols[*x as usize];
        let (nx, ny) = match d {
            R => (if *x == r2 { r1 } else { *x + 1 }, *y),
            D => (*x, if *y == c2 { c1 } else { *y + 1 }),
            L => (if *x == r1 { r2 } else { *x - 1 }, *y),
            U => (*x, if *y == c1 { c2 } else { *y - 1 }),
            _ => panic!(),
        };
        if self.is_open_tile(nx, ny) {
            (*x, *y) = (nx, ny);
            true
        } else {
            false
        }
    }

    fn is_open_tile(&self, x: i32, y: i32) -> bool {
        self.board[y as usize][x as usize] == 1
    }

    fn edge(&self, x: i32, y: i32, d: i32) -> i32 {
        match d {
            R => self.rows[y as usize].1,
            D => self.cols[x as usize].1,
            L => self.rows[y as usize].0,
            U => self.rows[x as usize].0,
            _ => panic!(),
        }
    }
}

struct Cube {
    surfaces: Vec<Vec<Vec<u8>>>,
}

impl Cube {
    fn new(input: &str) -> Cube {
        let surfaces = vec![Vec::new(); 6];
        Cube { surfaces }
    }

    fn next(
        &self,
        s: &mut i32,
        x: &mut i32,
        y: &mut i32,
        d: &mut i32,
    ) -> bool {
        true
    }

    fn is_open_tile(&self, s: i32, x: i32, y: i32) -> bool {
        self.surfaces[s as usize][y as usize][x as usize] == 1
    }
}

pub fn part_one(input: &str) -> i32 {
    let instructions = parse_input(input);
    let board = Board::new(input);
    let (mut x, mut y) = (board.edge(0, 0, L), 0);
    let mut d = R;

    for i in instructions.into_iter() {
        if i > 0 {
            for _ in 0..i {
                if !board.next(&mut x, &mut y, d) {
                    break;
                }
            }
        } else {
            d += i + 2;
            d += 4;
            d %= 4;
        }
    }

    1000 * (y + 1) + 4 * (x + 1) + d
}

pub fn part_two(input: &str) -> i32 {
    // let instructions = parse_input(input);
    // let cube = Cube::new(input);
    -1
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(22);
        assert_eq!(part_one(&input), 6032);
        assert_eq!(part_two(&input), 0);
    }
}
