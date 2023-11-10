use std::{
    fs::File,
    io::{BufRead, BufReader}, collections::HashMap,
};

pub fn sort_chars(str: &str) -> String {
    let mut chars: Vec<char> = str.chars().collect();
    chars.sort_by(|a, b| b.cmp(a));
    String::from_iter(chars)
}

pub fn add_number(
    patterns: &mut Vec<&str>,
    numbers: &mut HashMap<u8, String>,
    number: u8,
    pattern: &str,
) {
    numbers.entry(number).or_insert(sort_chars(pattern));
    patterns.retain(|&c| c != pattern);
}

pub fn get_diff_chars(string_1: &str, string_2: &str) -> Vec<char> {
    let mut diff = Vec::new();
    let mut target: Vec<char> = string_2.chars().collect();

    for chr in string_1.chars() {
        if let Some(pos) = target.iter().position(|e2| chr == *e2) {
            target.remove(pos);
        } else {
            diff.push(chr);
        }
    }

    diff.append(&mut target);
    diff
}

pub fn solution_a() -> String {
    let file = File::open("src/year2021/day08_input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut digit_count = 0;
    for line in reader.lines() {
        let input: Vec<&str> = line.as_ref().unwrap().split(" | ").collect();
        let output: Vec<&str> = input[1].split(' ').collect();

        for digit in output {
            match digit.len() {
                2 | 3 | 4 | 7 => digit_count += 1,
                _ => {}
            }
        }
    }
    format!("{}", digit_count)
}

pub fn solution_b() -> String {
    let file = File::open("src/year2021/day08_input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut solution: u32 = 0;
    for line in reader.lines() {
        let parsed: Vec<&str> = line.as_ref().unwrap().split(" | ").collect();
        let mut patterns: Vec<&str> = parsed[0].split(' ').collect();
        let chiffre: Vec<&str> = parsed[1].split(' ').collect();
        let mut numbers: HashMap<u8, String> = HashMap::with_capacity(10);

        while numbers.len() < 10 {
            for pattern in patterns.clone() {
                match pattern.len() {
                    2 => {
                        // 1
                        add_number(&mut patterns, &mut numbers, 1, pattern);
                    }
                    3 => {
                        // 7
                        add_number(&mut patterns, &mut numbers, 7, pattern);
                    }
                    4 => {
                        // 4
                        add_number(&mut patterns, &mut numbers, 4, pattern);
                    }
                    7 => {
                        // 8
                        add_number(&mut patterns, &mut numbers, 8, pattern);
                    }
                    5 => {
                        // 2, 3, 5
                        if numbers.get(&1).is_some()
                            && get_diff_chars(&numbers.get(&1).unwrap(), pattern).len() == 3
                        {
                            add_number(&mut patterns, &mut numbers, 3, pattern);
                        } else if numbers.get(&6).is_some()
                            && get_diff_chars(&numbers.get(&6).unwrap(), pattern).len() == 1
                        {
                            add_number(&mut patterns, &mut numbers, 5, pattern);
                        } else if numbers.get(&9).is_some()
                            && get_diff_chars(&numbers.get(&9).unwrap(), pattern).len() == 3
                        {
                            add_number(&mut patterns, &mut numbers, 2, pattern);
                        }
                    }
                    6 => {
                        // 0, 6, 9
                        if numbers.get(&4).is_some()
                            && get_diff_chars(&numbers.get(&4).unwrap(), pattern).len() == 2
                        {
                            add_number(&mut patterns, &mut numbers, 9, pattern);
                        } else if numbers.get(&1).is_some()
                            && get_diff_chars(&numbers.get(&1).unwrap(), pattern).len() == 6
                        {
                            add_number(&mut patterns, &mut numbers, 6, pattern);
                        } else if numbers.get(&5).is_some()
                            && get_diff_chars(&numbers.get(&5).unwrap(), pattern).len() == 3
                        {
                            add_number(&mut patterns, &mut numbers, 0, pattern);
                        }
                    }
                    _ => {}
                }
            }
        }
        let mut number: String = String::new();
        for chiffre_part in chiffre {
            number.push_str(
                &numbers
                    .iter()
                    .find_map(|(key, val)| {
                        if val.to_owned() == sort_chars(chiffre_part) {
                            Some(key)
                        } else {
                            None
                        }
                    })
                    .unwrap()
                    .to_string(),
            );
        }
        solution += number.parse::<u32>().unwrap();
    }
    format!("{}", solution)
}
