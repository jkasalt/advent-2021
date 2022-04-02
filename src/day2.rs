#[aoc(day2, part1)]
pub fn first(input: &str) -> i32 {
    let mut forward = 0;
    let mut depth = 0;
    input
        .lines()
        .filter_map(|line| line.split_once(' '))
        .for_each(|(direction, n)| {
            let n: i32 = n.parse().expect("A number as second word on each line");
            match direction {
                "forward" => forward += n,
                "down" => depth += n,
                "up" => depth -= n,
                _ => {}
            }
        });
    forward * depth
}

#[aoc(day2, part2)]
pub fn second(input: &str) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    input
        .lines()
        .filter_map(|line| line.split_once(' '))
        .for_each(|(direction, n)| {
            let n: i32 = n.parse().expect("A number as second word on each line");
            match direction {
                "forward" => {
                    horizontal += n;
                    depth += aim * n;
                }
                "down" => aim += n,
                "up" => aim -= n,
                _ => {}
            }
        });
    horizontal * depth
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_1() {
        let input = r"forward 5
down 5
forward 8
up 3
down 8
forward 2";
        assert_eq!(super::first(input), 150);
    }

    #[test]
    fn test_2() {
        let input = r"forward 5
down 5
forward 8
up 3
down 8
forward 2";
        assert_eq!(super::second(input), 900);
    }
}
