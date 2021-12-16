use crate::matrix::Matrix;
use std::collections::{HashMap, HashSet};

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
    let res = field.risk_from(0, 0, Vec::new(), &mut memory);
    let mut keys = memory.keys().collect::<Vec<_>>();
    keys.sort_unstable();
    println!("{:?}", field.final_path_from(0, 0, &memory, Vec::new()));
    res
}

trait Risk {
    fn risk_from(
        &self,
        x: usize,
        y: usize,
        visited: Vec<(usize, usize)>,
        memory: &mut HashMap<(usize, usize), u64>,
    ) -> u64;

    fn final_path_from(
        &self,
        x: usize,
        y: usize,
        memory: &HashMap<(usize, usize), u64>,
        visited: Vec<(usize, usize)>,
    ) -> Vec<((usize, usize), u32)>;
}

impl Risk for Matrix<u32> {
    fn risk_from(
        &self,
        x: usize,
        y: usize,
        mut visited: Vec<(usize, usize)>,
        memory: &mut HashMap<(usize, usize), u64>,
    ) -> u64 {
        visited.push((x, y));

        // If we start from the destination, the risk is just the value inside
        if x == self.width() - 1 && y == self.height() - 1 {
            memory.insert((x, y), self[(x, y)] as u64);
            self[(x, y)] as u64
        }
        // Otherwise if we already know the risk from a given point, just return that
        else if let Some(&val) = memory.get(&(x, y)) {
            val
        // Otherwise find the guys you can visit and recur
        } else {
            let mut can_go = self.rook_neighbor_indices(x, y).filter(|&(xn, yn)| {
                // If xn is close to either edge, can't go up
                if (xn < 2 || xn >= self.width() - 2) && yn + 1 == y {
                    return false;
                }
                // If yn is close to either edge, can't go left
                if (yn < 2 || yn >= self.height() - 2) && xn + 1 == x {
                    return false;
                }
                // If the new position is in contact with one of the nodes we already visited,
                // we can't go there
                if visited.iter().any(|&(xo, yo)| {
                    if xo == x && yo == y {
                        false
                    } else {
                        (xn as isize - xo as isize).abs() + (yn as isize - yo as isize).abs() <= 1
                    }
                }) {
                    return false;
                }
                // Finally can't go to a node we already visited
                !visited.contains(&(xn, yn))
            });

            if x == 8 && y == 6 {
                println!("visited: {:?}", visited);
            }

            let val = if visited.len() == 1 {
                can_go
                    .map(|(xn, yn)| self.risk_from(xn, yn, visited.clone(), memory))
                    .min()
                    .unwrap()
            } else {
                self[(x, y)] as u64
                    + can_go
                        .map(|(xn, yn)| {
                            if xn == 8 && yn == 5 {
                                println!("({},{}), can_go: {:?}", x, y, (xn, yn));
                            }

                            (xn, yn, self.risk_from(xn, yn, visited.clone(), memory))
                        })
                        .inspect(|(xn, yn, elem)| println!("({}, {}) - found val {} with ({},{})", x, y, elem, xn, yn))
                        .map(|(_,_, val)| val)
                        .min()
                        .unwrap()
            };
            memory.insert((x, y), val);

            val
        }
    }

    fn final_path_from(
        &self,
        x: usize,
        y: usize,
        memory: &HashMap<(usize, usize), u64>,
        mut visited: Vec<(usize, usize)>,
    ) -> Vec<((usize, usize), u32)> {
        visited.push((x, y));
        let (xb, yb) = self
            .rook_neighbor_indices(x, y)
            .filter(|neighbor| !visited.contains(neighbor))
            .min_by_key(|neighbor| memory[neighbor])
            .unwrap();
        if x == self.width() - 1 && y == self.height() - 1 {
            return vec![((x, y), self[(x, y)])];
        }
        let mut res = self.final_path_from(xb, yb, memory, visited.clone());
        res.push(((x, y), self[(x, y)]));
        res
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
        assert_eq!(first(&input), 2);
    }

    #[test]
    fn mini2() {
        let input = gen("111111
                         999991
                         111991
                         191111
                         199999
                         111111");
        assert_eq!(first(&input), 18);
    }
}
