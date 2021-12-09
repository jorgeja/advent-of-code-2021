fn input_day9(input: &str) -> Vec<Vec<usize>> {
    input.lines().map(|s| s.chars().map(|c| c.to_digit(10).unwrap() as usize).collect()).collect()
}

fn solve_part1(input: Vec<Vec<usize>>) -> usize {
    let mut sum_low_points = 0;
    let offsets = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    for row in 0..input.len() as i32 {
        for col in 0..input[row as usize].len() as i32 {
            let v = input[row as usize][col as usize];
            let mut low_neighbors = 0;
            eprintln!("checking {}", v);

            for (o_r, o_c) in offsets {
                let r = row + o_r;
                let c = col + o_c;                    
                if ((r < 0 || r >= input.len() as i32) && r != row) 
                || ((c < 0 || c >= input[row as usize].len() as i32) && c != col) 
                || (v < input[r as usize][c as usize]){
                    low_neighbors += 1;
                }
            }
            
            if low_neighbors == 4 {
                eprintln!("Low point at {} {} : {}", row, col, v);
                sum_low_points += 1 + v;
            }
        }
    }

    sum_low_points
}

fn solve_part2(input: Vec<Vec<usize>>) -> u32 {
    0
}

#[cfg(test)]
mod test_day9 {
    use super::{input_day9, solve_part1, solve_part2};
    #[test]
    fn day9_part1_test() {
        let input = include_str!("../input/2021/day9_test.txt");
        let parsed_input = input_day9(input);
        let result = solve_part1(parsed_input);
        dbg!(result);
    }

    #[test]
    fn day9_part1() {
        let input = include_str!("../input/2021/day9.txt");
        let parsed_input = input_day9(input);
        let result = solve_part1(parsed_input);
        dbg!(result);
    }
    #[test]
    fn day9_part2() {
        let input = include_str!("../input/2021/day9.txt");
        let parsed_input = input_day9(input);
        let result = solve_part2(parsed_input);
        dbg!(result);
    }
}
