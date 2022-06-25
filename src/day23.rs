use crate::matrix::Matrix;
use std::{
    collections::{HashMap, HashSet},
    fmt,
};

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
pub enum Color {
    A,
    B,
    C,
    D,
}

#[derive(PartialEq, Clone, Copy, Hash, Eq)]
pub enum Cell {
    Free,
    Wall,
    Someone(Color),
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cell::Free => write!(f, "."),
            Cell::Wall => write!(f, "#"),
            Cell::Someone(c) => c.fmt(f),
        }
    }
}

pub fn parse(input: &str) -> Matrix<Cell> {
    let mut items = Vec::new();

    for line in input.lines() {
        for c in line.chars() {
            items.push(match c {
                '#' | ' ' => Cell::Wall,
                '.' => Cell::Free,
                x => match x {
                    'A' => Cell::Someone(Color::A),
                    'B' => Cell::Someone(Color::B),
                    'C' => Cell::Someone(Color::C),
                    'D' => Cell::Someone(Color::D),
                    x => panic!("Invalid input: {x}"),
                },
            });
        }
    }

    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    while items.len() < width * height {
        // Finish padding the last line
        items.push(Cell::Wall);
    }

    Matrix::new(items, width, height)
}

pub fn parse_second(input: &str) -> Matrix<Cell> {
    let mut cells = parse(input);
    let extension = parse(
        "  #D#C#B#A###
  #D#B#A#C###",
    );
    cells.insert_row_at(extension, 3);
    cells
}

type Coord = (usize, usize);

#[derive(Debug)]
struct Node {
    pos: Coord,
    distance: u32,
}

fn path_length(from: Coord, to: Coord, cells: &Matrix<Cell>) -> Option<u32> {
    if from == to {
        return Some(0);
    }
    if cells[from] == Cell::Wall || cells[to] == Cell::Wall || matches!(cells[to], Cell::Someone(_))
    {
        // If you are going from or into a wall or the destination is occupied
        return None;
    }
    // Otherwise BFS !!
    let mut visited = HashSet::new();
    let mut stack: Vec<_> = cells
        .rook_neighbor_indices(from.0, from.1)
        .filter(|pos| cells[*pos] == Cell::Free)
        .map(|pos| Node { pos, distance: 1 })
        .collect();
    while let Some(node) = stack.pop() {
        visited.insert(node.pos);
        if node.pos == to {
            return Some(node.distance);
        }
        let mut new_nodes = cells
            .rook_neighbor_indices(node.pos.0, node.pos.1)
            .filter(|p| cells[*p] == Cell::Free && !visited.contains(p))
            .map(|pos| Node {
                pos,
                distance: node.distance + 1,
            })
            .collect();
        stack.append(&mut new_nodes);
    }
    None
}

fn possible_moves_and_length(
    from: (usize, usize),
    cells: &Matrix<Cell>,
) -> Vec<((usize, usize), u32)> {
    let color = match cells[from] {
        Cell::Free | Cell::Wall => return Vec::new(),
        Cell::Someone(c) => c,
    };
    let corr_col = match color {
        Color::A => 3,
        Color::B => 5,
        Color::C => 7,
        Color::D => 9,
    };
    if from.1 < 2 {
        // If the amphipod is in the hallway
        // Then its only possibility is to move to their destination room if it can

        // If the room contains a amphipod with the wrong color we can't go in
        for y in (2..cells.height() - 1).rev() {
            let cell = cells[(corr_col, y)];
            if matches!(cell, Cell::Someone(_)) && cell != Cell::Someone(color) {
                return vec![];
            }
            // Otherwise we can go in if the cell is free and if there is a path
            if cell == Cell::Free {
                if let Some(len) = path_length(from, (corr_col, y), cells) {
                    return vec![((corr_col, y), len)];
                }
            }
        }
        vec![]
    } else {
        // Otherwise the amphipod is in one of the rooms
        // We don't move if we are in the correct room and all the folk behind us also have the
        // correct color
        if from.0 == corr_col
            && (from.1..cells.height() - 1).all(|y| cells[(from.0, y)] == Cell::Someone(color))
        {
            vec![]
        } else {
            // An amphipod will never move in front of a room
            let xes = [1, 2, 4, 6, 8, 10, 11];
            xes.into_iter()
                .filter_map(|x| path_length(from, (x, 1), cells).map(|n| ((x, 1), n)))
                .collect()
        }
    }
}

fn is_finished(cells: &Matrix<Cell>) -> bool {
    for i in [3, 5, 7, 9] {
        let correct_color = match i {
            3 => Color::A,
            5 => Color::B,
            7 => Color::C,
            9 => Color::D,
            _ => unreachable!(),
        };
        for j in 2..cells.height() - 1 {
            if cells[(i, j)] != Cell::Someone(correct_color) {
                return false;
            }
        }
    }
    true
}

fn shortest_perfection(
    cells: &Matrix<Cell>,
    memory: &mut HashMap<Matrix<Cell>, Option<u32>>,
) -> Option<u32> {
    // Given a certain situation, recurse on all the possible moves
    // The cost of the current situation is the minimum of all the subsituations plus the cost of
    // the move to get there

    if let Some(value) = memory.get(cells) {
        return *value;
    }

    if is_finished(cells) {
        memory.entry(cells.clone()).or_insert(Some(0));
        return Some(0);
    }

    let mut all_amphipods_pos = Vec::new();
    for x in 0..cells.width() {
        for y in 0..cells.height() {
            if let Cell::Someone(who) = cells[(x, y)] {
                all_amphipods_pos.push(((x, y), who))
            }
        }
    }
    let mut moves_and_costs = Vec::new();
    for (pos, c) in all_amphipods_pos {
        let paths_and_lengths = possible_moves_and_length(pos, cells);
        for (path_dest, len) in paths_and_lengths {
            let mut cells_clone = cells.clone();
            cells_clone.swap(pos, path_dest);
            // println!("(((\n{cells:?}\n{cells_clone:?}\n)))\n");
            let move_cost = len
                * match c {
                    Color::A => 1,
                    Color::B => 10,
                    Color::C => 100,
                    Color::D => 1000,
                };
            if let Some(sp) = shortest_perfection(&cells_clone, memory) {
                let cost = move_cost + sp;
                moves_and_costs.push(((pos, path_dest), cost));
            }
        }
    }

    let result = moves_and_costs.into_iter().map(|(_, cost)| cost).min();
    memory.insert(cells.clone(), result);
    result
}

pub fn first(cells: &Matrix<Cell>) -> u32 {
    let mut memory = HashMap::new();
    shortest_perfection(cells, &mut memory).unwrap()
}

pub fn second(cells: &Matrix<Cell>) -> u32 {
    let mut memory = HashMap::new();
    shortest_perfection(cells, &mut memory).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn possible_paths_test() {
        let cells = parse(
            r"#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########",
        );
        assert_eq!(
            possible_moves_and_length((3, 2), &cells),
            vec![
                ((1, 1), 3),
                ((2, 1), 2),
                ((4, 1), 2),
                ((6, 1), 4),
                ((8, 1), 6),
                ((10, 1), 8),
                ((11, 1), 9)
            ],
            "From a room into hallway"
        );

        let cells = parse(
            r"#############
#...C.......#
###.#.#.#D###
  #A#D#B#A#
  #########",
        );
        assert_eq!(
            possible_moves_and_length((4, 1), &cells),
            vec![],
            "From hallway into a room when your own is busy"
        );

        let cells = parse(
            r"#############
#...C.......#
###.#.#.#D###
  #A#D#C#A#
  #########",
        );
        assert_eq!(
            possible_moves_and_length((4, 1), &cells),
            vec![((7, 2), 4)],
            "From hallway into a room when yours has friend"
        );

        let cells = parse(
            r"#############
#...C.......#
###.#.#.#D###
  #A#D#.#A#
  #########",
        );
        assert_eq!(
            possible_moves_and_length((4, 1), &cells),
            vec![((7, 3), 5)],
            "From hallway into a room when yours is free"
        );
    }

    #[test]
    fn one() {
        let cells = parse(
            "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########
",
        );
        let mut memory = HashMap::new();
        assert_eq!(shortest_perfection(&cells, &mut memory), Some(12521));
    }
}
