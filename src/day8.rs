
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
    0
}

fn solve_segment(note: &SegmentNote) {
    let mut numbers = Vec::new();
    let mut possible_char_pos = Vec::new();
    for s in &note.0 {
        match s.len() {
            2 => {
                numbers.push((s.clone(), 1));
                let mut chars = s.chars();
                possible_char_pos.push((chars.next().unwrap(), vec!['c', 'f']));
                possible_char_pos.push((chars.next().unwrap(), vec!['c', 'f']));
            }
            3 => {
                numbers.push((s.clone(), 7));
                let mut chars = s.chars();
                possible_char_pos.push((chars.next().unwrap(), vec!['a', 'c', 'f']));
                possible_char_pos.push((chars.next().unwrap(), vec!['', 'c', 'f']));
                possible_char_pos.push((chars.next().unwrap(), vec!['c', 'f']));
            }
        }
    }
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
    fn day8_part2() {
        let input = include_str!("../input/2021/day8.txt");
        let parsed_input = input_day8(input);
        let result = solve_part2(&parsed_input);
        dbg!(result);
    }
}
