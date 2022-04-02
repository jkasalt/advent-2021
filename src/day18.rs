use std::ops::Add;
use std::str::FromStr;

enum Token {
    Open,
    Num(u32),
    Close,
}

struct SnailNum(Vec<Token>);

impl Add for SnailNum {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut res = vec![Token::Open];
        for token in self.0 {
            res.push(token);
        }
        for token in rhs.0 {
            res.push(token);
        }
        res.push(Token::Close);
        SnailNum(res).reduce()
    }
}

impl FromStr for SnailNum {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.chars()
                .filter_map(|c| match c {
                    '[' => Some(Token::Open),
                    ']' => Some(Token::Close),
                    ',' => None,
                    x => x.to_digit(10).map(Token::Num),
                })
                .collect(),
        ))
    }
}

impl SnailNum {
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
        let mut open_count = 0;
        for token in &self.0 {
            match token {
                Token::Open => open_count += 1,
                Token::Close => open_count -= 1,
                _ => {}
            };
            if open_count >= how_much {
                return true;
            }
        }
        false
    }

    fn has_splittable(&self, how_much: u32) -> bool {
        self.0.iter().any(|t| {
            if let &Token::Num(n) = t {
                if n >= how_much {
                    return true;
                }
            }
            false
        })
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
    fn test() {}
}
