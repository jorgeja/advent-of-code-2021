enum SubCommand {
    Up(i32),
    Down(i32),
    Forward(i32),
}

fn input_day2(input: &str) -> Vec<SubCommand> {
    input
        .lines()
        .filter_map(|s| {
            let mut parts = s.split_whitespace();
            let str_cmd = parts.next().unwrap();
            let value = parts.next().unwrap().parse::<i32>().unwrap();
            match str_cmd {
                "up" => SubCommand::Up(value),
                "down" => SubCommand::Down(value),
                "forward" => SubCommand::Forward(value),
                _ => unreachable!(),
            }
            .into()
        })
        .collect()
}

fn solve_part1(input: &[SubCommand]) -> i32 {
    let mut horizontal_pos = 0;
    let mut depth = 0;

    for cmd in input.iter() {
        match cmd {
            SubCommand::Up(v) => depth -= v,
            SubCommand::Down(v) => depth += v,
            SubCommand::Forward(v) => horizontal_pos += v,
        }
    }
    horizontal_pos * depth
}

fn solve_part2(input: &[SubCommand]) -> i32 {
    let mut horizontal_pos = 0;
    let mut depth = 0;
    let mut aim = 0;

    for cmd in input.iter() {
        match cmd {
            SubCommand::Up(v) => aim -= v,
            SubCommand::Down(v) => aim += v,
            SubCommand::Forward(v) => {
                horizontal_pos += v;
                depth += aim * v;
            }
        }
    }
    horizontal_pos * depth
}

#[cfg(test)]
mod test_day2 {
    use super::{input_day2, solve_part1, solve_part2};
    #[test]
    fn day2_part1() {
        let input = include_str!("../input/2021/day2.txt");
        let parsed_input = input_day2(input);
        let result = solve_part1(&parsed_input);
        dbg!(result);
    }
    #[test]
    fn day2_part2() {
        let input = include_str!("../input/2021/day2.txt");
        let parsed_input = input_day2(input);
        let result = solve_part2(&parsed_input);
        dbg!(result);
    }
}
