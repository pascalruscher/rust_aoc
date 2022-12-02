use std::{fs::File, io::Read};

fn is_nice_string(line: &str) -> bool {
    let mut vowel_count = 0;
    let mut has_double = false;

    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let illegal_strings = [['a', 'b'], ['c', 'd'], ['p', 'q'], ['x', 'y']];

    let mut prev_char: Option<char> = None;
    for c in line.chars() {
        let mut i = 0;
        if let Some(prev) = prev_char {
            while i < illegal_strings.len() {
                if prev == illegal_strings[i][0] && c == illegal_strings[i][1] {
                    return false;
                }
                i += 1;
            }

            if !has_double && c == prev {
                has_double = true;
            }
        }
        i = 0;
        while vowel_count < 3 && i < vowels.len() {
            if c == vowels[i] {
                vowel_count += 1;
            }
            i += 1;
        }
        prev_char = Some(c);
    }
    has_double && vowel_count == 3
}

fn get_nice_strings(data: String) -> Vec<String> {
    let mut nice_strings: Vec<String> = Vec::new();
    let lines = data.lines();
    for line in lines {
        if is_nice_string(line) {
            nice_strings.push(line.to_string());
        }
    }
    nice_strings
}

fn main() {
    let mut data = String::new();
    let mut f = File::open("2015/day05/input.txt").expect("Error on File::open");
    f.read_to_string(&mut data)
        .expect("Error on read_to_string");

    println!("Nice strings count: {:?}", get_nice_strings(data).len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_nice_string_1() {
        // ugknbfddgicrmopn is nice
        assert!(is_nice_string("ugknbfddgicrmopn"));
    }

    #[test]
    fn is_nice_string_2() {
        // aaa is nice
        assert!(is_nice_string("aaa"));
    }

    #[test]
    fn is_nice_string_3() {
        // jchzalrnumimnmhp is naughty - it has no double letter
        assert!(!is_nice_string("jchzalrnumimnmhp"));
    }

    #[test]
    fn is_nice_string_4() {
        // haegwjzuvuyypxyu is naughty - it contains the string xy
        assert!(!is_nice_string("haegwjzuvuyypxyu"));
    }

    #[test]
    fn is_nice_string_5() {
        // dvszwmarrgswjxmb is naughty - it contains only one vowel
        assert!(!is_nice_string("dvszwmarrgswjxmb"));
    }

    #[test]
    fn is_nice_string_6() {
        // iabjgmbbhilrcyyp is naughty - it contains string ab
        assert!(!is_nice_string("iabjgmbbhilrcyyp"));
    }
}
