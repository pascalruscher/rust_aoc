use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn reduce_to_one(higher_count: bool, bytes: &Vec<String>) -> String {
    let mut bytes_left = bytes.to_owned();
    let mut bit_pos: usize = 0;
    let bits_len = bytes_left[0].len();

    while bytes_left.len() > 1 && bit_pos < bits_len {
        let mut ones: Vec<String> = Vec::new();
        let mut zeroes: Vec<String> = Vec::new();

        for byte in bytes_left {
            if &byte[bit_pos..1 + bit_pos] == "1" {
                ones.push(byte.to_string());
            } else {
                zeroes.push(byte.to_string());
            }
        }

        if higher_count == true {
            if ones.len() >= zeroes.len() {
                bytes_left = ones;
            } else {
                bytes_left = zeroes;
            }
        } else {
            if zeroes.len() <= ones.len() {
                bytes_left = zeroes;
            } else {
                bytes_left = ones;
            }
        }
        bit_pos += 1;
    }
    return bytes_left[0].to_string();
}

pub fn add_bits_count(mut bits: Vec<Vec<i32>>, line: String) -> Vec<Vec<i32>> {
    for (pos, bit_char) in line.trim_end().chars().enumerate() {
        let bit = bit_char.to_digit(10).unwrap() as usize;
        bits[pos][bit] += 1;
    }
    return bits;
}

pub fn solution_a() -> String {
    let file = File::open("src/year2021/day03_input.txt").unwrap();
    let mut reader = BufReader::new(file);

    let mut first_line = String::new();
    reader.read_line(&mut first_line).unwrap();

    let mut bits = vec![vec![0; 2]; first_line.trim_end().len()];
    bits = add_bits_count(bits, first_line);

    for line in reader.lines() {
        bits = add_bits_count(bits, line.unwrap());
    }

    let mut gamma = String::new();
    let mut epsilon = String::new();

    for bit_count in &bits {
        if bit_count[0] > bit_count[1] {
            gamma.push('0');
            epsilon.push('1');
        } else {
            gamma.push('1');
            epsilon.push('0');
        }
    }
    format!("{}", i32::from_str_radix(&gamma, 2).unwrap() * i32::from_str_radix(&epsilon, 2).unwrap())
}

pub fn solution_b() -> String {
    let file = File::open("src/year2021/day03_input.txt").unwrap();
    let reader = BufReader::new(file);
    let bytes = reader.lines().map(|s| s.unwrap()).collect::<Vec<_>>();

    let oxygen = reduce_to_one(true, &bytes);
    let scrubber = reduce_to_one(false, &bytes);

    format!("{}", i32::from_str_radix(&oxygen, 2).unwrap() * i32::from_str_radix(&scrubber, 2).unwrap())
}
