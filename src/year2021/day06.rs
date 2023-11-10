use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solution_a() -> String {
    let file = File::open("src/year2021/day05_input.txt").unwrap();
    let mut reader = BufReader::new(file);

    let mut input = String::new();
    reader.read_line(&mut input).unwrap();

    let mut fishes: Vec<u8> = input
        .trim()
        .split(',')
        .map(|c| c.parse::<u8>().unwrap())
        .collect();

    for _ in 0..80 {
        let mut new_fishes: Vec<u8> = Vec::new();
        for fish in &mut fishes {
            if *fish > 0 {
                *fish -= 1;
            } else {
                *fish = 6;
                new_fishes.push(8);
            }
        }
        fishes.extend(new_fishes);
    }
    format!("{}", fishes.len())
}

pub fn solution_b() -> String {
    let file = File::open("src/year2021/day06_input.txt").unwrap();
    let mut reader = BufReader::new(file);

    let mut input = String::new();
    reader.read_line(&mut input).unwrap();

    let init_fishes: Vec<u8> = input
        .trim()
        .split(',')
        .map(|c| c.parse::<u8>().unwrap())
        .collect();

    let mut fishes: Vec<u64> = vec![0; 9];
    for fish in init_fishes {
        fishes[fish as usize] += 1;
    }

    for _ in 0..256 {
        let mut new_fishes: Vec<u64> = vec![0; 9];
        for (i, fish_count) in fishes.iter().enumerate() {
            if i == 0 {
                new_fishes[8] += fish_count;
                new_fishes[6] += fish_count;
            } else {
                new_fishes[i - 1] += fish_count;
            }
        }
        fishes = new_fishes;
    }

    let mut total_fishes = 0;
    for fish_count in fishes {
        total_fishes += fish_count;
    }
    format!("{}", total_fishes)
}
