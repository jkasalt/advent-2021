use anyhow::{anyhow, Error};
use regex::Regex;
use std::{cmp, ops};
use std::{collections::HashMap, str::FromStr};

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

fn line(p1: Point, p2: Point, try_diag: bool) -> Vec<Point> {
    if p1.x == p2.x {
        let (y1, y2) = (cmp::min(p1.y, p2.y), cmp::max(p1.y, p2.y));
        (y1..=y2).map(|y| Point::new(p1.x, y)).collect()
    } else if p1.y == p2.y {
        let (x1, x2) = (cmp::min(p1.x, p2.x), cmp::max(p1.x, p2.x));
        (x1..=x2).map(|x| Point::new(x, p1.y)).collect()
    } else if try_diag {
        line_diag(p1, p2)
    } else {
        Vec::new()
    }
}

fn line_diag(p1: Point, p2: Point) -> Vec<Point> {
    let Point { x: ax, y: ay } = &p1 - &p2;

    // Detect 45 degree angle
    if ax.abs() == ay.abs() {
        // Get the ones with smallest and biggest x
        let pp = cmp::min_by_key(&p1, &p2, |a| a.x);
        let pg = cmp::max_by_key(&p1, &p2, |a| a.x);

        // Negative slope
        if (p1.x < p2.x && p1.y > p2.y) || (p1.x > p2.x && p1.y < p2.y) {
            let line = (pp.x..=pg.x)
                .zip((pg.y..=pp.y).rev())
                .map(|(x, y)| Point::new(x, y))
                .collect();
            return line;
        }
        // Positive slope
        else {
            let line = (pp.x..=pg.x)
                .zip(pp.y..=pg.y)
                .map(|(x, y)| Point::new(x, y))
                .collect();
            return line;
        }
    }
    Vec::new()
}

impl FromStr for Point {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (s1, s2) = s.split_once(',').ok_or(anyhow!("need comma"))?;
        Ok(Self {
            x: s1.parse()?,
            y: s2.parse()?,
        })
    }
}

impl ops::Sub<&Point> for &Point {
    type Output = Point;
    fn sub(self, rhs: &Point) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[aoc_generator(day5, part1)]
fn gen(input: &str) -> HashMap<Point, u32> {
    let mut map = HashMap::new();
    let re = Regex::new(r"(\d+,\d+) -> (\d+,\d+)").unwrap();
    let mut lines = Vec::new();
    for cap in re.captures_iter(input) {
        lines.push(line(
            cap[1].parse().unwrap(),
            cap[2].parse().unwrap(),
            false,
        ));
    }

    for point in lines.concat() {
        *map.entry(point).or_insert(0) += 1;
    }

    map
}

#[aoc_generator(day5, part2)]
fn gen2(input: &str) -> HashMap<Point, u32> {
    let mut map = HashMap::new();
    let re = Regex::new(r"(\d+,\d+) -> (\d+,\d+)").unwrap();
    let mut lines = Vec::new();
    for cap in re.captures_iter(input) {
        lines.push(line(cap[1].parse().unwrap(), cap[2].parse().unwrap(), true));
    }

    for point in lines.concat() {
        *map.entry(point).or_insert(0) += 1;
    }

    map
}

#[aoc(day5, part1)]
fn first(map: &HashMap<Point, u32>) -> u32 {
    map.values().filter(|&&v| v >= 2).count() as u32
}

#[aoc(day5, part2)]
fn second(map: &HashMap<Point, u32>) -> u32 {
    map.values().filter(|&&v| v >= 2).count() as u32
}

#[cfg(test)]
mod test {
    use super::*;

    fn input1() -> HashMap<Point, u32> {
        crate::day5::gen(
            "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2",
        )
    }

    fn input2() -> HashMap<Point, u32> {
        crate::day5::gen2(
            "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2",
        )
    }

    #[test]
    fn one() {
        let map = input1();
        assert_eq!(first(&map), 5);
    }

    #[test]
    fn two() {
        let map = input2();
        assert_eq!(second(&map), 12);
    }
}
