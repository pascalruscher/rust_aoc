use std::{fs::File, io::Read};

fn calculate_feet(length: i32, width: i32, height: i32) -> i32 {
    let mut sides: [i32; 3] = [length, width, height];
    sides.sort_unstable();
    sides[0] * 2 + sides[1] * 2 + sides[0] * sides[1] * sides[2]
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

fn main() {
    let mut data = String::new();
    let mut f = File::open("2015/day02/input.txt").expect("Error on File::open");
    f.read_to_string(&mut data)
        .expect("Error on read_to_string");

    println!("Square feet: {}", get_total_feet(data));
}

#[cfg(test)]
mod tests {
    use super::*;

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
