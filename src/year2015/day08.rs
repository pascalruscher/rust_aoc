use regex::Regex;

use crate::utils::get_input_string;

fn get_char_count_a(data: String) -> usize {
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

fn get_char_count_b(data: String) -> usize {
    let mut char_count: usize = 0;
    let lines = data.lines();
    for line in lines {
        char_count += line.escape_default().count() + 2 - line.len();
    }
    char_count
}

pub fn solution_a() -> String {
    let data = get_input_string("src/year2015/day08_input.txt");
    format!("{}", get_char_count_a(data))
}

pub fn solution_b() -> String {
    let data = get_input_string("src/year2015/day08_input.txt");
    format!("{}", get_char_count_b(data))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_char_count_1() {
        assert_eq!(get_char_count_a("\"\"".to_string()), 2);
    }

    #[test]
    fn get_char_count_2() {
        assert_eq!(get_char_count_a("\"abc\"".to_string()), 2);
    }

    #[test]
    fn get_char_count_3() {
        assert_eq!(get_char_count_a("\"aaa\\\"aaa\"".to_string()), 3);
    }

    #[test]
    fn get_char_count_4() {
        assert_eq!(get_char_count_a("\"\\x27\"".to_string()), 5);
    }
}
