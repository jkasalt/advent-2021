#[aoc(day6, part1)]
fn first(input: &str) -> u64 {
    compute(80, input)
}

#[aoc(day6, part2)]
fn second(input: &str) -> u64 {
    compute(256, input)
}

fn compute(days: usize, input: &str) -> u64 {
    let mut fishes: Vec<u8> = input.split(',').map(|s| s.parse().unwrap()).collect();
    for _ in 0..days {
        let mut babbies = 0;
        for fish in &mut fishes {
            if *fish == 0 {
                *fish = 6;
                babbies += 1;
            } else {
                *fish -= 1;
            }
        }
        for _ in 0..babbies {
            fishes.push(8)
        }
    }
    fishes.len() as u64
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn one() {
        let input = "3,4,3,1,2";
        assert_eq!(first(input), 5934);
    }

    #[test]
    fn two() {
        let input = "3,4,3,1,2";
        assert_eq!(second(input), 26984457539);
    }
}