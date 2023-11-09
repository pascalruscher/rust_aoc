use std::{fs::File, io::Read};

fn calculate_square_feet(length: i32, width: i32, height: i32) -> i32 {
    let sides: [i32; 3] = [length * width, width * height, height * length];
    let mut smallest: i32 = i32::MAX;
    for side in sides {
        if side < smallest {
            smallest = side;
        }
    }
    2 * sides[0] + 2 * sides[1] + 2 * sides[2] + smallest
}

fn calculate_feet(length: i32, width: i32, height: i32) -> i32 {
    let mut sides: [i32; 3] = [length, width, height];
    sides.sort_unstable();
    sides[0] * 2 + sides[1] * 2 + sides[0] * sides[1] * sides[2]
}

fn get_total_square_feet(data: String) -> i32 {
    let mut square_feet: i32 = 0;
    let lines = data.lines();
    for line in lines {
        let numbers: Vec<i32> = line
            .split('x')
            .map(|string_num| string_num.parse::<i32>().unwrap())
            .collect();
        square_feet += calculate_square_feet(numbers[0], numbers[1], numbers[2]);
    }
    square_feet
}

fn get_total_feet(data: String) -> i32 {
    let mut feet: i32 = 0;
    let lines = data.lines();
    for line in lines {
        let numbers: Vec<i32> = line
            .split('x')
            .map(|string_num| string_num.parse::<i32>().unwrap())
            .collect();
        feet += calculate_feet(numbers[0], numbers[1], numbers[2]);
    }
    feet
}

pub fn solution_a() -> i32 {
    let mut data = String::new();
    let mut file = File::open("src/year2015/day02_input.txt").expect("Error on File::open");
    file.read_to_string(&mut data)
        .expect("Error on read_to_string");
    get_total_square_feet(data)
}

pub fn solution_b() -> i32 {
    let mut data = String::new();
    let mut file = File::open("src/year2015/day02_input.txt").expect("Error on File::open");
    file.read_to_string(&mut data)
        .expect("Error on read_to_string");
    get_total_feet(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_square_feet_1() {
        // A present with dimensions `2x3x4` requires a total of `58` square feet
        assert_eq!(calculate_square_feet(2, 3, 4), 58);
    }

    #[test]
    fn calculate_square_feet_2() {
        // A present with dimensions `1x1x10` requires a total of `43` square feet
        assert_eq!(calculate_square_feet(1, 1, 10), 43);
    }
    
    #[test]
    fn calculate_feet_1() {
        // A present with dimensions `2x3x4` requires a total of `34` feet
        assert_eq!(calculate_feet(2, 3, 4), 34);
    }

    #[test]
    fn calculate_feet_2() {
        // A present with dimensions `1x1x10` requires a total of `14` feet
        assert_eq!(calculate_feet(1, 1, 10), 14);
    }
}
