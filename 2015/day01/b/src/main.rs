use std::{fs::File, io::Read};

fn get_basement_position(instructions: String) -> i32 {
    // start at floor 0
    let mut floor: i32 = 0;
    let mut position: i32 = 0;
    for instruction in instructions.chars() {
        match instruction {
            // floor up on (
            '(' => floor += 1,
            // floor down on )
            ')' => floor -= 1,
            _ => {}
        }
        position += 1;
        if floor == -1 {
            break;
        }
    }
    position
}

fn main() {
    let mut data = String::new();
    let mut f = File::open("2015/day01/input.txt").expect("Error on File::open");
    f.read_to_string(&mut data)
        .expect("Error on read_to_string");

    println!("Basement position: {}", get_basement_position(data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_basement_position_1() {
        // `)` causes him to enter the basement at character position `1`
        assert_eq!(get_basement_position(String::from(")")), 1);
    }

    #[test]
    fn get_basement_position_2() {
        // `()())` causes him to enter the basement at character position `5`
        assert_eq!(get_basement_position(String::from("()())")), 5);
    }
}
