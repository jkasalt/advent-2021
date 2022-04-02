use std::collections::HashSet;
use std::iter;

#[aoc(day8, part1)]
pub fn first(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            line.split_once('|')
                .unwrap()
                .1
                .split_whitespace()
                .fold(0, |sum, word| {
                    sum + (word.len() == 2 || word.len() == 3 || word.len() == 4 || word.len() == 7)
                        as u32
                })
        })
        .sum()
}

fn decode_line(input: &str) -> u32 {
    let words: Vec<HashSet<_>> = input
        .split_once('|')
        .unwrap()
        .0
        .split_whitespace()
        .map(|word| word.chars().collect())
        .collect();
    let one: &HashSet<char> = words.iter().find(|word| word.len() == 2).unwrap();
    let seven: &HashSet<char> = words.iter().find(|word| word.len() == 3).unwrap();
    let four: &HashSet<char> = words.iter().find(|word| word.len() == 4).unwrap();
    let eight: &HashSet<char> = words.iter().find(|word| word.len() == 7).unwrap();
    let four_seven: HashSet<char> = four.union(seven).copied().collect();
    let nine: &HashSet<_> = words
        .iter()
        .find(|word| word.is_superset(&four_seven) && (*word - &four_seven).len() == 1)
        .unwrap();
    let epsilon = eight.iter().find(|c| !nine.contains(c)).unwrap();
    let a: HashSet<_> = four_seven.iter().chain(iter::once(epsilon)).collect();
    let eta = eight.iter().find(|c| !a.contains(c)).unwrap();
    let seven_eta: HashSet<_> = seven.iter().chain(iter::once(eta)).copied().collect();
    let three: &HashSet<_> = words
        .iter()
        .find(|word| word.iter().filter(|c| !seven_eta.contains(c)).count() == 1)
        .unwrap();
    let delta = three.difference(&seven_eta).next().unwrap();
    let zero: HashSet<_> = {
        let mut z = eight.clone();
        z.remove(delta);
        z
    };
    let five = words
        .iter()
        .filter(|word| word.is_subset(nine) && word.len() == 5)
        .find(|&word| word != three)
        .unwrap();
    let six = {
        let mut s = five.clone();
        s.insert(*epsilon);
        s
    };
    let mut so_far = vec![&zero, one, three, four, five, &six, seven, eight, nine];
    let two = words
        .iter()
        .find(|word| !so_far.iter().any(|num| num == word))
        .unwrap();

    so_far.insert(2, two);

    input
        .split_once('|')
        .unwrap()
        .1
        .split_whitespace()
        .map(|word| {
            let word: HashSet<_> = word.chars().collect();
            so_far
                .iter()
                .position(|num| **num == word)
                .unwrap_or_else(|| {
                    panic!(
                        "Cannot find {:#?} in {:#?} with input {}",
                        word, so_far, input
                    )
                }) as u32
        })
        .rev()
        .enumerate()
        .fold(0, |acc, (pwr, digit)| {
            acc + (digit * 10_u32.pow(pwr as u32))
        })
}

#[aoc(day8, part2)]
pub fn second(input: &str) -> u32 {
    input.lines().map(decode_line).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    fn sample() -> &'static str {
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"
    }

    #[test]
    fn mini() {
        assert_eq!(second("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"), 5353);
    }

    #[test]
    fn one() {
        assert_eq!(first(sample()), 26);
    }

    #[test]
    fn two() {
        assert_eq!(second(sample()), 61229);
    }
}
