use std::collections::HashMap;

fn error_bonus (closing_char: u8) -> usize{
    match closing_char {
        b'}'=> 1197,
        b']'=> 57,
        b')'=> 3,
        b'>'=> 25137,
        _ => 0, 
    }
}

fn closing_char(opening_char: u8) -> u8 {
    match opening_char {
        b'{'=> b'}',
        b'['=> b']',
        b'('=> b')',
        b'<'=> b'>',
        _ => unreachable!(),
    }
}

fn solve_code_line_part1(line: &str) -> usize {
    let mut pending_closing_chars = Vec::new();
    for c in line.bytes() {
        match c {
            b'{'|b'['|b'('|b'<' => pending_closing_chars.push(closing_char(c)),
            b'}'|b']'|b')'|b'>' => {
                if let Some(last_closing_char) = pending_closing_chars.pop() {
                    if last_closing_char != c {
                        eprintln!("Expected {}, but found {} instead", last_closing_char as char, c as char);
                        return error_bonus(c);
                    }
                } else {
                    break;
                }                
            }
            _ => break,
        }
    }
    0
}

fn closing_char_bonus (closing_char: u8) -> usize{
    match closing_char {
        b'}'=> 3,
        b']'=> 2,
        b')'=> 1,
        b'>'=> 4,
        _ => 0, 
    }
}

fn solve_code_line_part2(line: &str) -> usize {
    let mut pending_closing_chars = Vec::new();
    for c in line.bytes() {
        match c {
            b'{'|b'['|b'('|b'<' => pending_closing_chars.push(closing_char(c)),
            b'}'|b']'|b')'|b'>' => {
                if let Some(last_closing_char) = pending_closing_chars.pop() {
                    if last_closing_char != c {                        
                        return 0;
                    }
                } else {
                    break;
                }                
            }
            _ => break,
        }
    }

    if pending_closing_chars.len() > 0 {
        let mut sum = 0;
        for char in pending_closing_chars.iter().rev() {
            sum = sum * 5 + closing_char_bonus(*char)
        }
        return sum;
    }
    0
}

fn solve_part1(input: &str) -> u32 {    
    let mut score = HashMap::new();
    for line in input.lines() {
        let error_code = solve_code_line_part1(line);
        if error_code > 0 {
            *score.entry(error_code).or_insert(0) += 1;
        }
    }

    score.iter().fold(0, | mut acc, (char_score, num )|{
        acc += *char_score as u32 * *num as u32;
        acc
    })
}

fn solve_part2(input: &str) -> usize {
   let mut scores = input.lines().filter_map(|l| {
        let s = solve_code_line_part2(l);
        if 0 < s {
            Some(s)
        } else {
            None
        }
       
   }).collect::<Vec<_>>();
   scores.sort();
   let middle = scores.len() / 2;
   scores[middle]
}

#[cfg(test)]
mod test_day10 {
    use super::{solve_part1, solve_part2};
    
    #[test]
    fn day10_part1_test() {
        let input = include_str!("../input/2021/day10_test.txt");        
        let result = solve_part1(&input);
        dbg!(result);
    }

    #[test]
    fn day10_part1() {
        let input = include_str!("../input/2021/day10.txt");        
        let result = solve_part1(&input);
        dbg!(result);
    }

    #[test]
    fn day10_part2_test() {
        let input = include_str!("../input/2021/day10_test.txt");        
        let result = solve_part2(&input);
        dbg!(result);
    }

    #[test]
    fn day10_part2() {
        let input = include_str!("../input/2021/day10.txt");
        let result = solve_part2(&input);
        dbg!(result);
    }
}
