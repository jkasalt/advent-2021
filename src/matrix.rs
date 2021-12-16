use std::fmt;
use std::ops;

#[derive(Clone)]
pub struct Matrix<T> {
    pub vec: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Matrix<T> {
    pub fn new<I>(items: I, width: usize, height: usize) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        Self {
            vec: items.into_iter().collect(),
            width,
            height,
        }
    }
    #[allow(dead_code)]
    pub fn get(&self, x: isize, y: isize) -> Option<&T> {
        if x < 0
            || x > (self.width - 1).try_into().unwrap()
            || y < 0
            || y > (self.height - 1).try_into().unwrap()
        {
            return None;
        }
        let x: usize = x.try_into().unwrap();
        let y: usize = y.try_into().unwrap();
        self.vec.get(x + y * self.width)
    }

    #[allow(dead_code)]
    pub fn neighbors_of(&self, x: usize, y: usize) -> [Option<&T>; 8] {
        let x: isize = x.try_into().unwrap();
        let y: isize = y.try_into().unwrap();

        let top = self.get(x, y + 1);
        let topright = self.get(x + 1, y + 1);
        let right = self.get(x + 1, y);
        let botright = self.get(x + 1, y - 1);
        let bot = self.get(x, y - 1);
        let botleft = self.get(x - 1, y - 1);
        let left = self.get(x - 1, y);
        let topleft = self.get(x - 1, y + 1);

        [top, topright, right, botright, bot, botleft, left, topleft]
    }

    pub fn neighbor_indices(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut res = Vec::new();
        let mut xes = vec![x];
        let mut yes = vec![y];
        if y > 0 {
            yes.push(y - 1);
        }
        if y < self.height - 1 {
            yes.push(y + 1)
        }
        if x > 0 {
            xes.push(x - 1);
        }
        if x < self.width - 1 {
            xes.push(x + 1);
        }
        for yy in yes {
            for &xx in &xes {
                if xx == x && yy == y {
                    continue;
                }
                res.push((xx, yy));
            }
        }
        res
    }

    pub fn row(&self, i: usize) -> Vec<T>
    where
        T: Clone,
    {
        (0..self.width).map(|j| self[(i, j)].clone()).collect()
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }
}

impl<T> fmt::Debug for Matrix<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut count = 0;
        writeln!(f)?;
        for item in &self.vec {
            write!(f, "{:?}", item)?;
            count += 1;
            if count == self.width {
                count = 0;
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl<T> ops::Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        if x > self.width - 1 || y > self.height - 1 {
            panic!(
                "Index ({}, {}) out of range for Matrix with size ({}, {})",
                y, x, self.height, self.width
            );
        }
        &self.vec[x + y * self.width]
    }
}
impl<T> ops::IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        if x > self.width - 1 || y > self.height - 1 {
            panic!(
                "Index ({}, {}) out of range for Matrix with size ({}, {})",
                y, x, self.height, self.width
            );
        }
        &mut self.vec[x + y * self.width]
    }
}
