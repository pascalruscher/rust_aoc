use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn count_paths_a(
    possible_moves: &HashMap<String, Vec<String>>,
    visited_small_caves: &Vec<String>,
    path: Vec<String>,
) -> u32 {
    let current_position = &path[path.len() - 1];
    if current_position == "end" {
        return 1;
    }

    let mut count = 0;
    let mut possible_path = path.clone();

    for possible_move in possible_moves.get(current_position).unwrap() {
        let mut visited_small_caves = visited_small_caves.clone();
        if !possible_move.chars().any(|c| matches!(c, 'A'..='Z')) {
            if visited_small_caves.contains(&possible_move) {
                continue;
            } else {
                visited_small_caves.push(possible_move.clone());
            }
        }
        possible_path.push(possible_move.clone());
        count += count_paths_a(possible_moves, &visited_small_caves, possible_path.clone());
    }
    count
}

pub fn count_paths_b(
    possible_moves: &HashMap<String, Vec<String>>,
    visited_small_caves: &Vec<String>,
    path: Vec<String>,
) -> u32 {
    let current_position = &path[path.len() - 1];
    if current_position == "end" {
        return 1;
    }

    let mut count = 0;
    let mut possible_path = path.clone();

    for possible_move in possible_moves.get(current_position).unwrap() {
        let mut visited_small_caves = visited_small_caves.clone();
        if !possible_move.chars().any(|c| matches!(c, 'A'..='Z')) {
            let mut can_visit_twice = true;
            visited_small_caves.sort();
            if possible_move != "start" && possible_move != "end" {
                for (i, x) in visited_small_caves.iter().enumerate() {
                    if i > 0 && &visited_small_caves[i - 1] == x {
                        can_visit_twice = false;
                        break;
                    }
                }
            } else {
                can_visit_twice = false;
            }
            if can_visit_twice || !visited_small_caves.contains(&possible_move) {
                visited_small_caves.push(possible_move.clone());
            } else {
                continue;
            }
        }
        possible_path.push(possible_move.clone());
        count += count_paths_b(possible_moves, &visited_small_caves, possible_path.clone());
    }
    count
}

pub fn solution_a() -> String {
    let file = File::open("src/year2021/day12_input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut possible_moves: HashMap<String, Vec<String>> = HashMap::new();
    for line in reader.lines() {
        let parsed: Vec<String> = line
            .as_ref()
            .unwrap()
            .split('-')
            .map(|s| s.to_string())
            .collect();

        // Forward movement
        possible_moves
            .entry(parsed[0].clone())
            .or_insert(Vec::new())
            .push(parsed[1].clone());

        // Backward movement
        possible_moves
            .entry(parsed[1].clone())
            .or_insert(Vec::new())
            .push(parsed[0].clone());
    }

    let visited_small_caves: Vec<String> = vec!["start".to_string()];
    format!("{}", count_paths_a(
        &possible_moves,
        &visited_small_caves,
        vec!["start".to_string()],
    ))
}

pub fn solution_b() -> String {
    let file = File::open("src/year2021/day12_input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut possible_moves: HashMap<String, Vec<String>> = HashMap::new();
    for line in reader.lines() {
        let parsed: Vec<String> = line
            .as_ref()
            .unwrap()
            .split('-')
            .map(|s| s.to_string())
            .collect();

        // Forward movement
        possible_moves
            .entry(parsed[0].clone())
            .or_insert(Vec::new())
            .push(parsed[1].clone());

        // Backward movement
        possible_moves
            .entry(parsed[1].clone())
            .or_insert(Vec::new())
            .push(parsed[0].clone());
    }

    let visited_small_caves: Vec<String> = vec!["start".to_string()];
    format!("{}", count_paths_b(
        &possible_moves,
        &visited_small_caves,
        vec!["start".to_string()],
    ))
}
