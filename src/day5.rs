
type Coord = (i32, i32);
#[derive(Debug, Clone, Copy)]
struct Line {
    c1: Coord,
    c2: Coord
}
// Option 1: Create a vec of all positions on the Line
impl Line {
    fn all_coords(&self) -> Vec<Coord> {        
        let v_length = self.c2.0 - self.c1.0;
        let h_length = self.c2.1 - self.c1.1;

        let v_1 = v_length.clamp(-1, 1);
        let v_2 = h_length.clamp(-1, 1);
        let index_factor = (v_1, v_2);
        
        let length = if v_length != 0 { v_length.abs() } else { h_length.abs() } + 1;
        (0..length).map(|i| (self.c1.0 + i * index_factor.0, self.c1.1 + i * index_factor.1)).collect()                    
    }
}

// Option 2: Use Rusts iterator magic. Lazy evaluated.
//Same work as in "line::all_coords", but returns a LineIterator that implements Iterators::next method
impl IntoIterator for Line {
    type Item = Coord;
    type IntoIter = LineIterator;
    fn into_iter(self) -> Self::IntoIter {
        let v_length = self.c2.0 - self.c1.0;
        let h_length = self.c2.1 - self.c1.1;

        let v_1 = v_length.clamp(-1, 1);
        let v_2 = h_length.clamp(-1, 1);
        let index_factor = (v_1, v_2);
        
        let length = if v_length != 0 { v_length.abs() } else { h_length.abs() } + 1;
        
        LineIterator {
            start: self.c1.clone(),
            length,
            index_factor,
            pos: 0,
        }
    }
}

// The line iterator, keeps information about the state of the line and the initial conditions we need.
struct LineIterator{
    start: Coord,
    length: i32,    
    index_factor: (i32, i32),
    pos: i32,
}

//
impl Iterator for LineIterator {
    type Item = Coord;
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos == self.length {
            None
        } else {
            let item = (self.start.0 + self.pos * self.index_factor.0, self.start.1 + self.pos * self.index_factor.1).into();
            self.pos += 1;
            item
        }                
    }
}





fn input_day5(input: &str, allow_diagonal: bool) -> Vec<Line> {
    input.lines().filter_map(|s| {
        let mut segments = s.split(" -> ");
        let mut first_pair = segments.next()?.split(',');
        let c1_1 = first_pair.next()?.parse::<i32>().ok()?;
        let c1_2 = first_pair.next()?.parse::<i32>().ok()?;
        let mut second_pair = segments.next()?.split(',');
        let c2_1 = second_pair.next()?.parse::<i32>().ok()?;
        let c2_2 = second_pair.next()?.parse::<i32>().ok()?;

        if c1_1 == c2_1 || c1_2 == c2_2 || allow_diagonal {
            return Line {
                c1: (c1_1, c1_2),
                c2: (c2_1, c2_2),
            }.into()
        }
        None        
    }).collect()
}

use std::collections::HashMap;

fn solve(lines: &[Line]) -> u32 {
    let mut map = HashMap::new();
    lines
        .iter()
        .for_each(|line| 
            line.into_iter()
            .for_each(|c|
                *map.entry(c.clone()).or_insert(0) += 1 
            )
        );
    map.values().fold(0, |mut acc, v| {if *v > 1 {acc += 1}; acc})
}

#[cfg(test)]
mod test_day5 {
    use super::{input_day5, solve};
    #[test]
    fn test_day5_part1() {
        let input = include_str!("../input/2021/day5_part1_test.txt");
        let parsed_input = input_day5(input, false);        
        let result = solve(&parsed_input);        
        assert_eq!(result, 5);
    }

    #[test]
    fn day5_part1() {
        let input = include_str!("../input/2021/day5.txt");
        let parsed_input = input_day5(input, false);
        let result = solve(&parsed_input);
        dbg!(result);
    }

    #[test]
    fn test_day5_part2() {
        let input = include_str!("../input/2021/day5_part1_test.txt");
        let parsed_input = input_day5(input, true);        
        let result = solve(&parsed_input);
        assert_eq!(result, 12);
    }

    #[test]
    fn day5_part2() {
        let input = include_str!("../input/2021/day5.txt");
        let parsed_input = input_day5(input, true);
        let result = solve(&parsed_input);
        dbg!(result);
    }
}
