use crate::day16::as_number;
use crate::matrix::Matrix;
use itertools::Itertools;

#[allow(dead_code)]
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

fn compute(input: &str, t_max: u32) -> u32 {
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

    for t in 0..t_max {
        // print_mat(&mat);
        let inf_point = if code[0] { t % 2 != 0 } else { false };
        mat = mat.expand_contour(3, inf_point);
        // let mut nines = Matrix::new_default(mat.width(), mat.height());
        // Collect surrounding states
        // for x in 0..mat.width() {
        //     for y in 0..mat.height() {
        //         let x = x as isize;
        //         let y = y as isize;
        //         let nine: Vec<_> = [y - 1, y, y + 1]
        //             .iter()
        //             .cartesian_product([x - 1, x, x + 1].iter())
        //             .map(|(&xx, &yy)| *mat.get(xx, yy).unwrap_or(&inf_point))
        //             .collect();
        //         nines[(x as usize, y as usize)] = nine;
        //     }
        // }
        let items = (0..mat.width())
            .cartesian_product(0..mat.height())
            .map(|(x, y)| {
                let nine: [bool; 9] = [y - 1, y, y + 1]
                    .iter()
                    .cartesian_product([x - 1, x, x + 1].iter())
                    .map(|(&xx, &yy)| *mat.get(xx as isize, yy as isize).unwrap_or(&inf_point))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();
                nine
            });
        let mut nines = Matrix::new(items, mat.width(), mat.height());
        // Apply code to image
        for x in 0..mat.width() {
            for y in 0..mat.height() {
                let nine = std::mem::take(&mut nines[(x, y)]);
                let code_idx = as_number(nine.iter());
                let new_state: bool = code[code_idx as usize];
                mat[(x, y)] = new_state;
            }
        }
        // println!();
        // println!();
    }
    // print_mat(&mat);
    mat.vec.iter().filter(|b| **b).count() as u32
}

pub fn first(input: &str) -> u32 {
    compute(input, 2)
}

pub fn second(input: &str) -> u32 {
    compute(input, 50)
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
