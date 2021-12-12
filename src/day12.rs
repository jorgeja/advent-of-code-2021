use std::collections::HashMap;

fn input_day12(input: &str) -> HashMap<String, Vec<String>> {
    let mut edges = HashMap::new();
    for line in input.lines() {
        let mut elems = line.split("-");
        let first = elems.next().unwrap();
        let second = elems.next().unwrap();
        edges.entry(first.into()).or_insert(vec![second.into()]).push(second.into());
        edges.entry(second.into()).or_insert(vec![first.into()]).push(first.into());
    }
    edges
}

fn solve_part1(input: &[u32]) -> u32 {
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

fn solve_part2(input: &[u32]) -> u32 {
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
mod test_day12 {
    use super::{input_day12, solve_part1, solve_part2};
    #[test]
    fn day12_part1() {
        let input = include_str!("../input/2021/day12.txt");
        let parsed_input = input_day12(input);
        let result = solve_part1(&parsed_input);
        dbg!(result);
    }
    #[test]
    fn day12_part2() {
        let input = include_str!("../input/2021/day12.txt");
        let parsed_input = input_day12(input);
        let result = solve_part2(&parsed_input);
        dbg!(result);
    }
}
