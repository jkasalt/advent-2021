use anyhow::Context;
use itertools::Itertools;
use std::iter::Sum;
use std::ops::{Add, Index, IndexMut};
use std::str::FromStr;

#[derive(Debug)]
pub enum Elem {
    Com(Box<(Elem, Elem)>),
    Num(u32),
}

impl Clone for Elem {
    fn clone(&self) -> Self {
        match self {
            Elem::Com(b) => Elem::Com(b.clone()),
            Elem::Num(n) => Elem::Num(*n),
        }
    }
}

impl PartialEq for Elem {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Com(_), Self::Num(_)) => false,
            (Self::Num(_), Self::Com(_)) => false,
            (Self::Num(a), Self::Num(b)) => a == b,
            (Self::Com(box1), Self::Com(box2)) => *box1 == *box2,
        }
    }
}

impl ToString for Elem {
    fn to_string(&self) -> String {
        match self {
            Elem::Num(n) => n.to_string(),
            Elem::Com(b) => format!("[{},{}]", b.0.to_string(), b.1.to_string()),
        }
    }
}

impl FromStr for Elem {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(ss) = s.strip_prefix('[') {
            let mut rel_depth = 0;
            let comma_pos = ss
                .chars()
                .position(|c2| {
                    match c2 {
                        ',' if rel_depth == 0 => return true,
                        '[' => rel_depth += 1,
                        ']' => rel_depth -= 1,
                        _ => {}
                    };
                    false
                })
                .context("Failed to find correspoding comma")?;
            let mut rel_depth = 0;
            let closing_pos = ss
                .chars()
                .position(|c2| {
                    match c2 {
                        ']' if rel_depth == 0 => return true,
                        '[' => rel_depth += 1,
                        ']' => rel_depth -= 1,
                        _ => {}
                    };
                    false
                })
                .context("Failed to find correspoding closing comma")?;
            let (subs1, subs2) = ss[..closing_pos].split_at(comma_pos + 1);
            let elem1 = Self::from_str(subs1)?;
            let elem2 = Self::from_str(subs2)?;
            return Ok(Self::Com(Box::new((elem1, elem2))));
        }
        if let Ok(n) = s.trim_end_matches(',').parse() {
            return Ok(Self::Num(n));
        }
        Err(anyhow::anyhow!("problem with string \"{}\"", s))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Dir {
    Left,
    Right,
}

impl Dir {
    fn opposite(&self) -> Self {
        match self {
            Dir::Left => Dir::Right,
            Dir::Right => Dir::Left,
        }
    }
}

impl Index<&[Dir]> for Elem {
    type Output = Elem;
    fn index(&self, index: &[Dir]) -> &Self::Output {
        match self {
            Elem::Num(_) => self,
            Elem::Com(_) if index.is_empty() => self,
            Elem::Com(b) => match index[0] {
                Dir::Left => &b.0[&index[1..]],
                Dir::Right => &b.1[&index[1..]],
            },
        }
    }
}

impl IndexMut<&[Dir]> for Elem {
    fn index_mut(&mut self, index: &[Dir]) -> &mut Self {
        match self {
            Elem::Num(_) => self,
            Elem::Com(_) if index.is_empty() => self,
            Elem::Com(b) => match index[0] {
                Dir::Left => &mut b.0[&index[1..]],
                Dir::Right => &mut b.1[&index[1..]],
            },
        }
    }
}

impl Add for Elem {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Elem::Com(Box::new((self, rhs))).reduce()
    }
}

impl Sum for Elem {
    fn sum<I>(mut iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        let mut base = iter.next().unwrap();
        for elem in iter {
            base = base + elem;
        }
        base
    }
}

impl Elem {
    pub fn reduce(mut self) -> Self {
        loop {
            if let Some(dirs) = self.find_nested(4) {
                self.explode(&dirs);
            } else if let Some(dirs) = self.find_splittable(10) {
                self.split(&dirs);
            } else {
                break;
            }
        }
        self
    }

    fn is_simple(&self) -> bool {
        if let Elem::Com(b) = self {
            if b.0.is_num() && b.1.is_num() {
                return true;
            }
        }
        false
    }

    fn is_num(&self) -> bool {
        match self {
            Elem::Com(_) => false,
            Elem::Num(_) => true,
        }
    }

    fn find_nested(&self, how_much: i32) -> Option<Vec<Dir>> {
        self.has_nested_inner(how_much, &Vec::new())
    }

    fn has_nested_inner(&self, how_much: i32, dirs: &[Dir]) -> Option<Vec<Dir>> {
        if self.is_simple() && how_much <= 0 {
            Some(dirs.to_owned())
        } else if let Elem::Com(b) = self {
            let mut d0 = dirs.to_owned();
            d0.push(Dir::Left);
            b.0.has_nested_inner(how_much - 1, &d0).or_else(|| {
                let mut d1 = dirs.to_owned();
                d1.push(Dir::Right);
                b.1.has_nested_inner(how_much - 1, &d1)
            })
        } else {
            None
        }
    }

    fn carry(&mut self, direc: Dir, n: u32, start: &[Dir]) {
        let mut dirs = start.to_owned();
        loop {
            let last_dir = match dirs.pop() {
                None => return,
                Some(d) => d,
            };
            if last_dir != direc {
                dirs.push(direc);
                dirs.push(direc.opposite());
                loop {
                    if let Elem::Num(m) = self[&dirs] {
                        self[&dirs] = Elem::Num(n + m);
                        return;
                    }
                    dirs.push(direc.opposite());
                }
            }
        }
    }

    pub fn explode(&mut self, dirs: &[Dir]) {
        // Do the moving
        let (l, r) = &self[dirs].unwrap();
        self.carry(Dir::Left, *l, dirs);
        self.carry(Dir::Right, *r, dirs);

        // Change the exploded element to 0
        self[dirs] = Elem::Num(0);
    }

    /// Panics if !self.is_simple()
    fn unwrap(&self) -> (u32, u32) {
        match self {
            Elem::Num(_) => panic!("Called unwrap on {:?}", self),
            Elem::Com(b) => {
                if let Elem::Num(n0) = b.0 {
                    if let Elem::Num(n1) = b.1 {
                        return (n0, n1);
                    }
                }
                panic!("Called unwrap on {:?}", self);
            }
        }
    }

    fn find_splittable(&self, how_much: u32) -> Option<Vec<Dir>> {
        self.has_splittable_inner(how_much, &Vec::new())
    }

    fn has_splittable_inner(&self, how_much: u32, dirs: &[Dir]) -> Option<Vec<Dir>> {
        match self {
            Elem::Num(n) => {
                if *n >= how_much {
                    Some(dirs.to_owned())
                } else {
                    None
                }
            }
            Elem::Com(b) => {
                let mut d0 = dirs.to_owned();
                d0.push(Dir::Left);
                b.0.has_splittable_inner(how_much, &d0).or_else(|| {
                    let mut d1 = dirs.to_owned();
                    d1.push(Dir::Right);
                    b.1.has_splittable_inner(how_much, &d1)
                })
            }
        }
    }

    fn split(&mut self, dirs: &[Dir]) {
        if let Elem::Num(n) = self[dirs] {
            let left = Elem::Num((n as f32 / 2.0).floor() as u32);
            let right = Elem::Num((n as f32 / 2.0).ceil() as u32);
            self[dirs] = Elem::Com(Box::new((left, right)));
        }
    }

    pub fn magnitude(&self) -> u128 {
        match self {
            Self::Num(n) => *n as u128,
            Self::Com(b) => 3 * b.0.magnitude() + 2 * b.1.magnitude(),
        }
    }
}

pub fn first(input: &str) -> u128 {
    input
        .lines()
        .map(|l| Elem::from_str(l).unwrap())
        .sum::<Elem>()
        .magnitude()
}

pub fn second(input: &str) -> u128 {
    let elems: Vec<_> = input.lines().map(|l| Elem::from_str(l).unwrap()).collect();
    elems
        .iter()
        .cartesian_product(elems.iter())
        .map(|(e1, e2)| (e1.clone() + e2.clone()).magnitude())
        .max()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test]
    fn parsing() {
        let s1 = "[1,2]";
        assert_eq!(
            Elem::from_str(s1).unwrap(),
            Elem::Com(Box::new((Elem::Num(1), Elem::Num(2))))
        );

        let s2 = "[9,[8,7]]";
        assert_eq!(
            Elem::from_str(s2).unwrap(),
            Elem::Com(Box::new((
                Elem::Num(9),
                Elem::Com(Box::new((Elem::Num(8), Elem::Num(7))))
            ))),
        );
    }

    #[test]
    fn nested() {
        let elem1 = Elem::from_str("[[[[[9,8],1],2],3],4]").unwrap();
        assert_eq!(
            elem1.find_nested(4),
            Some(vec![Dir::Left, Dir::Left, Dir::Left, Dir::Left])
        );

        let elem2 = Elem::from_str("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]").unwrap();
        assert_eq!(
            elem2.find_nested(4),
            Some(vec![Dir::Left, Dir::Right, Dir::Right, Dir::Right])
        )
    }

    #[test]
    fn indexing() {
        let elem1 = Elem::from_str("[[[[[9,8],1],2],3],4]").unwrap();
        let nested1 = elem1.find_nested(4).unwrap();
        assert_eq!(
            elem1[&nested1],
            Elem::Com(Box::new((Elem::Num(9), Elem::Num(8))))
        );

        let elem2 = Elem::from_str("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]").unwrap();
        let nested2 = elem2.find_nested(4).unwrap();
        assert_eq!(
            elem2[&nested2],
            Elem::Com(Box::new((Elem::Num(7), Elem::Num(3))))
        )
    }

    #[test_case("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"; "sample 1")]
    #[test_case("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]"; "sample 2")]
    #[test_case("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]"; "sample 3")]
    #[test_case("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]", "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"; "sample 4")]
    fn oops(input: &'static str, expected: &'static str) {
        let mut input = Elem::from_str(input).unwrap();
        let expected = Elem::from_str(expected).unwrap();
        let coord = input.find_nested(4).unwrap();
        input.explode(&coord);
        assert_eq!(input, expected);
    }

    #[test_case("[[[[0,7],4],[15,[0,13]]],[1,1]]"; "sample 1")]
    fn has_splittable(input: &'static str) {
        let input = Elem::from_str(input).unwrap();
        let coord = input.find_splittable(10).unwrap();
        assert_eq!(coord, vec![Dir::Left, Dir::Right, Dir::Left]);
    }

    #[test_case("[[[[0,7],4],[15,[0,13]]],[1,1]]", "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]"; "sample 1")]
    #[test_case("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]", "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]"; "sample 2")]
    fn split(input: &str, expected: &str) {
        let mut input = Elem::from_str(input).unwrap();
        let expected = Elem::from_str(expected).unwrap();
        let coord = input.find_splittable(10).unwrap();
        input.split(&coord);
        assert_eq!(input, expected);
    }

    #[test_case("[[[[4,3],4],4],[7,[[8,4],9]]]", "[1,1]", "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"; "sample 1")]
    #[test_case("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]", "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]", "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]"; "bigadd 1")]
    fn add(e1: &str, e2: &str, expected: &str) {
        let e1 = Elem::from_str(e1).unwrap();
        let e2 = Elem::from_str(e2).unwrap();
        let expected = Elem::from_str(expected).unwrap();
        assert_eq!(e1 + e2, expected);
    }

    #[test]
    fn together() {
        let res: Elem = "[1,1]
[2,2]
[3,3]
[4,4]
[5,5]
[6,6]"
            .lines()
            .map(|line| Elem::from_str(line).unwrap())
            .sum();
        assert_eq!(
            res,
            Elem::from_str("[[[[5,0],[7,4]],[5,5]],[6,6]]").unwrap()
        )
    }

    #[test_case("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]", "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"; "sample 1")]
    #[test_case("[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]", "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]"; "sample 2")]
    fn sum(input: &str, expected: &str) {
        let sum: Elem = input.lines().map(|l| Elem::from_str(l).unwrap()).sum();
        assert_eq!(sum, Elem::from_str(expected).unwrap());
    }

    #[test_case("[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]", "3993"; "sample 1")]
    fn part2(input: &str, output: &str) {
        assert_eq!(second(input), output.parse().unwrap());
    }
}
