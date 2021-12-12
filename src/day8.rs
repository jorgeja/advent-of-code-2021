use std::collections::{HashSet, HashMap};


type SegmentNote = (Vec<String>, Vec<String>);
fn input_day8(input: &str) -> Vec<SegmentNote> {
    input.lines().filter_map(|s|{
        let mut notes = s.split(" | ");
        let sequence = notes.next()?.split(" ").map(|s| s.trim().into()).collect::<Vec<String>>();
        let output = notes.next()?.split(" ").map(|s| s.trim().into()).collect::<Vec<String>>();
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

fn solve_part2(input: &[SegmentNote]) -> usize {
    let orig_segment_diffs = get_orig_segment_diffs();
    let mut sum = 0;
    for segment_note in input {
        sum += solve_segment(segment_note, &orig_segment_diffs);
    }
    sum
}

fn find_differences_in_input(input: &[String]) -> Vec<(HashSet<char>, Vec<usize>)>{
    let sets = input.iter().map(|s| s.chars().collect::<HashSet<char>>()).collect::<Vec<_>>();
    let mut diffs = Vec::new();
    for set_first in sets.iter() {
        let mut set_diff = Vec::new();
        for set_second in sets.iter() {
            let diff = set_first.symmetric_difference(set_second).count();
            set_diff.push(diff);
        }
        set_diff.sort();
        diffs.push((set_first.clone(), set_diff));
    }
    diffs   
}

fn build_translator(input_diffs: Vec<(HashSet<char>, Vec<usize>)>, orig_segment_diffs: &HashMap<Vec<usize>, usize>) -> Vec<(HashSet<char>, usize)> {
    let mut translator = Vec::new();
    for (s, diff) in input_diffs {
        if let Some(known_diff_value) = orig_segment_diffs.get(&diff) {
            translator.push((s, *known_diff_value));            
        }
    }
    translator
}

fn solve_segment(note: &SegmentNote, orig_segment_diffs: &HashMap<Vec<usize>, usize>) -> usize {   
    
    let diffs = find_differences_in_input(&note.0);
    let translator = build_translator(diffs, orig_segment_diffs);

    let mut sum = 0;
    for (i, unknown) in note.1.iter().rev().enumerate() {
        let hashed_value = unknown.chars().collect::<HashSet<char>>();
        for (known_hashed, value) in translator.iter() {
            if *known_hashed == hashed_value {
                let val = 10usize.pow(i as u32) * value; 
                sum += val;
                break;
            }
        }
    }

    sum
}

const ORIG_SEGMENTS: [(&str, usize); 10] = [
    ("abcefg", 0),
    ("cf", 1),
    ("acdeg", 2),
    ("acdfg", 3),
    ("bcdf", 4),
    ("abdfg", 5),
    ("abdefg", 6),
    ("acf", 7),
    ("abcdefg", 8),
    ("abcdfg", 9),
];

fn get_orig_segment_diffs() -> HashMap<Vec<usize>, usize> {
    //Each output is unique, and the difference between each segment and all other segment is unique. Use this difference as key to the correct value.
    let sets = ORIG_SEGMENTS.iter().map(|(s, num)|{
        (*num, s.chars().collect::<HashSet<char>>())
    }).collect::<Vec<(usize, HashSet<char>)>>();

    let mut num_to_diffs = HashMap::new();
    for (num, s_first) in sets.iter() {
        let mut diffs = Vec::new();       
        for (_, s_second) in sets.iter() {
            let diff = s_first.symmetric_difference(s_second).count();
            diffs.push(diff);
        }
        diffs.sort();
        num_to_diffs.insert(diffs, *num);
    }

    num_to_diffs
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
    fn day8_part2_test_small() {
        let input = include_str!("../input/2021/day8_test_small.txt");
        let parsed_input = input_day8(input);
        let result = solve_part2(&parsed_input);
        assert_eq!(result, 5353);
    }

    #[test]
    fn day8_part2_test() {
        let input = include_str!("../input/2021/day8_test.txt");
        let parsed_input = input_day8(input);
        let result = solve_part2(&parsed_input);
        assert_eq!(result, 61229);
    }

    #[test]
    fn day8_part2() {
        let input = include_str!("../input/2021/day8.txt");
        let parsed_input = input_day8(input);
        let result = solve_part2(&parsed_input);
        dbg!(result);
    }
}
