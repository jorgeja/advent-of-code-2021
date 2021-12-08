
type SegmentNote = (Vec<String>, Vec<String>);
fn input_day8(input: &str) -> Vec<SegmentNote> {
    input.lines().filter_map(|s|{
        let mut notes = s.split(" | ");
        let sequence = notes.next()?.split(" ").map(|s| s.into()).collect::<Vec<String>>();
        let output = notes.next()?.split(" ").map(|s| s.into()).collect::<Vec<String>>();
        Some((sequence, output))
    }).collect()
}

fn unique_number(s: &str) -> bool {
    match s.len() {
        2 | 3 | 4 | 7 => true,        
        _ => false,
    }
}

fn solve_part1(input: &[SegmentNote]) -> u32 {
    let mut num_unique = 0;
    for sm in input {
        for s in &sm.1 {
            if unique_number(s.as_str()) {
                num_unique += 1;
            }
        }
    }

    num_unique
}

const ORIG_SEGMENTS: [(&str, usize); 10] = [
    ("abcefg", 0),
    ("cf", 1),
    ("acdeg", 2),
    ("acfg", 3),
    ("bcdf", 4),
    ("abdfg", 5),
    ("abdfg", 6),
    ("acf", 7),
    ("abcdefg", 8),
    ("abcdfg", 9),
];

fn solve_part2(input: &[SegmentNote]) -> u32 {
    let parsed_orig_segments = ORIG_SEGMENTS.iter().map(|(s, v)| (*v, parse_sequence(s))).collect::<HashMap<_,_>>();
    for segment_note in input {
        solve_segment(segment_note, &parsed_orig_segments)
    }


    0
}

use std::collections::{HashMap};

fn parse_sequence(s: &str) -> usize {
    let mut mask = 0;
    for char in s.chars() {
        match char {
            'a' => mask |= 1 << 6,
            'b' => mask |= 1 << 5,
            'c' => mask |= 1 << 4,
            'd' => mask |= 1 << 3,
            'e' => mask |= 1 << 2,
            'f' => mask |= 1 << 1,
            'g' => mask |= 1 << 0,
            _ => unreachable!(),            
        }
    }
    mask
}

fn solve_segment(note: &SegmentNote, _original_masks: &HashMap<usize, usize>) {
    let _parsed_sequence = note.0.iter().map(|s| parse_sequence(s)).collect::<Vec<usize>>();    
}

#[cfg(test)]
mod test_day8 {
    use super::{input_day8, solve_part1, solve_part2};
    
    #[test]
    fn day8_part1_test() {
        let input = include_str!("../input/2021/day8_test.txt");
        let parsed_input = input_day8(input);
        let result = solve_part1(&parsed_input);
        dbg!(result);
        assert_eq!(result, 26)
    }
    #[test]
    fn day8_part1() {
        let input = include_str!("../input/2021/day8.txt");
        let parsed_input = input_day8(input);
        let result = solve_part1(&parsed_input);
        dbg!(result);
    }

    #[test]
    fn day8_part2_test() {
        let input = include_str!("../input/2021/day8_test_small.txt");
        let parsed_input = input_day8(input);
        let result = solve_part2(&parsed_input);
        dbg!(result);
        assert_eq!(result, 26)
    }

    #[test]
    fn day8_part2() {
        let input = include_str!("../input/2021/day8.txt");
        let parsed_input = input_day8(input);
        let result = solve_part2(&parsed_input);
        dbg!(result);
    }
}
