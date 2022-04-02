use crate::matrix::Matrix;
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Point {
    pos: (usize, usize),
    weight: u32,
}

impl Point {
    pub fn new(x: usize, y: usize, weight: u32) -> Self {
        Self {
            weight,
            pos: (x, y),
        }
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.weight.partial_cmp(&self.weight)
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .weight
            .cmp(&self.weight)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

#[aoc_generator(day15, part1)]
pub fn gen(input: &str) -> Matrix<u32> {
    let items = input.chars().filter_map(|c| c.to_digit(10));
    let width = input.chars().position(|c| c == '\n').unwrap();
    let height = input.lines().count();

    Matrix::new(items, width, height)
}

#[aoc(day15, part1)]
pub fn first(field: &Matrix<u32>) -> u32 {
    let mut q = BinaryHeap::new();
    let mut distances = HashMap::new();

    for x in 0..field.width() {
        for y in 0..field.height() {
            distances.insert((x, y), u32::MAX);
        }
    }
    q.push(Point::new(0, 0, 0));
    *distances.get_mut(&(0, 0)).unwrap() = 0;

    while let Some(u) = q.pop() {
        let Point { pos, weight } = u;
        let (x, y) = pos;

        if (x, y) == (field.width() - 1, field.height() - 1) {
            return weight;
        }
        if weight > distances[&(x, y)] {
            continue;
        }
        for (xn, yn) in field.rook_neighbor_indices(x, y) {
            let alt = weight.checked_add(field[(xn, yn)]).unwrap_or(u32::MAX);
            let curr_distance = distances[&(xn, yn)];

            if alt < curr_distance {
                q.push(Point::new(xn, yn, alt));
                *distances.get_mut(&(xn, yn)).unwrap() = alt;
            }
        }
    }
    unreachable!()
}

#[aoc_generator(day15, part2)]
pub fn gen2(input: &str) -> Matrix<u32> {
    let small = gen(input);
    let mut result = Matrix::new(
        vec![0; 25 * small.len()],
        small.width() * 5,
        small.height() * 5,
    );

    for x in 0..result.width() {
        for y in 0..result.height() {
            let add_risk = ((x / small.width()) + (y / small.height())) as u32;
            let rel_x = x % small.width();
            let rel_y = y % small.height();
            let val = (add_risk + small[(rel_x, rel_y)]) % 9;

            result[(x, y)] = if val == 0 { 9 } else { val };
        }
    }

    result
}

#[aoc(day15, part2)]
pub fn second(field: &Matrix<u32>) -> u32 {
    first(field)
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
