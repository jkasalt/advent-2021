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
    fn paths(
        &self,
        start: &str,
        end: &str,
        history: &[String],
        count: &mut u32,
        small_max: usize,
    ) -> u32 {
        // dbg!(&history, &count);

        // println!(".....");
        if start == end {
            // println!("Reached end.. count: {}", *count + 1);
            // println!("Path: {:?}", history);
            return 1;
        }

        for cave_name in self.adjacent_of(start) {
            let prev_occu = history
                .iter()
                .filter(|&h_cave| h_cave.eq(&cave_name))
                .count();
            if (cave_name == "start" || cave_name == "end") && history.contains(&cave_name) {
                continue;
            }
            if cave_name.to_ascii_lowercase() == cave_name && prev_occu >= small_max {
                continue;
            }
            // dbg!(&cave_name, prev_occu);
            // println!("Moving, {} -> {}", start, cave_name);
            let hist_new = {
                let mut h = history.to_owned();
                h.push(cave_name.clone());
                h
            };
            *count += self.paths(&cave_name, "end", &hist_new, count, small_max);
        }
        if start == "start" {
            return *count;
        }
        0
    }

    fn adjacent_of(&self, cave_name: &str) -> Vec<String> {
        let cave_idx = self
            .cave_set
            .iter()
            .position(|nei| nei.name == cave_name)
            .unwrap();
        self.adjacencies
            .row(cave_idx)
            .iter()
            .enumerate()
            .filter_map(|(i, elem)| if *elem == 1 { Some(i) } else { None })
            .map(|i| self.cave_set[i].name.clone())
            .collect::<Vec<_>>()
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
fn first(cave_system: &CaveSystem) -> u32 {
    cave_system.paths("start", "end", &["start".to_string()], &mut 0, 1)
}


#[aoc(day12, part2)]
fn second(cave_system: &CaveSystem) -> u32 {
    cave_system.paths("start", "end", &["start".to_string()], &mut 0, 2)
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
        assert_eq!(first(&sample), 10);
    }

    #[test]
    fn two() {
        let sample = gen("start-A
start-b
A-c
A-b
b-d
A-end
b-end");
        assert_eq!(second(&sample), 54);
    }
}
