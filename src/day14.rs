use regex::Regex;
use std::collections::HashMap;
use std::iter;

#[derive(Debug)]
struct RuleSet {
    orig: String,
    rules: HashMap<(char, char), char>,
}

#[aoc_generator(day14)]
fn gen(input: &str) -> RuleSet {
    let orig = input.lines().next().unwrap().to_string();
    let mut rules = HashMap::new();
    let re = Regex::new(r"(\w+) -> (\w+)").unwrap();

    for cap in re.captures_iter(input) {
        let mut chars = cap[1].chars();
        rules.insert(
            (chars.next().unwrap(), chars.next().unwrap()),
            cap[2].chars().next().unwrap(),
        );
    }

    RuleSet { orig, rules }
}

fn compute(rule_set: &RuleSet, max_it: u8) -> u128 {
    let mut polys: HashMap<(char, char), u128> = rule_set
        .orig
        .chars()
        .zip(rule_set.orig.chars().skip(1))
        .fold(HashMap::new(), |mut acc, pair| {
            *acc.entry(pair).or_insert(0) += 1;
            acc
        });
    for _ in 0..max_it {
        let mut will_push = Vec::new();
        for ((c1, c2), n) in polys.iter_mut() {
            if *n == 0 {
                continue;
            }
            if let Some(&new) = rule_set.rules.get(&(*c1, *c2)) {
                will_push.push(((*c1, new), *n));
                will_push.push(((new, *c2), *n));
                *n = 0;
            }
        }
        for ((a1, a2), n) in will_push {
            *polys.entry((a1, a2)).or_insert(0) += n;
        }
    }
    let mut count = HashMap::new();
    for ((c1, _), n) in polys.iter() {
        *count.entry(c1).or_insert(0) += n;
    }
    let last_char = rule_set.orig.chars().last().unwrap();
    *count.entry(&last_char).or_insert(0) += 1;

    count.values().max().unwrap() - count.values().min().unwrap()
}

#[aoc(day14, part1)]
fn first(rule_set: &RuleSet) -> u128 {
    compute(rule_set, 10)
}
#[aoc(day14, part2)]
fn second(rule_set: &RuleSet) -> u128 {
    compute(rule_set, 40)
}

#[cfg(test)]
mod test {
    use super::*;
    fn input() -> RuleSet {
        gen("NNCB

        CH -> B
        HH -> N
        CB -> H
        NH -> C
        HB -> C
        HC -> B
        HN -> C
        NN -> C
        BH -> H
        NC -> B
        NB -> B
        BN -> B
        BB -> N
        BC -> B
        CC -> N
        CN -> C")
    }

    #[test]
    fn one() {
        assert_eq!(first(&input()), 1588);
    }

    #[test]
    fn two() {
        assert_eq!(second(&input()), 2188189693529);
    }
}
