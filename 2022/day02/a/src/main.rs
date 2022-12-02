use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

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
        assert_eq!(get_score(data), 15);
    }
}
