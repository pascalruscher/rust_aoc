use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

enum Shape {
    Rock,
    Paper,
    Scissor,
}

impl Shape {
    fn value(&self) -> i32 {
        match *self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissor => 3,
        }
    }
}

enum RoundResult {
    Loose = 0,
    Draw = 3,
    Win = 6,
}

impl RoundResult {
    fn value(&self) -> i32 {
        match *self {
            RoundResult::Loose => 0,
            RoundResult::Draw => 3,
            RoundResult::Win => 6,
        }
    }
}

fn file_to_vec<T: FromStr>(file: File) -> Vec<Option<T>> {
    let reader = BufReader::new(file);
    let mut data = Vec::new();

    for line in reader.lines() {
        data.push({
            let this = line.unwrap().parse::<T>();
            match this {
                Ok(t) => Some(t),
                Err(_) => None,
            }
        });
    }

    data
}

fn calc_round(instruction: String) -> i32 {
    let shapes = instruction.split(" ").collect::<Vec<&str>>();
    let mut score = 0;

    match shapes[0] {
        "A" => match shapes[1] {
            "X" => {
                score += Shape::Scissor.value() + RoundResult::Loose.value();
            }
            "Y" => {
                score += Shape::Rock.value() + RoundResult::Draw.value();
            }
            "Z" => {
                score += Shape::Paper.value() + RoundResult::Win.value();
            }
            _ => {}
        },
        "B" => match shapes[1] {
            "X" => {
                score += Shape::Rock.value() + RoundResult::Loose.value();
            }
            "Y" => {
                score += Shape::Paper.value() + RoundResult::Draw.value();
            }
            "Z" => {
                score += Shape::Scissor.value() + RoundResult::Win.value();
            }
            _ => {}
        },
        "C" => match shapes[1] {
            "X" => {
                score += Shape::Paper.value() + RoundResult::Loose.value();
            }
            "Y" => {
                score += Shape::Scissor.value() + RoundResult::Draw.value();
            }
            "Z" => {
                score += Shape::Rock.value() + RoundResult::Win.value();
            }
            _ => {}
        },
        _ => {}
    };

    score
}

fn get_score(data: Vec<Option<String>>) -> i32 {
    let mut score = 0;

    for item in data {
        if let Some(instruction) = item {
            score += calc_round(instruction);
        }
    }

    score
}

fn main() {
    let file = File::open("2022/day02/input.txt").expect("Error on File::open");
    let data = file_to_vec::<String>(file);
    println!("{}", get_score(data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_score_1() {
        let data = vec![
            Some("A Y".to_string()),
            Some("B X".to_string()),
            Some("C Z".to_string()),
        ];
        assert_eq!(get_score(data), 12);
    }
}
