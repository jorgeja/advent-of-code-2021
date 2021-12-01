use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn input_day1(input: &str) -> Vec<u32> {
    let v = input.lines().filter_map(|s| s.parse::<u32>().ok()).collect();
    v
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[u32]) -> u32 {
    let mut last = None;
    let mut increases = 0;
    for i in input.iter() {
        if let Some(last_i) = last {
            if last_i < i {
                increases += 1;
            }
        }
        last = Some(i);
    }
    increases
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[u32]) -> u32 {
    let mut last = None;
    let mut increases = 0;
    for window in input.windows(3) {
        let sum = window[0] + window[1] + window[2];
        if let Some(last_i) = last {            
            if last_i < sum {
                increases += 1;
            }
        }

        last = Some(sum);
    }
    increases
}
