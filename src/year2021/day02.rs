use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solution_a() -> String {
    let file = File::open("src/year2021/day02_input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut horizontal: u32 = 0;
    let mut depth: u32 = 0;

    for line in reader.lines() {
        let pos_move: Vec<&str> = line.as_ref().unwrap().split(' ').collect();
        match pos_move[0] {
            "forward" => horizontal += pos_move[1].parse::<u32>().unwrap(),
            "down" => depth += pos_move[1].parse::<u32>().unwrap(),
            "up" => depth -= pos_move[1].parse::<u32>().unwrap(),
            _ => {}
        }
    }
    format!("{}", horizontal * depth)
}

pub fn solution_b() -> String {
    let file = File::open("src/year2021/day02_input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut horizontal: u32 = 0;
    let mut depth: u32 = 0;
    let mut aim: u32 = 0;

    for line in reader.lines() {
        let pos_move: Vec<&str> = line.as_ref().unwrap().split(' ').collect();
        let val: u32 = pos_move[1].parse::<u32>().unwrap();

        match pos_move[0] {
            "forward" => {
                horizontal += val;
                depth += aim * val;
            }
            "down" => aim += val,
            "up" => aim -= val,
            _ => {}
        }
    }
    format!("{}", horizontal * depth)
}
