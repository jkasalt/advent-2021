pub fn first(input: &str) -> u64 {
    let mut count = 0;

    'outer: for line in input.lines() {
        let mut stack = Vec::new();

        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' | ']' | '}' | '>' => {
                    let paren = stack.pop().unwrap();
                    match (paren, c) {
                        ('(', ')') | ('[', ']') | ('{', '}') | ('<', '>') => {}
                        (_, ')') => {
                            count += 3;
                            continue 'outer;
                        }
                        (_, ']') => {
                            count += 57;
                            continue 'outer;
                        }
                        (_, '}') => {
                            count += 1197;
                            continue 'outer;
                        }
                        (_, '>') => {
                            count += 25137;
                            continue 'outer;
                        }
                        (p, s) => unreachable!("{}, {}", p, s),
                    }
                }
                x => panic!("unexpected input {}", x),
            }
        }
    }
    count
}

pub fn second(input: &str) -> u64 {
    let mut scores = Vec::new();
    'line: for line in input.lines() {
        let mut stack = Vec::new();
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' | ']' | '}' | '>' => {
                    let paren = stack.pop().unwrap();
                    match (paren, c) {
                        ('(', ')') | ('[', ']') | ('{', '}') | ('<', '>') => {}
                        (_, _) => continue 'line,
                    }
                }
                x => panic!("unexpected input {}", x),
            }
        }

        let score = stack.iter().rev().fold(0, |score, paren| {
            let val = match paren {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                x => panic!("unexpected input {}", x),
            };
            score * 5 + val
        });
        scores.push(score);
    }
    scores.sort_unstable();
    scores[scores.len() / 2]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one() {
        let input = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
        assert_eq!(first(input), 26397);
    }

    #[test]
    fn two() {
        let input = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
        assert_eq!(second(input), 288957);
    }
}
