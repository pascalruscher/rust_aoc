use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solution_a() -> String {
    let file = File::open("src/year2021/day14_input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|s| s.unwrap()).collect::<Vec<_>>();

    let mut polymer = lines[0].clone();
    let mut instructions: HashMap<String, String> = HashMap::new();
    for line in lines[2..].iter() {
        let splitted = line.split_once(" -> ").unwrap();
        instructions
            .entry(splitted.0.to_string())
            .or_insert(splitted.1.to_string());
    }

    for _ in 0..10 {
        let mut new_polymer = String::new();
        for i in 0..polymer.len() {
            new_polymer.push_str(&polymer[i..i + 1]);
            if i + 1 < polymer.len() && instructions.contains_key(&polymer[i..i + 2].to_string()) {
                new_polymer.push_str(instructions.get(&polymer[i..i + 2].to_string()).unwrap());
            }
        }
        polymer = new_polymer;
    }

    let mut letters: HashMap<char, u32> = HashMap::new();
    for letter in polymer.chars().collect::<Vec<char>>() {
        *letters.entry(letter).or_insert(0) += 1;
    }

    let mut most_common_count: u32 = 0;
    let mut least_common_count: u32 = u32::MAX;
    for (_, letter_count) in letters {
        if letter_count > most_common_count {
            most_common_count = letter_count;
        }
        if letter_count < least_common_count {
            least_common_count = letter_count;
        }
    }
    format!("{}", most_common_count - least_common_count)
}

pub fn solution_b() -> String {
    let file = File::open("src/year2021/day14_input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines().map(|s| s.unwrap()).collect::<Vec<_>>();

    let mut instructions: HashMap<String, String> = HashMap::new();
    for line in lines[2..].iter() {
        let splitted = line.split_once(" -> ").unwrap();
        instructions
            .entry(splitted.0.to_string())
            .or_insert(splitted.1.to_string());
    }

    let letters = lines[0].chars().collect::<Vec<char>>();

    let mut polymer: HashMap<String, u64> = HashMap::new();
    for i in 0..letters.len() - 1 {
        *polymer
            .entry(format!("{}{}", letters[i], letters[i + 1]))
            .or_insert(0) += 1;
    }

    for _ in 0..40 {
        let mut new_polymer: HashMap<String, u64> = HashMap::new();
        for (pair, count) in polymer {
            let insert_letter = instructions.get(&pair).unwrap();
            *new_polymer
                .entry(format!(
                    "{}{}",
                    pair[0..1].to_string(),
                    insert_letter.to_string()
                ))
                .or_insert(0) += count;
            *new_polymer
                .entry(format!(
                    "{}{}",
                    insert_letter.to_string(),
                    pair[1..2].to_string()
                ))
                .or_insert(0) += count;
        }
        polymer = new_polymer;
    }

    // We count always the first letter in the pairs to avoid counting twice
    let mut letters_count: HashMap<String, u64> = HashMap::new();
    for (pair, count) in polymer {
        *letters_count.entry(pair[0..1].to_string()).or_insert(0) += count;
    }
    // But we need to count the last letter also (which always stays the same)
    *letters_count
        .entry(letters[letters.len() - 1].to_string())
        .or_insert(0) += 1;

    let mut most_common_count: u64 = 0;
    let mut least_common_count: u64 = u64::MAX;
    for (_, letter_count) in letters_count {
        if letter_count > most_common_count {
            most_common_count = letter_count;
        }
        if letter_count < least_common_count {
            least_common_count = letter_count;
        }
    }
    format!("{}", most_common_count - least_common_count)
}
