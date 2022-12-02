use std::{collections::HashMap, fs::File, io::Read};

fn init_matrix(size: usize) -> HashMap<String, bool> {
    let mut matrix: HashMap<String, bool> = HashMap::new();
    for x in 0..size {
        for y in 0..size {
            matrix.insert(format!("{},{}", x, y), false);
        }
    }
    matrix
}

fn set_light_matrix(data: String) -> HashMap<String, bool> {
    let mut matrix: HashMap<String, bool> = init_matrix(1000);
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

fn get_light_count(matrix: HashMap<String, bool>) -> usize {
    let mut count: usize = 0;
    for (_, light) in matrix {
        if light {
            count += 1;
        }
    }
    count
}

fn main() {
    let mut data = String::new();
    let mut f = File::open("2015/day06/input.txt").expect("Error on File::open");
    f.read_to_string(&mut data)
        .expect("Error on read_to_string");

    println!(
        "Light matrix: {:?}",
        get_light_count(set_light_matrix(data))
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_light_matrix_1() {
        // turn on 0,0 through 999,999 would turn on (or leave on) every light.
        assert_eq!(
            get_light_count(set_light_matrix(String::from(
                "turn on 0,0 through 999,999"
            ))),
            1000000
        );
    }

    #[test]
    fn set_light_matrix_2() {
        // toggle 0,0 through 999,0 would toggle the first line of 1000 lights
        assert_eq!(
            get_light_count(set_light_matrix(String::from("toggle 0,0 through 999,0"))),
            1000
        );
    }

    #[test]
    fn set_light_matrix_3() {
        // turn off 499,499 through 500,500 would turn off (or leave off) the middle four lights
        assert_eq!(
            get_light_count(set_light_matrix(String::from(
                "turn off 499,499 through 500,500"
            ))),
            0
        );
    }
}
