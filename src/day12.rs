use crate::day11::Matrix;
use regex::Regex;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
struct Cave {
    name: String,
    small: bool,
}

impl Cave {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            small: name.to_lowercase() == name,
        }
    }
}

#[derive(Debug, Clone)]
struct CaveSystem {
    cave_set: Vec<Cave>,
    adjacencies: Matrix<u8>,
}

impl CaveSystem {
    fn paths(&self, start: &str, end: &str) {
        let start_idx = self
            .cave_set
            .iter()
            .position(|cave| cave.name == start)
            .unwrap();
        let end_idx = self
            .cave_set
            .iter()
            .position(|cave| cave.name == end)
            .unwrap();
        let start_adj = self
            .adjacencies
            .row(start_idx)
            .iter()
            .enumerate()
            .filter_map(|(i, elem)| if *elem == 1 { Some(i) } else { None })
            .collect::<Vec<_>>();

        let mut head = start_idx;
        let mut stack = Vec::new();
        let mut paths = Vec::new();
        loop {
            while head != end_idx {
                // Mark current node as visited by adding it to the stack of parents
                println!("head: {}", head);
                stack.push(head);
                // Move to the first adjacent cave we find
                for (i, elem) in self.adjacencies.row(head).iter().enumerate() {
                    if *elem == 1 && !(self.cave_set[i].small && stack.contains(&i)) {
                        head = i;
                    }
                }
            }
            // Then we have found a path
            println!("{:?}", stack);

            if stack == start_adj && head == start_idx {
                break;
            }
        }
    }
}

#[aoc_generator(day12)]
fn gen(input: &str) -> CaveSystem {
    let re = Regex::new(r"(\w+)-(\w+)").unwrap();
    let mut vertices = HashSet::new();
    for cap in re.captures_iter(input) {
        vertices.insert(cap[1].to_string());
        vertices.insert(cap[2].to_string());
    }
    let h = vertices.len();
    let vec: Vec<_> = vertices.into_iter().collect();
    let mut mat = Matrix::new(vec![0; h * h], h, h);

    for cap in re.captures_iter(input) {
        let p1 = vec.iter().position(|name| name == &cap[1]).unwrap();
        let p2 = vec.iter().position(|name| name == &cap[2]).unwrap();
        mat[(p1, p2)] = 1;
        mat[(p2, p1)] = 1;
    }

    CaveSystem {
        cave_set: vec.into_iter().map(|name| Cave::new(&name)).collect(),
        adjacencies: mat,
    }
}

#[aoc(day12, part1)]
fn first(cave_system: &CaveSystem) -> u64 {
    cave_system.paths("start", "end");
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one() {
        let sample = gen("start-A
start-b
A-c
A-b
b-d
A-end
b-end");
        dbg!(&sample);
        first(&sample);
        panic!();
    }
}
