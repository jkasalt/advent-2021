use crate::matrix::Matrix;

#[aoc_generator(day15)]
fn gen(input: &str) -> Matrix<u32> {
    let items = input.chars().filter_map(|c| c.to_digit(10));
    let width = input.chars().position(|c| c == '\n').unwrap();
    let height = input.lines().count();

    Matrix::new(items, width, height)
}

#[aoc(day15, part1)]
fn first(field: &Matrix<u32>) -> u64 {
    field.risk_from(0, 0)
}

trait Risk {
    fn risk_from(&self, x: usize, y: usize) -> u64;
}

impl Risk for Matrix<u32> {
    fn risk_from(&self, x: usize, y: usize) -> u64 {
        if x == self.width() && y == self.height() {
            self[(x, y)] as u64
        } else {
            self[(x, y)] as u64
                + self
                    .neighbor_indices(x, y)
                    .iter()
                    .map(|(xn, yn)| self.risk_from(*xn, *yn))
                    .min()
                    .unwrap()
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
        assert_eq!(first(&sample()), 40)
    }
}
