use crate::matrix::Matrix;
use rand::prelude::*;
use std::collections::{HashMap, HashSet, BinaryHeap};
use std::cmp::Reverse;

#[aoc_generator(day15)]
fn gen(input: &str) -> Matrix<u32> {
    let items = input.chars().filter_map(|c| c.to_digit(10));
    let width = input.chars().position(|c| c == '\n').unwrap();
    let height = input.lines().count();

    Matrix::new(items, width, height)
}

#[aoc(day15, part1)]
fn first(field: &Matrix<u32>) -> u64 {
    let mut distances = BinaryHeap::new();
    let mut predecessor = HashMap::new();
    let mut q = HashSet::new();

    for x in 0..field.width() {
        for y in 0..field.height() {
            distances.insert(Reverse((None, (x,y))));
            predecessor.insert((x,y), None);
            q.push((x,y))
        }
    }
    distances.insert(Reverse(((0,0), 0)));
}

trait Risk {
    fn risk_from(
        &self,
        x: usize,
        y: usize,
        visited: Vec<(usize, usize)>,
        memory: &mut HashMap<(usize, usize), u64>,
        rng: &mut impl Rng,
    ) -> Option<u64>;

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
        rng: &mut impl Rng,
    ) -> Option<u64> {
        visited.push((x, y));

        // println!(
        //     "When we see ({},{}) .. we already visited {:?}",
        //     x, y, visited
        // );
        //
        // println!("memory: {:?}", memory);

        // If we start from the destination, the risk is just the value inside
        if x == self.width() - 1 && y == self.height() - 1 {
            memory.insert((x, y), self[(x, y)] as u64);
            Some(self[(x, y)] as u64)
        }
        // Otherwise if we already know the risk from a given point, just return that
        else if let Some(&val) = memory.get(&(x, y)) {
            Some(val)
        // Otherwise find the guys you can visit and recur
        } else {
            let mut can_go: Vec<_> = self.rook_neighbor_indices(x, y).collect();
            can_go.shuffle(rng);

            let maybe_val = if visited.len() == 1 {
                can_go
                    .iter()
                    .map(|&(xn, yn)| (xn, yn, self.risk_from(xn, yn, visited.clone(), memory, rng)))
                    .inspect(|(xn, yn, elem)| {
                        println!(
                            "({},{}) -> ({},{}) .. found val {:?} + {}",
                            x,
                            y,
                            xn,
                            yn,
                            elem,
                            self[(x, y)]
                        )
                    })
                    .filter_map(|(_, _, val)| val)
                    .min()
            } else {
                can_go
                    .iter()
                    .map(|&(xn, yn)| (xn, yn, self.risk_from(xn, yn, visited.clone(), memory, rng)))
                    .inspect(|(xn, yn, elem)| {
                        println!(
                            "({},{}) -> ({},{}) .. found val {:?} + {}",
                            x,
                            y,
                            xn,
                            yn,
                            elem,
                            self[(x, y)]
                        )
                    })
                    .filter_map(|(_, _, val)| val)
                    .min()
                    .map(|maybe_val| self[(x, y)] as u64 + maybe_val)
            };
            if let Some(val) = maybe_val {
                memory.insert((x, y), val);
            }

            maybe_val
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
    fn mini1() {
        let input = gen("11
                         91");
        assert_eq!(first(&input), 2);
    }

    #[test]
    fn mini2() {
        let input = gen("111111
                         999991
                         111991
                         111111
                         111199
                         111111");
        assert_eq!(first(&input), 14);
    }
}
