#[aoc_generator(day7)]
fn gen(input: &str) -> Vec<i32> {
    input.split(',').map(|s| s.parse().unwrap()).collect()
}

#[aoc(day7, part1)]
fn first(v: &[i32]) -> i32 {
    let mut w = vec![0; v.len()];
    w.copy_from_slice(v);
    w.sort_unstable();
    let med = w[w.len() / 2];
    w.iter().fold(0, |s, n| s + (med - n).abs())
}

#[aoc(day7, part2)]
fn second(v: &[i32]) -> i32 {
    let avg = v.iter().sum::<i32>() as f64 / v.len() as f64;
    ((avg - 0.5).floor() as i32..(avg + 0.5).ceil() as i32)
        .map(|p| {
            v.iter().fold(0, |sum, crab| {
                sum + ((p - crab).pow(2) + (p - crab).abs()) / 2
            })
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    fn input() -> Vec<i32> {
        gen("16,1,2,0,4,2,7,1,2,14")
    }

    #[test]
    fn one() {
        assert_eq!(first(&input()), 37);
    }

    #[test]
    fn two() {
        assert_eq!(second(&input()), 168);
    }
}
