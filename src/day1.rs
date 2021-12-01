pub fn input_day1(input: &str) -> Vec<u32> {
    input.lines().filter_map(|s| s.parse::<u32>().ok()).collect()
}

pub fn solve_part1(input: &[u32]) -> u32 {
    let mut last = 0;
    let mut increases = 0;
    for i in input.iter() {
        if last < *i {
            increases += 1;
        }
        last = *i;
    }
    increases
}

pub fn solve_part2(input: &[u32]) -> u32 {
    let mut last = 0;
    let mut increases = 0;
    for window in input.windows(3) {
        let sum = window[0] + window[1] + window[2];
        if last < sum {
            increases += 1;
        }
        last = sum;
    }
    increases
}

#[cfg(test)]
mod test {    
    use super::{input_day1, solve_part1, solve_part2};
    #[test]
    fn day1_part1() {
        let input = include_str!("../input/2021/day1.txt");
        let parsed_input = input_day1(input);
        let result = solve_part1(&parsed_input);
        eprintln!("day1_part1: {}", result);
    }
    #[test]
    fn day1_part2() {
        let input = include_str!("../input/2021/day1.txt");
        let parsed_input = input_day1(input);
        let result = solve_part2(&parsed_input);
        eprintln!("day1_part2: {}", result);
    }
}