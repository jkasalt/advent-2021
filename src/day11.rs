use colored::Colorize;
use std::fmt;
use std::ops;

#[derive(Clone)]
struct Matrix<T> {
    vec: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Matrix<T> {
    #[allow(dead_code)]
    fn get(&self, x: isize, y: isize) -> Option<&T> {
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

    #[allow(dead_code)]
    fn neighbors_of(&self, x: usize, y: usize) -> [Option<&T>; 8] {
        let x: isize = x.try_into().unwrap();
        let y: isize = y.try_into().unwrap();

        let top = self.get(x, y + 1);
        let topright = self.get(x + 1, y + 1);
        let right = self.get(x + 1, y);
        let botright = self.get(x + 1, y - 1);
        let bot = self.get(x, y - 1);
        let botleft = self.get(x - 1, y - 1);
        let left = self.get(x - 1, y);
        let topleft = self.get(x - 1, y + 1);

        [top, topright, right, botright, bot, botleft, left, topleft]
    }

    fn neighbor_indices(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut res = Vec::new();
        let mut xes = vec![x];
        let mut yes = vec![y];
        if y > 0 {
            yes.push(y - 1);
        }
        if y < self.height - 1 {
            yes.push(y + 1)
        }
        if x > 0 {
            xes.push(x - 1);
        }
        if x < self.width - 1 {
            xes.push(x + 1);
        }
        for yy in yes {
            for &xx in &xes {
                if xx == x && yy == y {
                    continue;
                }
                res.push((xx, yy));
            }
        }
        res
    }
}

impl fmt::Debug for Matrix<Octopus> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut count = 0;
        writeln!(f)?;
        for oct in &self.vec {
            write!(f, "{:?}", oct)?;
            count += 1;
            if count == self.width {
                count = 0;
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl<T> ops::Index<(usize, usize)> for Matrix<T> {
    type Output = T;

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
impl<T> ops::IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        if x > self.width - 1 || y > self.height - 1 {
            panic!(
                "Index ({}, {}) out of range for Matrix with size ({}, {})",
                y, x, self.height, self.width
            );
        }
        &mut self.vec[x + y * self.width]
    }
}

#[derive(Clone)]
struct Octopus {
    energy: u32,
    has_flashed: bool,
}

impl Octopus {
    pub fn new(energy: impl Into<u32>) -> Self {
        Self {
            energy: energy.into(),
            has_flashed: false,
        }
    }
}

impl fmt::Debug for Octopus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.energy != 0 {
            write!(f, "{}", self.energy.to_string().bright_black())
        } else {
            write!(f, "{}", self.energy.to_string().bright_white())
        }
    }
}

#[aoc_generator(day11)]
fn gen(input: &str) -> Matrix<Octopus> {
    let width = input.chars().position(|c| c == '\n').unwrap();
    let height = input.lines().count();
    let vec = input
        .chars()
        .filter_map(|ch| ch.to_digit(10))
        .map(Octopus::new)
        .collect();

    Matrix { width, height, vec }
}

#[aoc(day11, part1)]
fn first(mat: &Matrix<Octopus>) -> u64 {
    let mut mat = (*mat).clone();
    let mut count = 0;
    for _t in 0..100 {
        // Increment all the energies and reset flash status
        for oct in &mut mat.vec {
            oct.has_flashed = false;
            oct.energy += 1;
        }
        // Check for flashes
        let mut new_flashes = true;
        let mut updated_pos = (0..mat.width) // Initialize with all positions at the start
            .flat_map(|x| (0..mat.height).map(move |y| (x, y)))
            .collect::<Vec<_>>();
        while new_flashes {
            new_flashes = false;
            let mut new_flash_indices = Vec::new();
            for (x, y) in updated_pos {
                let oct = &mut mat[(x, y)];
                if oct.energy > 9 {
                    oct.energy = 0;
                    new_flashes = true;
                    oct.has_flashed = true;
                    count += 1;
                    for pos in mat.neighbor_indices(x, y) {
                        // Write down which octopi will have energy increased
                        // because of a neighbor flash
                        if !mat[pos].has_flashed {
                            new_flash_indices.push(pos);
                        }
                    }
                }
            }
            updated_pos = new_flash_indices;
            // Increase energy of those who were flashed
            for &pos in &updated_pos {
                let oct = &mut mat[pos];
                if !oct.has_flashed {
                    oct.energy += 1;
                }
            }
        }
        println!("{}", _t);
    }
    count
}

#[aoc(day11, part2)]
fn second(mat: &Matrix<Octopus>) -> u64 {
    let mut mat = (*mat).clone();
    let mut t: u64 = 0;
    'outer: loop {
        t += 1;
        // Increment all the energies and reset flash status
        for oct in &mut mat.vec {
            oct.has_flashed = false;
            oct.energy += 1;
        }
        // Check for flashes
        let mut new_flashes = true;
        while new_flashes {
            new_flashes = false;
            let mut new_flash_indices = Vec::new();
            for x in 0..mat.width {
                for y in 0..mat.height {
                    let oct = &mut mat[(x, y)];
                    if oct.energy > 9 {
                        oct.energy = 0;
                        new_flashes = true;
                        oct.has_flashed = true;
                        for pos in mat.neighbor_indices(x, y) {
                            // Write down which octopi will have energy increased becasue of a neighbor flash
                            new_flash_indices.push(pos);
                        }
                    }
                }
            }
            // Increase energy of those who were flashed
            for pos in new_flash_indices {
                let oct = &mut mat[pos];
                if !oct.has_flashed {
                    mat[pos].energy += 1;
                }
            }
        }
        //println!("{:?}", mat);
        //thread::sleep(Duration::from_millis(70));
        let sync_me = mat[(0, 0)].energy;
        if mat.vec.iter().all(|oct| oct.energy == sync_me) {
            break 'outer;
        }
    }
    t
}

#[cfg(test)]
mod test {
    use super::*;
    mod matrix {
        use crate::day11::Matrix;
        #[test]
        fn neighbor_indices() {
            let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
            let mat = Matrix {
                vec,
                width: 3,
                height: 3,
            };
            assert_eq!(
                mat.neighbor_indices(1, 1),
                [
                    (0, 1),
                    (2, 1),
                    (1, 0),
                    (0, 0),
                    (2, 0),
                    (1, 2),
                    (0, 2),
                    (2, 2)
                ]
            );

            assert_eq!(mat.neighbor_indices(0, 0), [(1, 0), (0, 1), (1, 1)]);
        }
    }

    #[test]
    fn one() {
        let mat = gen("5483143223
        2745854711
        5264556173
        6141336146
        6357385478
        4167524645
        2176841721
        6882881134
        4846848554
        5283751526");

        assert_eq!(first(&mat), 1656);
    }
    #[test]
    fn two() {
        let mat = gen("5483143223
        2745854711
        5264556173
        6141336146
        6357385478
        4167524645
        2176841721
        6882881134
        4846848554
        5283751526");

        assert_eq!(second(&mat), 195);
    }
}
