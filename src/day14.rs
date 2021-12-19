use std::collections::HashMap;
fn input_day14(input: &str) -> (String, HashMap<&[u8], &[u8]>) {
    let mut line_iterator = input.lines();
    let start_sentence = line_iterator.next().unwrap().into();
    line_iterator.next();
    let mut translator = HashMap::new();
    for line in line_iterator {
        let mut split = line.split(" -> ");
        let key = split.next().unwrap().as_bytes().clone();
        let val = split.next().unwrap().as_bytes().clone();
        translator.insert(key, val);
    }
    (start_sentence, translator) 
}

fn contains(whole: &[u8], sub_section: &[u8]) -> usize {
    let mut num_matches = 0;
    let mut index = 0;
    for byte in whole {
        if index < sub_section.len() && *byte == sub_section[index] {
            index += 1;
        } else if index == sub_section.len() {
            num_matches += 1;
            index = 0;
        } else {
            index = 0;
        }
    }

    if index == sub_section.len() {
        num_matches += 1;
    }

    num_matches
}

fn solve_part1(start_sentence: String, translator: HashMap<&[u8], &[u8]>, steps: usize) -> usize {
    eprintln!("Start: {}", start_sentence);
    let mut sentence = start_sentence.as_bytes().iter().cloned().collect::<Vec<u8>>();
    let mut former_solutions: Vec<Vec<u8>> = Vec::new();
    let mut counter = HashMap::new();

    for _step in 0..steps {
        let mut new_sentence = Vec::new();

        eprintln!("Step: {}", _step + 1);
        eprintln!("Sentence length: {} estimated next_length: {}", sentence.len(), (sentence.len() -1 ) + sentence.len());

        new_sentence.push(sentence[0]);
        for chars in sentence.windows(2) {
            if let Some(insert) = translator.get(chars) {                
                new_sentence.push(insert[0]);
                new_sentence.push(chars[1]);
            }
        }
        former_solutions.push(sentence.clone());
        let mut new_counter = HashMap::new();       
        for char in new_sentence.iter() {                        
            *new_counter.entry(*char).or_insert(0) += 1;            
        }

        sentence = new_sentence;
        
        for (char, num) in new_counter.iter() {
            eprintln!("{}: {} : {}", *char as char, *num, sentence.len() / *num);
            *counter.entry(*char).or_insert(0) += *num;
        }
    }

    let mut counts = counter.iter().map(|(_, v)| *v).collect::<Vec<usize>>();
    counts.sort();
    counts.last().unwrap() - counts.first().unwrap()
}


#[cfg(test)]
mod test_day14 {
    use super::{input_day14, solve_part1, contains};
    #[test]
    fn day14_part1_test() {
        let input = include_str!("../input/2021/day14_test.txt");
        let (start_sentence, translator) = input_day14(input);
        let result = solve_part1(start_sentence, translator, 10);
        dbg!(result);
    }
    #[test]
    fn day14_part1() {
        let input = include_str!("../input/2021/day14.txt");
        let (start_sentence, translator) = input_day14(input);
        let result = solve_part1(start_sentence, translator, 20);
        dbg!(result);
    }

    #[test]
    fn day14_part2() {
        let input = include_str!("../input/2021/day14.txt");
        let (start_sentence, translator) = input_day14(input);
        let result = solve_part1(start_sentence, translator, 40);
        dbg!(result);
    }

    #[test]
    fn test_contains() {
        let first = "Hello World; Goodbye World".as_bytes();
        let second = "World".as_bytes();
        let num = contains(first, second);
        dbg!(num);
    }
}
