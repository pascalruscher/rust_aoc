use regex::Regex;
use std::{fs::File, io::Read};

fn get_char_count(data: String) -> usize {
    let mut char_count: usize = 0;
    let hx_re = Regex::new(r"\\x[0-9A-Fa-f]{2}").unwrap();
    let lines = data.lines();
    for line in lines {
        let line_tmp = &line.replace("\\\\", "_").replace("\\\"", "_");
        let line_tmp_2 = hx_re.replace_all(line_tmp, "_");
        // -2 because " at start and end
        char_count += line.chars().count() - (line_tmp_2.chars().count() - 2);
    }
    char_count
}

fn main() {
    let mut data = String::new();
    let mut f = File::open("2015/day08/input.txt").expect("Error on File::open");
    f.read_to_string(&mut data)
        .expect("Error on read_to_string");

    println!("Solution: {:?}", get_char_count(data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_char_count_1() {
        assert_eq!(get_char_count("\"\"".to_string()), 2);
    }

    #[test]
    fn get_char_count_2() {
        assert_eq!(get_char_count("\"abc\"".to_string()), 2);
    }

    #[test]
    fn get_char_count_3() {
        assert_eq!(get_char_count("\"aaa\\\"aaa\"".to_string()), 3);
    }

    #[test]
    fn get_char_count_4() {
        assert_eq!(get_char_count("\"\\x27\"".to_string()), 5);
    }
}
