
fn input_day3(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn find_most_common(input: &[&str]) -> Vec<usize> {
    let mut num_ones = Vec::new();
    let mut first = true;
    for line in input.iter() {
        for (i, char) in line.chars().enumerate() {
            if first {
                num_ones.push(0);
            }

            if char == '1' {
                num_ones[i] += 1
            }
        }
        first = false;
    }
    num_ones
}

fn solve_part1(input: &[&str]) -> u64 {
    
    let num_ones = find_most_common(input);
    eprintln!("{:?}", num_ones);
    
    let mut gamma = 0u64;
    let mut epsilon = 0u64;

    for n in num_ones {
        if n > (input.len() / 2) {
            gamma = (gamma << 1) | 1;
            epsilon = (epsilon << 1) | 0;
        } else { 
            gamma = (gamma << 1) | 0;
            epsilon = (epsilon << 1) | 1;
        }
    }

    eprintln!("{:b}", gamma);
    eprintln!("{:b}", epsilon);

    gamma * epsilon
}

fn create_mask(orig_size: usize, input: Vec<usize>) -> String {
    let mut mask = String::new();
    for n in input {
        if n > (orig_size / 2) {
            mask = (mask << 1) | 1;
        } else {
            mask = (mask << 1) | 0;
        }
    }    
    mask
}

fn solve_part2(input: &[&str]) -> i32 {
    let mut horizontal_pos = 0;
    let mut depth = 0;
    let mut aim = 0;

    horizontal_pos * depth
}

#[cfg(test)]
mod test_day3 {    
    use super::{input_day3, solve_part1, solve_part2};
    #[test]
    fn day3_part1() {
        let input = include_str!("../input/2021/day3.txt");
        let parsed_input = input_day3(input);
        let result = solve_part1(&parsed_input);
        dbg!(result);
    }
    #[test]
    fn day3_part2() {
        let input = include_str!("../input/2021/day3.txt");
        let parsed_input = input_day3(input);
        let result = solve_part2(&parsed_input);
        dbg!(result);
    }
}