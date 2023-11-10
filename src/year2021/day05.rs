use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn get_start_end(point_1: u32, point_2: u32) -> (u32, u32) {
    if point_1 < point_2 {
        (point_1, point_2)
    } else {
        (point_2, point_1)
    }
}

pub fn get_calc_num(point_1: u32, point_2: u32) -> i8 {
    if point_1 > point_2 {
        return -1;
    } else if point_1 < point_2 {
        return 1;
    }
    0
}

pub fn solution_a() -> String {
    let file = File::open("src/year2021/day05_input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut points: HashMap<String, u32> = HashMap::new();

    for line in reader.lines() {
        let coordinates: Vec<&str> = line.as_ref().unwrap().split(" -> ").collect();
        let from: Vec<u32> = coordinates[0]
            .split(',')
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        let to: Vec<u32> = coordinates[1]
            .split(',')
            .map(|s| s.parse::<u32>().unwrap())
            .collect();

        if from[0] == to[0] {
            let (start, end) = get_start_end(from[1], to[1]);
            for y in start..=end {
                let point_key = format!("{},{}", from[0], y);
                *points.entry(point_key).or_insert(0) += 1;
            }
        } else if from[1] == to[1] {
            let (start, end) = get_start_end(from[0], to[0]);
            for x in start..=end {
                let point_key = format!("{},{}", x, from[1]);
                *points.entry(point_key).or_insert(0) += 1;
            }
        }
    }

    let mut dangerous_points = 0;
    for (_, val) in points.iter() {
        if val >= &(2 as u32) {
            dangerous_points += 1;
        }
    }
    format!("{}", dangerous_points)
}

pub fn solution_b() -> String {
    let file = File::open("src/year2021/day05_input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut points: HashMap<String, u32> = HashMap::new();

    for line in reader.lines() {
        let coordinates: Vec<&str> = line.as_ref().unwrap().split(" -> ").collect();
        let from: Vec<u32> = coordinates[0]
            .split(',')
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        let to: Vec<u32> = coordinates[1]
            .split(',')
            .map(|s| s.parse::<u32>().unwrap())
            .collect();

        if from[0] == to[0] {
            let (start, end) = get_start_end(from[1], to[1]);
            for y in start..=end {
                let point_key = format!("{},{}", from[0], y);
                *points.entry(point_key).or_insert(0) += 1;
            }
        } else if from[1] == to[1] {
            let (start, end) = get_start_end(from[0], to[0]);
            for x in start..=end {
                let point_key = format!("{},{}", x, from[1]);
                *points.entry(point_key).or_insert(0) += 1;
            }
        } else {
            let x_step = get_calc_num(from[0], to[0]);
            let y_step = get_calc_num(from[1], to[1]);
            let mut x_cur = from[0] as i32;
            let mut y_cur = from[1] as i32;

            while x_cur as u32 != to[0] && y_cur as u32 != to[1] {
                let point_key = format!("{},{}", x_cur, y_cur);
                *points.entry(point_key).or_insert(0) += 1;
                x_cur += x_step as i32;
                y_cur += y_step as i32;
            }
            let point_key = format!("{},{}", to[0], to[1]);
            *points.entry(point_key).or_insert(0) += 1;
        }
    }

    let mut dangerous_points = 0;
    for (_, val) in points.iter() {
        if val >= &(2 as u32) {
            dangerous_points += 1;
        }
    }
    format!("{}", dangerous_points)
}