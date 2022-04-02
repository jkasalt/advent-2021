use anyhow::{bail, Result};
use core::ops;

#[derive(Debug, Clone)]
pub struct Board {
    board: Vec<(u32, bool)>,
    width: usize,
    height: usize,
}

impl Board {
    pub fn new(vec: Vec<u32>, width: usize, height: usize) -> Result<Self> {
        if width * height != vec.len() {
            bail!("Wrong height and width");
        }
        let board = vec.iter().map(|&n| (n, false)).collect();

        Ok(Self {
            board,
            width,
            height,
        })
    }

    pub fn is_win(&self) -> bool {
        self.is_win_col() || self.is_win_row()
    }

    fn is_win_col(&self) -> bool {
        for j in 0..self.width {
            let mut col_won = true;

            for i in 0..self.height {
                if !self[(i, j)].1 {
                    col_won = false;
                }
            }

            if col_won {
                return true;
            }
        }
        false
    }

    fn is_win_row(&self) -> bool {
        for i in 0..self.height {
            let mut row_won = true;

            for j in 0..self.width {
                if !self[(i, j)].1 {
                    row_won = false;
                }
            }

            if row_won {
                return true;
            }
        }
        false
    }

    pub fn mark(&mut self, num: u32) -> Option<(usize, usize)> {
        match self.board.iter().position(|(n, _)| *n == num) {
            None => None,
            Some(idx) => {
                self.board[idx].1 = true;
                Some((idx / self.width, idx % self.width))
            }
        }
    }

    pub fn sum_unmarked(&self) -> u32 {
        self.board
            .iter()
            .filter_map(|(num, marked)| if !marked { Some(num) } else { None })
            .sum()
    }
}

impl ops::Index<(usize, usize)> for Board {
    type Output = (u32, bool);

    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        if j > self.width - 1 || i > self.height - 1 {
            panic!(
                "Index ({}, {}) out of range for Board with size ({}, {})",
                i, j, self.height, self.width
            );
        }
        &self.board[j + i * self.width]
    }
}

#[aoc_generator(day4)]
pub fn gen(input: &str) -> (Vec<u32>, Vec<Board>) {
    let mut elements = input.split("\n\n");

    let numbers: Vec<u32> = elements
        .next()
        .unwrap()
        .split(',')
        .filter_map(|n| n.parse().ok())
        .collect();

    let boards = elements
        .map(|board| {
            board
                .split('\n')
                .map(|line| {
                    line.split_whitespace()
                        .filter_map(|s| s.parse().ok())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .fold(Vec::new(), |mut v, board| {
            let width = board[0].len();
            let height = width;
            let board: Vec<u32> = board.into_iter().flatten().collect();
            v.push(Board::new(board, width, height).unwrap());
            v
        });

    (numbers, boards)
}

#[aoc(day4, part1)]
pub fn first(input: &(Vec<u32>, Vec<Board>)) -> u32 {
    let (numbers, mut boards) = input.clone();
    for num in numbers {
        for board in &mut boards {
            board.mark(num);
            if board.is_win() {
                return num * board.sum_unmarked();
            }
        }
    }
    unreachable!()
}

#[aoc(day4, part2)]
pub fn second(input: &(Vec<u32>, Vec<Board>)) -> u32 {
    let (numbers, mut boards) = input.clone();
    for num in numbers {
        for board in &mut boards {
            board.mark(num);
        }
        if boards.len() == 1 {
            if boards[0].is_win() {
                return num * boards[0].sum_unmarked();
            }
        } else {
            boards.retain(|board| !board.is_win());
        }
    }
    unreachable!()
}

#[cfg(test)]
mod test {
    use super::*;

    fn input() -> (Vec<u32>, Vec<Board>) {
        gen(
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7",
        )
    }

    #[test]
    fn one() {
        assert_eq!(first(&input()), 4512);
    }

    #[test]
    fn two() {
        assert_eq!(second(&input()), 1924);
    }

    #[test]
    fn col_win() {
        let board = Board {
            board: vec![(1, true), (2, false), (3, true), (4, false)],
            width: 2,
            height: 2,
        };
        assert!(board.is_win_col());
        assert!(!board.is_win_row());
    }
    #[test]
    fn row_win() {
        let board = Board {
            board: vec![(1, true), (2, true), (3, false), (4, false)],
            width: 2,
            height: 2,
        };
        assert!(board.is_win_row());
        assert!(!board.is_win_col());
    }
    #[test]
    fn diag1_win() {
        let board = Board {
            board: vec![(1, true), (2, false), (3, false), (4, true)],
            width: 2,
            height: 2,
        };
        assert!(!board.is_win_row());
        assert!(!board.is_win_col());
    }
    #[test]
    fn diag2_win() {
        let board = Board {
            board: vec![(1, false), (2, true), (3, true), (4, false)],
            width: 2,
            height: 2,
        };
        assert!(!board.is_win_row());
        assert!(!board.is_win_col());
    }
}
