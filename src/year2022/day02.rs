use crate::utils::get_input_vec;

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

fn calc_round_a(instruction: String) -> i32 {
    let shapes = instruction.split(" ").collect::<Vec<&str>>();
    let mut score = 0;

    match shapes[1] {
        "X" => {
            score += 1;
            match shapes[0] {
                "A" => score += 3,
                "C" => score += 6,
                _ => score += 0,
            }
        }
        "Y" => {
            score += 2;
            match shapes[0] {
                "A" => score += 6,
                "B" => score += 3,
                _ => score += 0,
            }
        }
        "Z" => {
            score += 3;
            match shapes[0] {
                "B" => score += 6,
                "C" => score += 3,
                _ => score += 0,
            }
        }
        _ => {}
    };

    score
}

fn calc_round_b(instruction: String) -> i32 {
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

fn get_score(data: Vec<Option<String>>, day: &str) -> i32 {
    let mut score = 0;

    for item in data {
        if let Some(instruction) = item {
            match day {
                "a" => {
                    score += calc_round_a(instruction);
                }
                "b" => {
                    score += calc_round_b(instruction);
                }
                _ => {}
            }
        }
    }

    score
}

pub fn solution_a() -> String {
    let data = get_input_vec::<String>("src/year2022/day02_input.txt");
    format!("{}", get_score(data, "a"))
}

pub fn solution_b() -> String {
    let data = get_input_vec::<String>("src/year2022/day02_input.txt");
    format!("{}", get_score(data, "b"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_score_a() {
        let data = vec![
            Some("A Y".to_string()),
            Some("B X".to_string()),
            Some("C Z".to_string()),
        ];
        assert_eq!(get_score(data, "a"), 15);
    }

    #[test]
    fn get_score_b() {
        let data = vec![
            Some("A Y".to_string()),
            Some("B X".to_string()),
            Some("C Z".to_string()),
        ];
        assert_eq!(get_score(data, "b"), 12);
    }
}
