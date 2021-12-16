use crate::matrix::Matrix;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn r#move(&self, x: usize, y: usize) -> (usize, usize) {
        match self {
            Self::Left => (x - 1, y),
            Self::Right => (x + 1, y),
            Self::Up => (x, y - 1),
            Self::Down => (x, y + 1),
        }
    }

    fn opp(&self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}

#[aoc_generator(day15)]
fn gen(input: &str) -> Matrix<u32> {
    let items = input.chars().filter_map(|c| c.to_digit(10));
    let width = input.chars().position(|c| c == '\n').unwrap();
    let height = input.lines().count();

    Matrix::new(items, width, height)
}

#[aoc(day15, part1)]
fn first(field: &Matrix<u32>) -> u64 {
    let mut memory = HashMap::new();
    field.risk_from(0, 0, Direction::Up, &mut memory)
}

trait Risk {
    fn risk_from(
        &self,
        x: usize,
        y: usize,
        came_from: Direction,
        memory: &mut HashMap<(usize, usize), u64>,
    ) -> u64;
}

enum Edge {
    Min,
    Max,
    Neither,
}

impl Risk for Matrix<u32> {
    fn risk_from(
        &self,
        x: usize,
        y: usize,
        came_from: Direction,
        memory: &mut HashMap<(usize, usize), u64>,
    ) -> u64 {

        println!("x: {}, y: {}, from: {:?}, cur: {}", x, y, came_from, self[(x,y)]);
        println!("memory: {:?}", memory);

        let y_edge = if y == 0 {
            Edge::Min
        } else if y == self.height() - 1 {
            Edge::Max
        } else {
            Edge::Neither
        };
        let x_edge = if x == 0 {
            Edge::Min
        } else if x == self.width() - 1 {
            Edge::Max
        } else {
            Edge::Neither
        };

        if let Some(val) = memory.get(&(x, y)) {
            *val
        } else {
            let val = match (x_edge, y_edge) {
                // Corners
                (Edge::Max, Edge::Max) => 0,
                (Edge::Min, Edge::Max) => self.risk_from(x + 1, y, Direction::Left, memory),
                (Edge::Max, Edge::Min) => self.risk_from(x, y + 1, Direction::Up, memory),
                (Edge::Min, Edge::Min) => *[
                    self.risk_from(x + 1, y, Direction::Left, memory),
                    self.risk_from(x, y + 1, Direction::Up, memory),
                ]
                .iter()
                .min()
                .unwrap(),
                // Edges
                (Edge::Neither, Edge::Min) => {
                    if came_from == Direction::Down {
                        self.risk_from(x + 1, y, Direction::Left, memory)
                    } else {
                        *[
                            self.risk_from(x + 1, y, Direction::Left, memory),
                            self.risk_from(x, y + 1, Direction::Up, memory),
                        ]
                        .iter()
                        .min()
                        .unwrap()
                    }
                }
                (Edge::Neither, Edge::Max) => {
                    if came_from == Direction::Up {
                        self.risk_from(x + 1, y, Direction::Left, memory)
                    } else {
                        *[
                            self.risk_from(x + 1, y, Direction::Left, memory),
                            self.risk_from(x, y - 1, Direction::Down, memory),
                        ]
                        .iter()
                        .min()
                        .unwrap()
                    }
                }
                (Edge::Min, Edge::Neither) => {
                    if came_from == Direction::Right {
                        self.risk_from(x, y + 1, Direction::Up, memory)
                    } else {
                        *[
                            self.risk_from(x + 1, y, Direction::Left, memory),
                            self.risk_from(x, y + 1, Direction::Up, memory),
                        ]
                        .iter()
                        .min()
                        .unwrap()
                    }
                }
                (Edge::Max, Edge::Neither) => {
                    if came_from == Direction::Left {
                        self.risk_from(x, y + 1, Direction::Up, memory)
                    } else {
                        *[
                            self.risk_from(x, y + 1, Direction::Up, memory),
                            self.risk_from(x - 1, y, Direction::Right, memory),
                        ]
                        .iter()
                        .min()
                        .unwrap()
                    }
                }
                (Edge::Neither, Edge::Neither) => [
                    Direction::Right,
                    Direction::Down,
                    Direction::Up,
                    Direction::Left,
                ]
                .iter()
                .filter(|&&dir| dir != came_from)
                .map(|dir| {
                    let (x_mv, y_mv) = dir.r#move(x, y);
                    self.risk_from(x_mv, y_mv, dir.opp(), memory)
                })
                .min()
                .unwrap(),
            };
            memory.insert((x, y), self[(x, y)] as u64 + val);
            memory[&(x, y)]
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample() -> Matrix<u32> {
        gen("1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581")
    }

    #[test]
    fn one() {
        assert_eq!(first(&sample()), 40);
    }

    #[test]
    fn mini() {
        let input = gen("11
                         91");
        assert_eq!(first(&input), 3);
    }

    #[test]
    fn mini2() {
        let input = gen("111111
                         999991
                         111991
                         191111
                         199999
                         111111");
        assert_eq!(first(&input), 3);
    }
}
