use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solution_a() -> String {
    let file = File::open("src/year2021/day01_input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut prev_measurement: Option<u32> = None;
    let mut measurements = 0;

    for line in reader.lines() {
        let measurement: u32 = line.unwrap().parse().unwrap();
        if prev_measurement.is_none() {
            prev_measurement = Some(measurement);
            continue;
        }
        if prev_measurement.unwrap() < measurement {
            measurements += 1;
        }
        prev_measurement = Some(measurement);
    }

    format!("{}", measurements)
}

pub fn solution_b() -> String {
    let file = File::open("src/year2021/day01_input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut prev_measurement: Option<u32> = None;
    let mut measurements = 0;

    let lines = reader.lines().collect::<Vec<_>>();
    for line_num in 0..lines.len() {
        if line_num + 2 > &lines.len() - 1 {
            break;
        }

        let mut measurement = 0;
        for i in 0..3 {
            measurement += &lines[line_num + i]
                .as_ref()
                .unwrap()
                .parse::<u32>()
                .unwrap();
        }

        if prev_measurement.is_none() {
            prev_measurement = Some(measurement);
            continue;
        }

        if prev_measurement.unwrap() < measurement {
            measurements += 1;
        }
        prev_measurement = Some(measurement);
    }
    format!("{}", measurements)
}
