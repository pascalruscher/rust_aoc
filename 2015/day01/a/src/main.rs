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

fn main() {
    let mut data = String::new();
    let mut f = File::open("2015/day01/input.txt").expect("Error on File::open");
    f.read_to_string(&mut data)
        .expect("Error on read_to_string");

    println!("Floor: {}", get_floor(data));
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
}
