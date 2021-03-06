
fn input_day9(input: &str) -> Vec<Vec<usize>> {
    input.lines().map(|s| s.chars().map(|c| c.to_digit(10).unwrap() as usize).collect()).collect()
}

const OFFSETS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn solve_part1(input: Vec<Vec<usize>>) -> usize {
    let mut sum_low_points = 0;    
    for row in 0..input.len() as i32 {
        for col in 0..input[row as usize].len() as i32 {
            let v = input[row as usize][col as usize];
            let mut low_neighbors = 0;
            eprintln!("checking {}", v);

            for (o_r, o_c) in OFFSETS {
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

fn find_neighbor_lowpoints(row: i32, col: i32, input: &Vec<Vec<usize>>) -> Vec<(i32, i32)>{
    let v = input[row as usize][col as usize];
    OFFSETS.iter().filter_map(|(o_r, o_c)| {
        let r = row + o_r;
        let c = col + o_c; 
        
        if r < 0 || r >= input.len() as i32 || c < 0 || c >= input[row as usize].len() as i32 {
            return None;
        }

        let nv = input[r as usize][c as usize];
        if (v < nv || nv < v) && nv < 9 { 
            Some((r, c))
        } else {
            None
        }
    }).collect()  
}

use std::collections::HashSet;

fn solve_part2(input: Vec<Vec<usize>>) -> u32 {
    let mut in_basin = HashSet::new();
    let mut basins = Vec::new();
    for row in 0..input.len() as i32 {
        for col in 0..input[row as usize].len() as i32 {
            if !in_basin.contains(&(row, col)) {
                let mut num_basin_locations = 0;            
                let mut pending_locations = vec![(row.clone(), col.clone())];
               
                loop {
                    let mut new_pending_locations = Vec::new();
                    for (row, col) in pending_locations.iter() {
                        if !in_basin.contains(&(*row, *col)) {                            
                            in_basin.insert((*row, *col));
                            if input[*row as usize][*col as usize] < 9 {
                                new_pending_locations.extend(find_neighbor_lowpoints(*row, *col, &input));
                                num_basin_locations += 1;                                                        
                            }                                                        
                        }                        
                    }
                    if new_pending_locations.len() == 0 {
                        break;
                    } else {                        
                        pending_locations = new_pending_locations;
                    }                                
                } 
                basins.push(num_basin_locations);           
            } else {

            }
        }
    }
    basins.sort();
    let l = basins.len();
    if l >= 3 {
        basins[l-1] * basins[l-2] * basins[l-3]    
    } else {
        0
    }
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
    fn day9_part2_test() {
        let input = include_str!("../input/2021/day9_test.txt");
        let parsed_input = input_day9(input);
        let result = solve_part2(parsed_input);
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
