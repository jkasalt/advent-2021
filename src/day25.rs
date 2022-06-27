use std::fmt::Write;

use crate::matrix::Matrix;

#[derive(PartialEq, Clone)]
pub enum Cell {
    Down,
    Right,
    Free,
}

impl std::fmt::Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Cell::Down => 'v',
            Cell::Right => '>',
            Cell::Free => '.',
        };
        f.write_char(c)
    }
}

pub fn gen(input: &str) -> Matrix<Cell> {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let items = input.chars().filter_map(|c| match c {
        '.' => Some(Cell::Free),
        'v' => Some(Cell::Down),
        '>' => Some(Cell::Right),
        _ => None,
    });
    Matrix::new(items, width, height)
}

fn apply_move(cells: &mut Matrix<Cell>, kind: Cell) -> bool {
    let mut has_moved = false;

    let mut will_swap = Vec::new();
    for x in 0..cells.width() {
        for y in 0..cells.height() {
            let (x, y) = (x as isize, y as isize);
            if cells.get(x, y) == Some(&kind) {
                let (x, y) = (x as usize, y as usize);
                let new_pos = match kind {
                    Cell::Down => (x, (y + 1) % cells.height()),
                    Cell::Right => ((x + 1) % cells.width(), y),
                    Cell::Free => continue,
                };
                // println!(
                //     "({x}, {y}) which is '{kind:?}' looking at {new_pos:?} which is '{:?}'",
                //     cells[(new_pos)]
                // );
                if cells[new_pos] == Cell::Free {
                    has_moved = true;
                    will_swap.push(((x, y), new_pos));
                }
            }
        }
    }
    for (a, b) in will_swap {
        cells.swap(a, b);
    }
    has_moved
}

pub fn first(cells: &Matrix<Cell>) -> usize {
    let mut cells = cells.clone();

    let mut count = 0;
    loop {
        // println!("{cells:?}");
        count += 1;

        // println!("Move right-facing cucumbers...");
        let has_moved_right = apply_move(&mut cells, Cell::Right);
        // println!("Move down-facing cucumbers...");
        let has_moved_down = apply_move(&mut cells, Cell::Down);

        if !(has_moved_down || has_moved_right) {
            break;
        }
    }
    count
}

pub fn second(_cells: &Matrix<Cell>) -> usize {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one() {
        let input = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";
        assert_eq!(first(&gen(input)), 58);
    }
}
