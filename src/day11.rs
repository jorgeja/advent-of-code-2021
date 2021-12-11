use std::collections::HashSet;

fn input_day11(input: &str) -> Vec<Vec<usize>> {
    input.lines().map(|s| s.chars().map(|c| c.to_digit(10).unwrap() as usize).collect()).collect()
}

fn charge_octopi(input: &mut Vec<Vec<usize>>) -> usize {    
    let mut flash_ready_octopi = Vec::new();
    for row in 0..input.len() {
        for col in 0..input[row].len() {
            input[row][col] += 1;

            if 9 < input[row][col] {
                flash_ready_octopi.push((row, col));
            }
        }
    }

    let mut flashed_octopi = HashSet::new();
    while flash_ready_octopi.len() > 0 {
        let mut new_flash_ready_octopi = Vec::new();
        for (row, col) in flash_ready_octopi.iter() {
            if !flashed_octopi.contains(&(*row, *col)) {
                flashed_octopi.insert((*row, *col));
                new_flash_ready_octopi.extend(flash_octopus(*row, *col, input));
            }
        }
        flash_ready_octopi = new_flash_ready_octopi;
    }

    for (row, col) in flashed_octopi.iter() {
        input[*row][*col] = 0;
    }

    flashed_octopi.len()
}

fn flash_octopus(row: usize, col: usize, input: &mut Vec<Vec<usize>>) -> Vec<(usize, usize)>{
    let mut neighbor_octopi = Vec::new();
    for r in -1..=1 {
        for c in -1..=1 {            
            if (row == 0 && r < 0) || (col == 0 && c < 0) || (row == input.len() - 1 && r > 0) || (col == input[row].len() - 1 && c > 0) || ( r == 0 && c == 0) {                
                continue
            } else {
                let n_r = (row as i32 + r) as usize;
                let n_c = (col as i32 + c) as usize;
                
                input[n_r][n_c] += 1;

                if 9 < input[n_r][n_c] {
                    neighbor_octopi.push((n_r, n_c));
                }
            }            
        }
    }
    //eprint!("\n");
    neighbor_octopi
}

fn print_octopi(input: &Vec<Vec<usize>>) {
    for row in 0..input.len() {
        for col in 0..input[row].len() {
            eprint!("{}", input[row][col]);
        }
        eprint!("\n");
    }
}

fn solve_part1(mut input: Vec<Vec<usize>>, days: usize) -> usize {
    let mut num_flashes = 0;
    for _d in 0..days {
        num_flashes += charge_octopi(&mut input);
    }
    num_flashes
}

fn solve_part2(mut input: Vec<Vec<usize>>) -> usize {
    let num_octopi = input.len() * input[0].len(); 
    let mut day = 1;         
    while day < usize::MAX {        
        if charge_octopi(&mut input) == num_octopi {
            return day;
        }
        day += 1;
    }
    0
}

#[cfg(test)]
mod test_day11 {
    use super::{input_day11, solve_part1, solve_part2};
    #[test]
    fn day11_part1_test() {
        let input = include_str!("../input/2021/day11_test.txt");
        let parsed_input = input_day11(input);
        let result = solve_part1(parsed_input, 100);
        assert_eq!(result, 1656);
    }

    #[test]
    fn day11_part1() {
        let input = include_str!("../input/2021/day11.txt");
        let parsed_input = input_day11(input);
        let result = solve_part1(parsed_input, 100);
        dbg!(result);
    }

    #[test]
    fn day11_part2_test() {
        let input = include_str!("../input/2021/day11_test.txt");
        let parsed_input = input_day11(input);
        let result = solve_part2(parsed_input);
        assert_eq!(result, 195);
    }

    #[test]
    fn day11_part2() {
        let input = include_str!("../input/2021/day11.txt");
        let parsed_input = input_day11(input);
        let result = solve_part2(parsed_input);
        dbg!(result);
    }
}
