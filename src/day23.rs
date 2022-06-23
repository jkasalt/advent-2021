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
    println!("{cells:?},,,{extension:?}");
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
    if from.1 < 2 {
        // If the amphipod is in the hallway
        // Then its only possibility is to move to their destination room if it can
        let first_destination = match color {
            Color::A => (3, 3),
            Color::B => (5, 3),
            Color::C => (7, 3),
            Color::D => (9, 3),
        };
        let second_destination = (first_destination.0, first_destination.1 - 1);
        if let Some(n) = path_length(from, first_destination, cells) {
            vec![(first_destination, n)]
        } else if let Some(n) = path_length(from, second_destination, cells) {
            if cells[first_destination] == cells[from] {
                vec![(second_destination, n)]
            } else {
                vec![]
            }
        } else {
            vec![]
        }
    } else {
        // Otherwise the amphipod is in one of the rooms
        // If an amphipod is not already in the correct room and its not blocking someone who wants to
        // move then we proceed
        let correct_column = match color {
            Color::A => 3,
            Color::B => 5,
            Color::C => 7,
            Color::D => 9,
        };
        let blocking_someone = from.1 == 2
            && matches!(cells[(correct_column, 3)], Cell::Someone(_))
            && cells[(correct_column, 3)] != cells[from];
        let snugly_parked = from == (correct_column, 3);
        if from.0 == correct_column && (snugly_parked || !blocking_someone) {
            return vec![];
        }
        // An amphipod will never move in front of a room
        let xes = vec![1, 2, 4, 6, 8, 10, 11];
        xes.into_iter()
            .filter_map(|x| {
                if let Some(n) = path_length(from, (x, 1), cells) {
                    Some(((x, 1), n))
                } else {
                    None
                }
            })
            .collect()
    }
}

fn is_finished(cells: &Matrix<Cell>) -> bool {
    let perfect = parse(
        // This is inefficient because we parse it again and again each time, we should wrap it
        // into a static Lazy<..>
        r"#############
#...........#
###A#B#C#D###
  #A#B#C#D#
  #########",
    );
    *cells == perfect
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
