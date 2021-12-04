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
                num_ones[i] += 1;
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

fn solve_part2(size: usize, input: &[usize]) -> usize {
    eprintln!("Size {}, len input {}", size, input.len());
    let mut oxygen_ratio = 0;
    let mut working_list: Vec<usize> = input.clone().into();

    for bit in (0..size).rev() {
        let most_common = find_most_common_bits(size, &working_list);
        let mask = create_mask(true, working_list.len(), most_common);
        working_list = filter_on_bitmask(bit, mask, &working_list);
        if working_list.len() == 1 {
            oxygen_ratio = working_list[0];
            break;
        }
    }
    dbg!(oxygen_ratio);

    let mut co2_ratio = 0;
    let mut working_list: Vec<usize> = input.clone().into();
    for bit in (0..size).rev() {
        let most_common = find_most_common_bits(size, &working_list);
        let mask = create_mask(false, working_list.len(), most_common);
        working_list = filter_on_bitmask(bit, mask, &working_list);
        if working_list.len() == 1 {
            co2_ratio = working_list[0];
            break;
        }
    }
    dbg!(co2_ratio);
    oxygen_ratio * co2_ratio
}

fn input_day3_part2(input: &str) -> (usize, Vec<usize>) {
    let elem = input.lines().next().unwrap();
    (
        elem.len(),
        input
            .lines()
            .filter_map(|l| usize::from_str_radix(l, 2).ok())
            .collect(),
    )
}

fn create_mask(filter_on_most_common: bool, length: usize, input: Vec<usize>) -> usize {
    let mut mask = 0;
    let v = if filter_on_most_common { 1 } else { 0 };
    let threshold = length / 2;
    for n in input.iter().rev() {
        let zeros = length - n;
        if *n == zeros || *n > threshold {
            mask = (mask << 1) | (v & 1);
        } else {
            mask = (mask << 1) | (!v & 1);
        }
    }
    mask
}

fn filter_on_bitmask(bit: usize, mask: usize, input: &[usize]) -> Vec<usize> {
    let reverse_mask = mask & (1 << bit) == 0;

    input
        .iter()
        .filter_map(|v| {
            if reverse_mask {
                if (!v & (1 << bit)) > 0 {
                    Some(*v)
                } else {
                    None
                }
            } else {
                if (v & (1 << bit)) > 0 {
                    Some(*v)
                } else {
                    None
                }
            }
        })
        .collect()
}

fn find_most_common_bits(size: usize, input: &[usize]) -> Vec<usize> {
    let mut num_ones = vec![0; size];
    for v in input {
        for bit in 0..size {
            if (v & (1 << bit)) > 0 {
                num_ones[bit] += 1;
            }
        }
    }
    num_ones
}

#[cfg(test)]
mod test_day3 {
    use super::{input_day3, input_day3_part2, solve_part1, solve_part2};
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
        let (size, parsed_input) = input_day3_part2(input);
        let result = solve_part2(size, &parsed_input);
        dbg!(result);
    }

    #[test]
    fn test_day3_part2() {
        let input = include_str!("../input/2021/day3_part2_test.txt");
        let (size, parsed_input) = input_day3_part2(input);
        let result = solve_part2(size, &parsed_input);
        dbg!(result);
    }

    #[test]
    fn test_mask() {
        let v = 1usize;
        eprintln!("{:b}", !v);
    }
}
