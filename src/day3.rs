fn most_common(input: &[Vec<bool>]) -> Vec<bool> {
    let width = input.iter().map(|line| line.len()).max().unwrap();
    // Get the count for each column
    let mut count: Vec<usize> = vec![0; width];
    for line in input {
        for (i, bit) in line.iter().enumerate() {
            count[i] += *bit as usize;
        }
    }
    // Turn each count into a binary
    let most_common: Vec<bool> = count
        .iter()
        .map(|&num| num as f32 >= input.len() as f32 / 2.0)
        .collect();

    most_common
}

fn bits_to_int(bits: &[bool]) -> i32 {
    bits.iter()
        .rev()
        .fold((0, 0), |(num, pow), &bit| {
            (num + 2_i32.pow(pow) * bit as i32, pow + 1)
        })
        .0
}

#[aoc_generator(day3)]
pub fn gen(input: &str) -> Vec<Vec<bool>> {
    let input: Vec<_> = input
        .lines()
        .map(|line| {
            line.chars().fold(Vec::new(), |mut v, bit| {
                match bit {
                    '0' => v.push(false),
                    '1' => v.push(true),
                    x => println!("Unexpected input: {}", x),
                };
                v
            })
        })
        .collect();

    input
}

#[aoc(day3, part1)]
pub fn first(input: &[Vec<bool>]) -> i32 {
    let most_common = most_common(input);
    let epsilon = bits_to_int(&most_common);
    let least_common: Vec<bool> = most_common.iter().map(|b| !b).collect();
    let gamma = bits_to_int(&least_common);

    epsilon * gamma
}

#[aoc(day3, part2)]
pub fn second(input: &[Vec<bool>]) -> i32 {
    let mut oxygen = input.to_vec();
    let mut i = 0;
    while oxygen.len() > 1 {
        let most_common = most_common(&oxygen);
        oxygen = oxygen
            .into_iter()
            .filter(|line| line[i] == most_common[i])
            .collect();
        i += 1;
    }
    let oxygen = bits_to_int(&oxygen.pop().unwrap());

    let mut co2 = input.to_vec();
    let mut i = 0;
    while co2.len() > 1 {
        let most_common = most_common(&co2);
        co2 = co2
            .into_iter()
            .filter(|line| line[i] != most_common[i])
            .collect();
        i += 1;
    }
    let co2 = bits_to_int(&co2.pop().unwrap());

    oxygen * co2
}

#[cfg(test)]
mod test {
    #[test]
    fn one() {
        let input = super::gen(
            r"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010",
        );
        assert_eq!(super::first(&input), 198)
    }
    #[test]
    fn two() {
        let input = super::gen(
            r"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010",
        );
        assert_eq!(super::second(&input), 230)
    }
}
