use itertools::Itertools;

pub fn gen(input: &str) -> Vec<u32> {
    input
        .lines()
        .filter_map(|s| s.parse::<u32>().ok())
        .collect()
}

pub fn first(input: &[u32]) -> usize {
    input.iter().tuple_windows().filter(|(a, b)| a < b).count()
}

pub fn second(input: &[u32]) -> usize {
    input
        .iter()
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .filter(|(s1, s2)| s1 < s2)
        .count()
}

#[cfg(test)]
mod test {
    #[test]
    fn first() {
        let input = super::gen(
            r"199
200
208
210
200
207
240
269
260
263",
        );
        assert_eq!(super::first(&input), 7);
    }

    #[test]
    fn second() {
        let input = super::gen(
            r"199
200
208
210
200
207
240
269
260
263",
        );
        assert_eq!(super::second(&input), 5);
    }
}
