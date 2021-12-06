fn input_day6(input: &str) -> Vec<u32> {
    input.split(',').filter_map(|s| {
        let n = s.parse::<u32>();
        if n.is_err() {
            eprintln!("Could not parse {}", s)
        }
        n.ok()
    }).collect()
}

fn solve_part1(mut fish: Vec<u32>, days: u32) -> usize {
    eprintln!("Initial State: {:?}", fish);
    for _i in 0..days {
        let new_fish = fish.iter_mut().fold(0, |mut new_fish, fish| {
            if *fish == 0 {
                new_fish += 1;
                *fish = 7;
            }
            *fish -= 1;
            new_fish
        });

        for _ in 0..new_fish {
            fish.push(8);
        }
        //eprintln!("After {} days: {:?}", _i + 1, fish);
    }
    fish.len()
}

fn solve_part2(fish: Vec<u32>, days: u32) -> usize {    
    let mut grouped_fish = [0u64; 9];
    for v in fish {
        grouped_fish[v as usize] += 1; 
    }

    for _i in 0..days {
        let new_fish = grouped_fish[0];        
        grouped_fish[7] += grouped_fish[0];
        
        for i in 1..9 {
            grouped_fish[i-1] = grouped_fish[i];
        }

        grouped_fish[8] = new_fish;        
    }

    grouped_fish.iter().fold(0, |mut acc, v| {acc += *v as usize; acc})
}

#[cfg(test)]
mod test_day6 {
    use super::{input_day6, solve_part1, solve_part2};
    #[test]
    fn day6_part1_test() {
        let input = include_str!("../input/2021/day6_part1_test.txt");
        let parsed_input = input_day6(input);
        
        let result = solve_part1(parsed_input, 18);
        assert_eq!(result, 26);
        let parsed_input = input_day6(input);
        let result = solve_part1(parsed_input, 80);
        assert_eq!(result, 5934);
    }
    
    #[test]
    fn day6_part1() {
        let input = include_str!("../input/2021/day6.txt");
        let mut parsed_input = input_day6(input);        
        parsed_input.push(4);
        let result = solve_part1(parsed_input, 80);
        dbg!(result);
    }

    #[test]
    fn day6_part2_test() {
        let input = include_str!("../input/2021/day6_part1_test.txt");
        let parsed_input = input_day6(input);
        
        let result = solve_part2(parsed_input, 18);
        assert_eq!(result, 26);
        let parsed_input = input_day6(input);
        let result = solve_part2(parsed_input, 80);
        assert_eq!(result, 5934);
    }

    #[test]
    fn day6_part2() {
        let input = include_str!("../input/2021/day6.txt");
        let mut parsed_input = input_day6(input);        
        parsed_input.push(4);
        let result = solve_part2(parsed_input, 256);
        dbg!(result);
    }
}
