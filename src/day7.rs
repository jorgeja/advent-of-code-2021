fn input_day7(input: &str) -> Vec<i32> {
    input.split(",").filter_map(|s| s.trim_end().parse().ok()).collect()
}

fn solve_part1(mut positions: Vec<i32>) -> i32 {
    positions.sort();
    let middle_pos = positions.len() / 2;
    let median = positions[middle_pos];
    let sum_fuel = positions.iter().fold(0, |mut fuel, pos| { 
        fuel += (pos - median).abs();
        fuel
    });
    sum_fuel
}

fn solve_part2(mut positions: Vec<i32>) -> i32 {
    positions.sort();
    let min_pos = positions[0];
    let max_pos = *positions.last().unwrap();
    let mut fuel_consumption = Vec::new();
    for aligned_pos in min_pos..=max_pos {
        let sum_fuel = positions.iter().fold(0, |mut fuel, pos| { 
            let distance = (pos - aligned_pos).abs();
            fuel += distance * (distance + 1) / 2;
            fuel
        });
        fuel_consumption.push(sum_fuel);
    }
    fuel_consumption.sort();
    fuel_consumption[0]
}

#[cfg(test)]
mod test_day7 {

    use super::{input_day7, solve_part1, solve_part2};

    #[test]
    fn day7_part1_test() {
        let input = include_str!("../input/2021/day7_test.txt");
        let parsed_input = input_day7(input);
        let result = solve_part1(parsed_input);
        assert_eq!(result, 37);        
    }

    #[test]
    fn day7_part1() {
        let input = include_str!("../input/2021/day7.txt");
        let parsed_input = input_day7(input);
        let result = solve_part1(parsed_input);
        dbg!(result);
    }

    #[test]
    fn day7_part2_test() {
        let input = include_str!("../input/2021/day7_test.txt");
        let parsed_input = input_day7(input);
        let result = solve_part2(parsed_input);
        assert_eq!(result, 168);        
    }

    #[test]
    fn day7_part2() {
        let input = include_str!("../input/2021/day7.txt");
        let parsed_input = input_day7(input);
        let result = solve_part2(parsed_input);
        dbg!(result);
    }
}
