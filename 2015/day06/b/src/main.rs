use std::{collections::HashMap, fs::File, io::Read};

fn init_matrix(size: i32) -> HashMap<String, i32> {
    let mut matrix: HashMap<String, i32> = HashMap::new();
    for x in 0..size {
        for y in 0..size {
            matrix.insert(format!("{},{}", x, y), 0);
        }
    }
    matrix
}

fn set_light_matrix(data: String) -> HashMap<String, i32> {
    let mut matrix: HashMap<String, i32> = init_matrix(1000);
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

fn get_brightness(matrix: HashMap<String, i32>) -> i32 {
    let mut total_brightness: i32 = 0;
    for (_, brightness) in matrix {
        total_brightness += brightness;
    }
    total_brightness
}

fn main() {
    let mut data = String::new();
    let mut f = File::open("2015/day06/input.txt").expect("Error on File::open");
    f.read_to_string(&mut data)
        .expect("Error on read_to_string");

    println!(
        "Total brightness {:?}",
        get_brightness(set_light_matrix(data))
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_brightness_1() {
        // turn on 0,0 through 0,0 would increase the total brightness by 1
        assert_eq!(
            get_brightness(set_light_matrix(String::from("turn on 0,0 through 0,0"))),
            1
        );
    }

    #[test]
    fn get_brightness_2() {
        // toggle 0,0 through 999,999 would increase the total brightness by 2000000
        assert_eq!(
            get_brightness(set_light_matrix(String::from("toggle 0,0 through 999,999"))),
            2000000
        );
    }
}
