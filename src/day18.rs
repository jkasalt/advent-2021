use anyhow::Context;
use std::ops::{Add, Index, IndexMut};
use std::str::FromStr;

#[derive(Debug)]
pub enum Elem {
    Com(Box<(Elem, Elem)>),
    Num(u32),
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
        if let Some(n) = s.chars().next().context("Got empty string")?.to_digit(10) {
            return Ok(Self::Num(n));
        }
        Err(anyhow::anyhow!("problem with string {}", s))
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

impl Elem {
    fn reduce(mut self) -> Self {
        loop {
            if let Some(dirs) = self.find_nested(4) {
                self.explode(&dirs);
            } else if let Some(dirs) = self.has_splittable(10) {
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

    pub fn find_nested(&self, how_much: u32) -> Option<Vec<Dir>> {
        self.has_nested_inner(how_much, Vec::new())
    }

    fn has_nested_inner(&self, how_much: u32, dirs: Vec<Dir>) -> Option<Vec<Dir>> {
        if self.is_simple() && how_much == 0 {
            Some(dirs)
        } else if let Elem::Com(b) = self {
            let mut d0 = dirs.clone();
            d0.push(Dir::Left);
            b.0.has_nested_inner(how_much - 1, d0).or_else(|| {
                let mut d1 = dirs;
                d1.push(Dir::Right);
                b.1.has_nested_inner(how_much - 1, d1)
            })
        } else {
            None
        }
    }

    fn carry(&mut self, direc: Dir, n: u32, start: &[Dir]) {
        let mut dirs = dbg!(start.to_owned());
        while let Some(pos) = dirs.iter().rposition(|d| *d == direc.opposite()) {
            dirs[pos] = direc;
            dbg!(&dirs);
            match self[&dirs] {
                Elem::Num(m) => {
                    self[&dirs] = Elem::Num(n + m);
                    return;
                }
                Elem::Com(_) => dirs.push(direc.opposite()),
            }
        }
    }

    fn carry_right(&mut self, n: u32, start: &[Dir]) {
        let mut dirs = start.to_owned();
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

    fn has_splittable(&self, how_much: u32) -> Option<Vec<Dir>> {
        todo!()
    }

    fn split(&mut self, dirs: &[Dir]) {
        todo!()
    }
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
    #[test_case("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]"; "sample 3")]
    #[test_case("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]", "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"; "sample 4")]
    fn exploding(input: &str, expected: &str) {
        let mut input = Elem::from_str(input).unwrap();
        let expected = Elem::from_str(expected).unwrap();
        let coord = input.find_nested(4).unwrap();
        input.explode(&coord);
        assert_eq!(input, expected);
    }
}
