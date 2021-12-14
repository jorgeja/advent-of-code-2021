use std::collections::HashSet;


fn input_day13(input: &str) -> (HashSet<(usize, usize)>, Vec<(usize, usize)>)  {
    let mut dots = HashSet::new();
    let mut fold_actions = Vec::new();
    
    for line in input.lines() {
        if line.contains("fold") {
            if let Some(raw_fold_action) = line.split("fold along ").nth(1) {
                let mut action_elems = raw_fold_action.split("=");
                let dir = action_elems.next().unwrap();
                let val = action_elems.next().unwrap().parse::<usize>().ok().unwrap();
                match dir {
                    "x" => fold_actions.push((val, 0)),
                    "y" => fold_actions.push((0, val)),
                    _ => unreachable!(),
                }
            }
        } else if !line.is_empty() {
            let mut split = line.split(",");
            let first_digit = split.next().unwrap().parse::<usize>().ok().unwrap();
            let second_digit = split.next().unwrap().parse::<usize>().ok().unwrap();
            dots.insert((first_digit, second_digit));
        }
    }
    (dots, fold_actions)
}

fn solve_transform(dots: HashSet<(usize, usize)>, transform: (usize, usize)) -> HashSet<(usize, usize)> {
    let mut new_dots = HashSet::new();
    for dot in dots.iter() {
        let new_x = if dot.0 > transform.0 && transform.0 > 0 {
            dot.0 - ((dot.0 - transform.0) * 2)
        } else {
            dot.0
        };
        let new_y = if dot.1 > transform.1 && transform.1 > 0 {
            dot.1 - ((dot.1 - transform.1) * 2)
        } else {
            dot.1
        };
        new_dots.insert((new_x, new_y));
    }
    new_dots
}

fn solve_part1(dots: HashSet<(usize, usize)>, fold_actions: Vec<(usize, usize)>) -> usize {    
    let new_dots = solve_transform(dots, *fold_actions.first().unwrap());
    new_dots.len()
}

fn solve_part2(mut dots: HashSet<(usize, usize)>, fold_actions: Vec<(usize, usize)>) {
    for transform in fold_actions {
        dots = solve_transform(dots, transform);
    }

    let mut max_x = 0;
    let mut max_y = 0;
    for elem in dots.iter() {
        if elem.0 > max_x {
            max_x = elem.0;
        }
        if elem.1 > max_y {
            max_y = elem.1;
        }
    }

    for y in 0..=max_y {
        for x in 0..=max_x {
            if dots.contains(&(x, y)) {
                eprint!("#");
            } else {
                eprint!(".");
            }
        }
        eprint!("\n");
    }

}

#[cfg(test)]
mod test_day13 {
    use super::{input_day13, solve_part1, solve_part2};
    #[test]
    fn day13_part1_test() {
        let input = include_str!("../input/2021/day13_test.txt");
        let (dots, fold_actions) = input_day13(input);
        let result = solve_part1(dots, fold_actions);
        dbg!(result);
    }
    #[test]
    fn day13_part1() {
        let input = include_str!("../input/2021/day13.txt");
        let (dots, fold_actions) = input_day13(input);
        let result = solve_part1(dots, fold_actions);
        dbg!(result);
    }

    #[test]
    fn day13_part2_test() {
        let input = include_str!("../input/2021/day13_test.txt");
        let (dots, fold_actions) = input_day13(input);
        solve_part2(dots, fold_actions);
    }

    #[test]
    fn day13_part2() {
        let input = include_str!("../input/2021/day13.txt");
        let (dots, fold_actions) = input_day13(input);
        solve_part2(dots, fold_actions);
    }
}
