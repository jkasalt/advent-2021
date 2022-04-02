use anyhow::Context;
use std::ops::Add;
use std::str::FromStr;

#[derive(Debug)]
enum Elem {
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
            dbg!(&s, &ss, &comma_pos, &closing_pos);
            let (subs1, subs2) = dbg!(ss[..closing_pos].split_at(comma_pos + 1));
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

impl Elem {
    fn reduce(mut self) -> Self {
        loop {
            if self.has_nested(4) {
                self.explode();
            } else if self.has_splittable(10) {
                self.split()
            } else {
                break;
            }
        }
        self
    }

    fn has_nested(&self, how_much: u32) -> bool {
        todo!()
    }

    fn has_splittable(&self, how_much: u32) -> bool {
        todo!()
    }

    fn explode(&mut self) {
        todo!()
    }

    fn split(&mut self) {}
}

#[cfg(test)]
mod test {
    use super::*;

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
}
