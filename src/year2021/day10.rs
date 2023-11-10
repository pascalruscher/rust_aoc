use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn solution_a() -> String {
    let file = File::open("src/year2021/day10_input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut illegal_score = 0;
    for line in reader.lines() {
        let characters: Vec<char> = line.as_ref().unwrap().chars().collect();
        let mut opened: Vec<char> = Vec::new();
        for character in characters {
            if "[{<(".contains(character) {
                opened.push(character);
            } else if "]}>)".contains(character) {
                let opened_last = opened.pop().unwrap();
                match character {
                    ')' => {
                        if opened_last != '(' {
                            illegal_score += 3;
                            break;
                        }
                    }
                    '}' => {
                        if opened_last != '{' {
                            illegal_score += 1197;
                            break;
                        }
                    }
                    ']' => {
                        if opened_last != '[' {
                            illegal_score += 57;
                            break;
                        }
                    }
                    '>' => {
                        if opened_last != '<' {
                            illegal_score += 25137;
                            break;
                        }
                    }
                    _ => {}
                };
            }
        }
    }
    format!("{}", illegal_score)
}

pub fn solution_b() -> String {
    let file = File::open("src/year2021/day08_input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut completion_scores: Vec<u64> = Vec::new();
    for line in reader.lines() {
        let characters: Vec<char> = line.as_ref().unwrap().chars().collect();
        let mut opened: Vec<char> = Vec::new();
        let mut is_valid = true;
        for character in characters {
            if "[{<(".contains(character) {
                opened.push(character);
            } else if "]}>)".contains(character) {
                let opened_last = opened.pop();
                if opened_last.is_none() {
                    break;
                } else {
                    match character {
                        ')' => {
                            if opened_last.unwrap() != '(' {
                                is_valid = false;
                                break;
                            }
                        }
                        '}' => {
                            if opened_last.unwrap() != '{' {
                                is_valid = false;
                                break;
                            }
                        }
                        ']' => {
                            if opened_last.unwrap() != '[' {
                                is_valid = false;
                                break;
                            }
                        }
                        '>' => {
                            if opened_last.unwrap() != '<' {
                                is_valid = false;
                                break;
                            }
                        }
                        _ => {}
                    };
                }
            }
        }

        if is_valid {
            let mut score: u64 = 0;
            while opened.len() > 0 {
                let opened_last = opened.pop().unwrap();
                score = score * 5;
                match opened_last {
                    '(' => {
                        score += 1;
                    }
                    '{' => {
                        score += 3;
                    }
                    '[' => {
                        score += 2;
                    }
                    '<' => {
                        score += 4;
                    }
                    _ => {}
                };
            }
            completion_scores.push(score);
        }
    }
    completion_scores.sort();
    format!("{}", completion_scores[completion_scores.len() / 2])
}
