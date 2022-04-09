use crate::day16::as_number;
use crate::matrix::Matrix;

fn print_mat(mat: &Matrix<bool>) {
    for y in 0..mat.height() {
        for x in 0..mat.width() {
            match mat.get(x as isize, y as isize).unwrap() {
                true => print!("#"),
                false => print!("."),
            };
        }
        println!();
    }
}

pub fn first(input: &str) -> u32 {
    let (code, image) = input.split_once("\n\n").expect("valid input");
    let code: Vec<_> = code
        .chars()
        .filter_map(|c| match c {
            '.' => Some(false),
            '#' => Some(true),
            _ => None,
        })
        .collect();
    debug_assert!(code.len() == 512);

    let t_max = 2;
    let width = image.lines().next().unwrap().len();
    let height = image.lines().count();
    let mut mat = Matrix::new(
        image.chars().filter_map(|c| match c {
            '.' => Some(false),
            '#' => Some(true),
            _ => None,
        }),
        width,
        height,
    );

    for _ in 0..t_max {
        print_mat(&mat);
        mat = mat.expand_contour(1);
        let mut nines = Matrix::new_default(mat.width(), mat.height());
        // Collect surrounding states
        for x in 0..mat.width() {
            for y in 0..mat.height() {
                let x = x as isize;
                let y = y as isize;
                let mut nine = Vec::new();
                for yy in [y - 1, y, y + 1] {
                    for xx in [x - 1, x, x + 1] {
                        let bit = mat.get(xx, yy).unwrap_or(&false);
                        nine.push(*bit);
                    }
                }
                nines[(x as usize, y as usize)] = nine;
            }
        }
        // Apply code to image
        for x in 0..mat.width() {
            for y in 0..mat.height() {
                let nine = nines[(x, y)].clone();
                let code_idx = as_number(nine.iter());
                let new_state: bool = code[code_idx as usize];
                mat[(x, y)] = new_state;
            }
        }
        println!();
        println!();
    }
    print_mat(&mat);
    mat.vec.iter().filter(|b| **b).count() as u32
}

pub fn second(_input: &str) -> u32 {
    1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1() {
        let input = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##
#..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###
.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.
.#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....
.#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..
...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....
..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";
        assert_eq!(first(input), 35)
    }
}
