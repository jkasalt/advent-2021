type Num = u64;
#[aoc(day6, part1)]
fn first(input: &str) -> Num {
    compute(80, input)
}

#[aoc(day6, part2)]
fn second(input: &str) -> Num {
    compute(256, input)
}

fn compute(days: usize, input: &str) -> Num {
    let mut fishes: [Num; 9] =
        input
            .split(',')
            .map(|n| n.parse().unwrap())
            .fold([0; 9], |mut arr, n: usize| {
                arr[n] += 1;
                arr
            });

    for _ in 0..days {
        fishes[..=6].rotate_left(1);
        let babbies = fishes[6];
        fishes[6] += fishes[7];
        fishes[7] = fishes[8];
        fishes[8] = babbies;
    }
    fishes.iter().sum()
}

#[cfg(test)]
mod test {
    const INPUT: &str = "3,4,3,1,2";

    #[test]
    fn mini() {
        let input = "2";
        assert_eq!(compute(5, input), 2);
    }

    use super::*;
    #[test]
    fn one() {
        assert_eq!(first(INPUT), 5934);
    }

    #[test]
    fn two() {
        assert_eq!(second(INPUT), 26984457539);
    }

    #[test]
    #[ignore]
    fn bigboy_1() {
        compute(9999999, INPUT);
    }
}
