use anyhow::{anyhow, Error};
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;
use std::{cmp, ops};

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

fn line(p1: Point, p2: Point, try_diag: bool) -> Box<dyn Iterator<Item = Point>> {
    if p1.x == p2.x {
        let (y1, y2) = (cmp::min(p1.y, p2.y), cmp::max(p1.y, p2.y));
        Box::new((y1..=y2).map(move |y| Point::new(p1.x, y)))
    } else if p1.y == p2.y {
        let (x1, x2) = (cmp::min(p1.x, p2.x), cmp::max(p1.x, p2.x));
        Box::new((x1..=x2).map(move |x| Point::new(x, p1.y)))
    } else if try_diag {
        line_diag(p1, p2)
    } else {
        Box::new(std::iter::empty())
    }
}

fn line_diag(p1: Point, p2: Point) -> Box<dyn Iterator<Item = Point>> {
    let Point { x: ax, y: ay } = &p1 - &p2;

    // Detect 45 degree angle
    if ax.abs() == ay.abs() {
        // Get the ones with smallest and biggest x
        let pp = cmp::min_by_key(&p1, &p2, |a| a.x);
        let pg = cmp::max_by_key(&p1, &p2, |a| a.x);

        // Negative slope
        if pp.y > pg.y {
            let line = (pp.x..=pg.x)
                .zip((pg.y..=pp.y).rev())
                .map(|(x, y)| Point::new(x, y));
            return Box::new(line);
        }
        // Positive slope
        else {
            let line = (pp.x..=pg.x)
                .zip(pp.y..=pg.y)
                .map(|(x, y)| Point::new(x, y));
            return Box::new(line);
        }
    }
    Box::new(std::iter::empty())
}

impl FromStr for Point {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (s1, s2) = s.split_once(',').ok_or_else(|| anyhow!("need comma"))?;
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

pub fn first(input: &str) -> u32 {
    let mut map = HashMap::new();
    let re = Regex::new(r"(\d+,\d+) -> (\d+,\d+)").unwrap();
    for cap in re.captures_iter(input) {
        line(cap[1].parse().unwrap(), cap[2].parse().unwrap(), false)
            .for_each(|p| *map.entry(p).or_insert(0) += 1);
    }
    map.values().filter(|&&v| v >= 2).count() as u32
}

pub fn second(input: &str) -> u32 {
    let mut map = HashMap::new();
    let re = Regex::new(r"(\d+,\d+) -> (\d+,\d+)").unwrap();
    for cap in re.captures_iter(input) {
        line(cap[1].parse().unwrap(), cap[2].parse().unwrap(), true)
            .for_each(|p| *map.entry(p).or_insert(0) += 1);
    }
    map.values().filter(|&&v| v >= 2).count() as u32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one() {
        let input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
        assert_eq!(first(input), 5);
    }

    #[test]
    fn two() {
        let input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
        assert_eq!(second(input), 12);
    }
}
