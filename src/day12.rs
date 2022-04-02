use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct CaveSystem {
    adjacencies: HashMap<String, Vec<String>>,
}

impl CaveSystem {
    fn paths(
        &self,
        start: &str,
        mut history: Vec<String>,
        count: &mut u64,
        part1: bool,
        visited_twice: bool,
    ) -> u64 {
        history.push(start.to_string());
        for cave_name in &self.adjacencies[start] {
            if cave_name == "start" {
                continue;
            } else if cave_name == "end" {
                *count += 1;
                continue;
            }
            // If it's a small cave and we already visited it
            if cave_name.to_lowercase() == *cave_name && history.contains(cave_name) {
                if part1 {
                    continue;
                }
                // If we already visited some cave twice skip this one
                if visited_twice {
                    continue;
                }
                // Otherwise visit this one for the second time
                *count += self.paths(cave_name, history.clone(), count, part1, true);
            } else {
                // Otherwise proceed as normal
                *count += self.paths(cave_name, history.clone(), count, part1, visited_twice);
            }
        }
        if start == "start" {
            return *count;
        }
        0
    }
}

#[aoc_generator(day12)]
pub fn gen(input: &str) -> CaveSystem {
    let re = Regex::new(r"(\w+)-(\w+)").unwrap();
    let mut adjacencies = HashMap::new();
    for cap in re.captures_iter(input) {
        adjacencies
            .entry(cap[1].to_string())
            .or_insert_with(Vec::new)
            .push(cap[2].to_string());
        adjacencies
            .entry(cap[2].to_string())
            .or_insert_with(Vec::new)
            .push(cap[1].to_string());
    }

    CaveSystem { adjacencies }
}

#[aoc(day12, part1)]
pub fn first(cave_system: &CaveSystem) -> u64 {
    cave_system.paths("start", Vec::new(), &mut 0, true, false)
}

#[aoc(day12, part2)]
pub fn second(cave_system: &CaveSystem) -> u64 {
    cave_system.paths("start", Vec::new(), &mut 0, false, false)
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
        assert_eq!(second(&sample), 36);
    }

    #[test]
    fn more() {
        let i1 = gen("dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc");
        let i2 = gen("fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW");
        assert_eq!(second(&i1), 103);
        assert_eq!(second(&i2), 3509);
    }
}
