use std::{fs::File, io::Read};

fn get_floor(instructions: String) -> i32 {
    // start at floor 0
    let mut floor: i32 = 0;
    for instruction in instructions.chars() {
        match instruction {
            // floor up on (
            '(' => floor += 1,
            // floor down on )
            ')' => floor -= 1,
            _ => {}
        }
    }
    floor
}

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

pub fn solution_a() -> i32 {
    let mut data = String::new();
    let mut file = File::open("src/year2015/day01_input.txt").expect("Error on File::open");
    file.read_to_string(&mut data)
        .expect("Error on read_to_string");
    get_floor(data)
}

pub fn solution_b() -> i32 {
    let mut data = String::new();
    let mut file = File::open("src/year2015/day01_input.txt").expect("Error on File::open");
    file.read_to_string(&mut data)
        .expect("Error on read_to_string");
    get_basement_position(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_floor_1() {
        // "`(())` and `()()` both result in floor `0`"
        assert_eq!(get_floor(String::from("(())")), 0);
        assert_eq!(get_floor(String::from("()()")), 0);
    }

    #[test]
    fn get_floor_2() {
        // "`(((` and `(()(()(` both result in floor `3`"
        assert_eq!(get_floor(String::from("(((")), 3);
        assert_eq!(get_floor(String::from("(()(()(")), 3);
    }

    #[test]
    fn get_floor_3() {
        // "`))(((((` also results in floor `3`"
        assert_eq!(get_floor(String::from("))(((((")), 3);
    }

    #[test]
    fn get_floor_4() {
        // "`())` and `))(` both result in floor `-1`"
        assert_eq!(get_floor(String::from("())")), -1);
        assert_eq!(get_floor(String::from("))(")), -1);
    }

    #[test]
    fn get_floor_5() {
        // "`)))` and `)())())` both result in floor `-3`"
        assert_eq!(get_floor(String::from(")))")), -3);
        assert_eq!(get_floor(String::from(")())())")), -3);
    }

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
