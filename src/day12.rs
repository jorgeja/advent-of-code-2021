use std::{collections::{HashMap, HashSet}, vec};

fn input_day12(input: &str) -> HashMap<String, Vec<String>> {
    let mut edges = HashMap::new();
    for line in input.lines() {
        let mut elems = line.split("-");
        let first = elems.next().unwrap();
        let second = elems.next().unwrap();
        edges.entry(first.into()).or_insert(Vec::new()).push(second.into());
        edges.entry(second.into()).or_insert(Vec::new()).push(first.into());
    }
    edges
}


fn is_all_caps(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_uppercase())
}

fn count_entries(s: &str, path: &[String]) -> usize {
    path.iter().fold(0, | mut v, path_s| {
        if s == path_s {
            v += 1;
            v
        } else {
            v
        }
    }) as usize
}

fn check_paths(node: &str, path: Vec<String>, complete_paths: &mut HashSet<String>, edges: &HashMap<String, Vec<String>>, max_small_cave_visits: usize) {    
    if let Some(neighbor_nodes) = edges.get(node) {
        for n_node in neighbor_nodes {
            if n_node == "end" {       
                let mut complete_path = path.join(",");
                complete_path += ",end";                
                complete_paths.insert(complete_path);
            }
            else if is_all_caps(n_node) || count_entries(n_node, &path) < max_small_cave_visits {
                let mut sub_path = path.clone();
                sub_path.push(n_node.clone());
                check_paths(n_node, sub_path, complete_paths, edges, max_small_cave_visits);
            } 
        }
    }
    
}

fn solve_part1(edges: HashMap<String, Vec<String>>) -> usize {
    let mut complete_paths = HashSet::new();
    let path = vec!["start".into()];
    check_paths("start", path, &mut complete_paths, &edges, 1);    
    complete_paths.len()
}

#[derive(Clone,Debug)]
struct Path {
    elements: Vec<String>,
    has_visited_small_cave: bool,
}

fn check_valid_cave(node: &String, path: &mut Path) -> bool {
    if is_all_caps(node) {
        true
    } else if path.has_visited_small_cave && path.elements.contains(node) {
        false
    } else if !path.has_visited_small_cave {                
        true
    } else {
        true
    }
}

fn solve_part2(edges: HashMap<String, Vec<String>>) -> usize {
    let mut pending_paths = Vec::new();
    let mut complete_paths  = Vec::new();
    for start_node in edges.get("start").unwrap() {
        pending_paths.push(Path{
            elements: vec!["start".into(), start_node.clone()], 
            has_visited_small_cave: false,
        });
    }
    loop {                
        let mut new_pending_paths = Vec::new();
        for path in pending_paths.iter_mut() {
            if let Some(neighbor_nodes) = edges.get(path.elements.last().unwrap()) {
                for n_node in neighbor_nodes {                    
                    if n_node == "end" {
                        let mut p = path.clone();
                        p.elements.push(n_node.clone());
                        complete_paths.push(p);
                    }
                    else if n_node == "start"{
                        continue;
                    }
                    else if check_valid_cave(n_node, path){                        
                        let mut p = path.clone();
                        if !p.has_visited_small_cave && !is_all_caps(n_node) && p.elements.contains(n_node) {
                            p.has_visited_small_cave = true;
                        }
                        p.elements.push(n_node.clone());
                        new_pending_paths.push(p);
                    }
                }
            }
        }

        if new_pending_paths.len() > 0 {
            pending_paths = new_pending_paths;
        } else {
            break;
        }
    }
    
    let unique_paths = complete_paths.iter().map(|elems| elems.elements.join(",")).collect::<HashSet<String>>();
    unique_paths.len()
}

#[cfg(test)]
mod test_day12 {

    use super::{input_day12, solve_part1, solve_part2};
    #[test]
    fn day12_part1_small_test() {
        let input = include_str!("../input/2021/day12_test_small.txt");
        let parsed_input = input_day12(input);
        let result = solve_part1(parsed_input);
        dbg!(result);
    }
    
    #[test]
    fn day12_part1_test() {
        let input = include_str!("../input/2021/day12_test.txt");
        let parsed_input = input_day12(input);
        let result = solve_part1(parsed_input);
        dbg!(result);
    }

    #[test]
    fn day12_part1() {
        let input = include_str!("../input/2021/day12.txt");
        let parsed_input = input_day12(input);
        let result = solve_part1(parsed_input);
        dbg!(result);
    }
    #[test]
    fn day12_part2_small_test() {
        let input = include_str!("../input/2021/day12_test_small.txt");
        let parsed_input = input_day12(input);
        let result = solve_part2(parsed_input);
        dbg!(result);
    }

    #[test]
    fn day12_part2_small_test_2() {
        let input = include_str!("../input/2021/day12_test_small.txt");
        let parsed_input = input_day12(input);
        let result = solve_part2(parsed_input);
        dbg!(result);
    }

    #[test]
    fn day12_part2() {
        let input = include_str!("../input/2021/day12.txt");
        let parsed_input = input_day12(input);
        let result = solve_part2(parsed_input);
        dbg!(result);
    }
}
