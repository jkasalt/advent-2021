use regex::Regex;
use std::collections::HashSet;

pub struct FoldInfo {
    points: HashSet<(usize, usize)>,
    folds: Vec<(char, usize)>,
}

impl FoldInfo {
    fn perform_folds(&self) {
        let max_x = self.points.iter().map(|p| p.0).max().unwrap();
        let min_x = self.points.iter().map(|p| p.0).min().unwrap();
        let max_y = self.points.iter().map(|p| p.1).max().unwrap();
        let min_y = self.points.iter().map(|p| p.1).min().unwrap();

        println!("X: {} -- {}", min_x, max_x);
        println!("Y: {} -- {}", min_y, max_y);

        let folded = self
            .folds
            .iter()
            .fold(self.points.clone(), |paper, &(d, at)| match d {
                'x' => fold_x(paper, at),
                'y' => fold_y(paper, at),
                z => panic!("Unexpected input {}", z),
            });
        display(&folded);
    }
}

pub fn gen(input: &str) -> FoldInfo {
    let re_coord = Regex::new(r"(\d+),(\d+)").unwrap();
    let re_fold = Regex::new(r"fold along (\w)=(\d+)").unwrap();
    let mut points = HashSet::new();
    let mut folds = Vec::new();

    for cap in re_coord.captures_iter(input) {
        points.insert((cap[1].parse().unwrap(), cap[2].parse().unwrap()));
    }
    for cap in re_fold.captures_iter(input) {
        folds.push((cap[1].chars().next().unwrap(), cap[2].parse().unwrap()));
    }
    FoldInfo { points, folds }
}

fn display(points: &HashSet<(usize, usize)>) {
    let max_x = points.iter().map(|p| p.0).max().unwrap();
    let min_x = points.iter().map(|p| p.0).min().unwrap();
    let max_y = points.iter().map(|p| p.1).max().unwrap();
    let min_y = points.iter().map(|p| p.1).min().unwrap();

    println!("X: {} -- {}", min_x, max_x);
    println!("Y: {} -- {}", min_y, max_y);

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if points.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn fold_y(points: HashSet<(usize, usize)>, y_fold: usize) -> HashSet<(usize, usize)> {
    points
        .iter()
        .map(|&(x, y)| {
            if y < y_fold {
                (x, y)
            } else {
                (x, y - 2 * (y - y_fold))
            }
        })
        .collect()
}

fn fold_x(points: HashSet<(usize, usize)>, x_fold: usize) -> HashSet<(usize, usize)> {
    points
        .iter()
        .map(|&(x, y)| {
            if x < x_fold {
                (x, y)
            } else {
                (x - 2 * (x - x_fold), y)
            }
        })
        .collect()
}

pub fn first(fold_info: &FoldInfo) -> usize {
    let (d, num) = fold_info.folds[0];
    if d == 'x' {
        fold_x(fold_info.points.clone(), num).len()
    } else if d == 'y' {
        fold_y(fold_info.points.clone(), num).len()
    } else {
        panic!("Unexpected input {}", d)
    }
}

pub fn second(fold_info: &FoldInfo) -> usize {
    fold_info.perform_folds();
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one() {
        let input = gen("6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5");
        assert_eq!(first(&input), 17);
    }
}
