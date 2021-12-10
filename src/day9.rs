use std::collections::HashSet;
use std::ops;

#[derive(Debug, Clone)]
struct Matrix {
    vec: Vec<u32>,
    width: usize,
    height: usize,
}

impl Matrix {
    fn get(&self, x: isize, y: isize) -> Option<&u32> {
        if x < 0
            || x > (self.width - 1).try_into().unwrap()
            || y < 0
            || y > (self.height - 1).try_into().unwrap()
        {
            return None;
        }
        let x: usize = x.try_into().unwrap();
        let y: usize = y.try_into().unwrap();
        self.vec.get(x + y * self.width)
    }

    fn neighbors_of(&self, x: usize, y: usize) -> [Option<&u32>; 4] {
        let x: isize = x.try_into().unwrap();
        let y: isize = y.try_into().unwrap();

        let top = self.get(x, y + 1);
        let bot = self.get(x, y - 1);
        let left = self.get(x - 1, y);
        let right = self.get(x + 1, y);

        [top, bot, left, right]
    }

    fn neighbor_indices(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut res = Vec::new();
        if y > 0 {
            res.push((x, y - 1));
        }
        if y < self.height - 1 {
            res.push((x, y + 1));
        }
        if x > 0 {
            res.push((x - 1, y));
        }
        if x < self.width - 1 {
            res.push((x + 1, y));
        }
        res
    }

    fn is_low_point(&self, x: usize, y: usize) -> bool {
        self.neighbors_of(x, y).iter().all(|nei| {
            if let Some(val) = nei {
                **val > self[(x, y)]
            } else {
                true
            }
        })
    }

    fn basin_of(&self, x: usize, y: usize) -> HashSet<(usize, usize)> {
        let mut basin = HashSet::new();
        let mut to_visit = vec![(x, y)];

        while !to_visit.is_empty() {
            let (x, y) = to_visit.pop().unwrap();
            for (xn, yn) in self.neighbor_indices(x, y) {
                if self[(xn, yn)] != 9 && basin.insert((xn, yn)) {
                    to_visit.push((xn, yn));
                }
            }
        }
        basin
    }

    fn low_points(&self) -> Vec<(usize, usize)> {
        let mut res = Vec::new();
        for x in 0..self.width {
            for y in 0..self.height {
                if self.is_low_point(x, y) {
                    res.push((x, y))
                }
            }
        }
        res
    }
}

impl ops::Index<(usize, usize)> for Matrix {
    type Output = u32;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        if x > self.width - 1 || y > self.height - 1 {
            panic!(
                "Index ({}, {}) out of range for Matrix with size ({}, {})",
                y, x, self.height, self.width
            );
        }
        &self.vec[x + y * self.width]
    }
}

#[aoc_generator(day9)]
fn gen(input: &str) -> Matrix {
    let width = input.chars().position(|c| c == '\n').unwrap();
    let height = input.lines().count();
    let vec = input.chars().filter_map(|c| c.to_digit(10)).collect();

    Matrix { vec, width, height }
}

#[aoc(day9, part1)]
fn first(mat: &Matrix) -> u32 {
    let mut heights = vec![];
    dbg!(&mat);
    for x in 0..mat.width {
        for y in 0..mat.height {
            if mat.is_low_point(x, y) {
                heights.push(mat[(x, y)])
            }
        }
    }
    heights.iter().map(|h| h + 1).sum()
}

#[aoc(day9, part2)]
fn second(mat: &Matrix) -> u32 {
    let mut b = mat
        .low_points()
        .iter()
        .map(|lp| mat.basin_of(lp.0, lp.1))
        .collect::<Vec<_>>();
    b.sort_unstable_by_key(|basin| basin.len());
    b.iter().rev().take(3).map(|basin| basin.len() as u32).product()
}

#[cfg(test)]
mod tests {
    use super::*;
    fn sample() -> Matrix {
        gen("2199943210
3987894921
9856789892
8767896789
9899965678")
    }

    #[test]
    fn one() {
        assert_eq!(first(&sample()), 15)
    }

    #[test]
    fn two() {
        assert_eq!(second(&sample()), 1134)
    }
}
