use crate::matrix::Matrix;
use colored::Colorize;
use std::fmt;

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
        .map(Octopus::new);

    Matrix::new(vec, width, height)
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
        let mut updated_pos = (0..mat.width()) // Initialize with all positions at the start
            .flat_map(|x| (0..mat.height()).map(move |y| (x, y)))
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
            for x in 0..mat.width() {
                for y in 0..mat.height() {
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
            let mat = Matrix::new(vec, 3, 3);
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
