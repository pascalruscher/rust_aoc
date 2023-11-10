use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solution_a() -> String {
    let file = File::open("src/year2021/day07_input.txt").unwrap();
    let mut reader = BufReader::new(file);

    let mut input = String::new();
    reader.read_line(&mut input).unwrap();

    let crabs: Vec<&str> = input.trim().split(',').collect();

    let mut crabs_sorted: HashMap<u32, u32> = HashMap::new();
    let mut pos_max: u32 = 0;
    let mut pos_min: u32 = u32::MAX;

    for crab in crabs {
        let position = crab.parse::<u32>().unwrap();
        *crabs_sorted.entry(position).or_insert(0) += 1;
        if position > pos_max {
            pos_max = position;
        }
        if position < pos_min {
            pos_min = position;
        }
    }

    let mut lowest_fuel_cost: u32 = u32::MAX;
    for tar_pos in pos_min..=pos_max {
        let mut fuel_cost: u32 = 0;
        for (cur_pos, count) in &crabs_sorted {
            fuel_cost += (*cur_pos as i32 - tar_pos as i32).abs() as u32 * count;
        }
        if fuel_cost < lowest_fuel_cost {
            lowest_fuel_cost = fuel_cost;
        }
    }
    format!("{}", lowest_fuel_cost)
}

pub fn solution_b() -> String {
    let file = File::open("src/year2021/day07_input.txt").unwrap();
    let mut reader = BufReader::new(file);

    let mut input = String::new();
    reader.read_line(&mut input).unwrap();

    let crabs: Vec<&str> = input.trim().split(',').collect();

    let mut crabs_sorted: HashMap<u32, u32> = HashMap::new();
    let mut pos_max: u32 = 0;
    let mut pos_min: u32 = u32::MAX;

    for crab in crabs {
        let position = crab.parse::<u32>().unwrap();
        *crabs_sorted.entry(position).or_insert(0) += 1;
        if position > pos_max {
            pos_max = position;
        }
        if position < pos_min {
            pos_min = position;
        }
    }

    let mut lowest_fuel_cost: u32 = u32::MAX;
    for tar_pos in pos_min..=pos_max {
        let mut fuel_cost: u32 = 0;
        for (cur_pos, count) in &crabs_sorted {
            let distance = (*cur_pos as i32 - tar_pos as i32).abs();
            for i in 1..=distance {
                fuel_cost += *count * i as u32;
            }
        }
        if fuel_cost < lowest_fuel_cost {
            lowest_fuel_cost = fuel_cost;
        }
    }
    format!("{}", lowest_fuel_cost)
}

