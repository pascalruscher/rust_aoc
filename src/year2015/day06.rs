use std::collections::HashMap;

use crate::utils::get_input_string;

fn init_matrix_a(size: usize) -> HashMap<String, bool> {
    let mut matrix: HashMap<String, bool> = HashMap::new();
    for x in 0..size {
        for y in 0..size {
            matrix.insert(format!("{},{}", x, y), false);
        }
    }
    matrix
}

fn init_matrix_b(size: i32) -> HashMap<String, i32> {
    let mut matrix: HashMap<String, i32> = HashMap::new();
    for x in 0..size {
        for y in 0..size {
            matrix.insert(format!("{},{}", x, y), 0);
        }
    }
    matrix
}

fn set_light_matrix_a(data: String) -> HashMap<String, bool> {
    let mut matrix: HashMap<String, bool> = init_matrix_a(1000);
    let lines = data.lines();
    for line in lines {
        let instruction: Vec<&str> = line.split(' ').collect();
        let from: Vec<i32> = instruction[instruction.len() - 3]
            .split(',')
            .map(|string_num| string_num.parse::<i32>().unwrap())
            .collect();
        let to: Vec<i32> = instruction[instruction.len() - 1]
            .split(',')
            .map(|string_num| string_num.parse::<i32>().unwrap())
            .collect();
        for x in from[0]..=to[0] {
            for y in from[1]..=to[1] {
                let coord = format!("{},{}", x, y);
                match instruction[instruction.len() - 4] {
                    "on" => {
                        *matrix.entry(coord).or_insert(true) = true;
                    }
                    "off" => {
                        *matrix.entry(coord).or_insert(false) = false;
                    }
                    "toggle" => match matrix.get(&coord) {
                        Some(true) => {
                            *matrix.entry(coord).or_insert(false) = false;
                        }
                        Some(false) => {
                            *matrix.entry(coord).or_insert(true) = true;
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
    }
    matrix
}

fn set_light_matrix_b(data: String) -> HashMap<String, i32> {
    let mut matrix: HashMap<String, i32> = init_matrix_b(1000);
    let lines = data.lines();
    for line in lines {
        let instruction: Vec<&str> = line.split(' ').collect();
        let from: Vec<i32> = instruction[instruction.len() - 3]
            .split(',')
            .map(|string_num| string_num.parse::<i32>().unwrap())
            .collect();
        let to: Vec<i32> = instruction[instruction.len() - 1]
            .split(',')
            .map(|string_num| string_num.parse::<i32>().unwrap())
            .collect();
        for x in from[0]..=to[0] {
            for y in from[1]..=to[1] {
                let coord = format!("{},{}", x, y);
                match instruction[instruction.len() - 4] {
                    "on" => {
                        *matrix.entry(coord).or_insert(0) += 1;
                    }
                    "off" => {
                        let matrix_value = matrix.get(&coord);
                        if matrix_value.is_some() && matrix_value.unwrap() > &0 {
                            *matrix.entry(coord).or_insert(1) -= 1;
                        }
                    }
                    "toggle" => {
                        *matrix.entry(coord).or_insert(0) += 2;
                    }
                    _ => {}
                }
            }
        }
    }
    matrix
}

fn get_light_count(matrix: HashMap<String, bool>) -> usize {
    let mut count: usize = 0;
    for (_, light) in matrix {
        if light {
            count += 1;
        }
    }
    count
}

fn get_brightness(matrix: HashMap<String, i32>) -> i32 {
    let mut total_brightness: i32 = 0;
    for (_, brightness) in matrix {
        total_brightness += brightness;
    }
    total_brightness
}

pub fn solution_a() -> String {
    let data = get_input_string("src/year2015/day06_input.txt");
    format!("{}", get_light_count(set_light_matrix_a(data)))
}

pub fn solution_b() -> String {
    let data = get_input_string("src/year2015/day06_input.txt");
    format!("{}", get_brightness(set_light_matrix_b(data)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_light_matrix_1() {
        // turn on 0,0 through 999,999 would turn on (or leave on) every light.
        assert_eq!(
            get_light_count(set_light_matrix_a(String::from(
                "turn on 0,0 through 999,999"
            ))),
            1000000
        );
    }

    #[test]
    fn set_light_matrix_2() {
        // toggle 0,0 through 999,0 would toggle the first line of 1000 lights
        assert_eq!(
            get_light_count(set_light_matrix_a(String::from("toggle 0,0 through 999,0"))),
            1000
        );
    }

    #[test]
    fn set_light_matrix_3() {
        // turn off 499,499 through 500,500 would turn off (or leave off) the middle four lights
        assert_eq!(
            get_light_count(set_light_matrix_a(String::from(
                "turn off 499,499 through 500,500"
            ))),
            0
        );
    }

    #[test]
    fn get_brightness_1() {
        // turn on 0,0 through 0,0 would increase the total brightness by 1
        assert_eq!(
            get_brightness(set_light_matrix_b(String::from("turn on 0,0 through 0,0"))),
            1
        );
    }

    #[test]
    fn get_brightness_2() {
        // toggle 0,0 through 999,999 would increase the total brightness by 2000000
        assert_eq!(
            get_brightness(set_light_matrix_b(String::from(
                "toggle 0,0 through 999,999"
            ))),
            2000000
        );
    }
}
