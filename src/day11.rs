#[derive(Debug, Clone)]
struct Matrix {
    vec: Vec<u32>,
    width: usize,
    height: usize,
}

impl Matrix {
    fn get(&self, x: isize, y: isize) -> Option<&u32> {
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

    fn neighbors_of(&self, x: usize, y: usize) -> [Option<&u32>; 8] {
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

    fn neighbor_indices(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut res = Vec::new();
        if y > 0 {
            res.push((x, y - 1));
        }
        if y < self.height - 1 {
            res.push((x, y + 1));
        }
        if x > 0 {
            res.push((x - 1, y));
        }
        if x < self.width - 1 {
            res.push((x + 1, y));
        }
        res
    }
}
