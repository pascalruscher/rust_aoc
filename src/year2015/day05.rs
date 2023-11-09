use std::{fs::File, io::Read, collections::HashMap};

fn is_nice_string_a(line: &str) -> bool {
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

fn is_nice_string_b(line: &str) -> bool {
    let mut pairlist_1: Vec<String> = Vec::new();
    let mut pairlist_2: Vec<String> = Vec::new();
    let mut has_repeating = false;

    for (j, c) in line.chars().enumerate() {
        // Collect pairs
        if let Some(next_c) = line.chars().nth(j + 1) {
            let pair = format!("{}{}", c, next_c);
            if j % 2 == 0 {
                pairlist_1.push(pair);
            } else {
                pairlist_2.push(pair);
            }
        }

        // Check repeating after in between letter
        if !has_repeating {
            if let Some(cmp_c) = line.chars().nth(j + 2) {
                if cmp_c == c {
                    has_repeating = true;
                }
            }
        }
    }

    // TODO: The Double Pair check definitely needs improvement
    let mut has_double = has_double_pair(&pairlist_1);

    if !has_double {
        has_double = has_double_pair(&pairlist_2);
    }

    if !has_double {
        'outer: for (i, pair1) in pairlist_1.into_iter().enumerate() {
            for (j, pair2) in pairlist_2.clone().into_iter().enumerate() {
                // skip overlapping chars
                if i == j || (i > 0 && i - 1 == j) {
                    continue;
                }
                if pair1 == pair2 {
                    has_double = true;
                    break 'outer;
                }
            }
        }
    }

    has_double && has_repeating
}

fn has_double_pair(pairlist: &[String]) -> bool {
    let mut pairlist_map: HashMap<&str, usize> = HashMap::new();
    for pair in pairlist.iter() {
        *pairlist_map.entry(pair).or_insert(0) += 1;
        if pairlist_map[&pair.as_str()] > 1 {
            return true;
        }
    }
    false
}

fn get_nice_strings(data: String, day: &str) -> Vec<String> {
    let mut nice_strings: Vec<String> = Vec::new();
    let lines = data.lines();
    for line in lines {
        match day {
            "a" => {
                if is_nice_string_a(line) {
                    nice_strings.push(line.to_string());
                }
            },
            "b" => {
                if is_nice_string_b(line) {
                    nice_strings.push(line.to_string());
                }
            },
            _ => {}
        }
    }
    nice_strings
}

pub fn solution_a() -> usize {
    let mut data = String::new();
    let mut file = File::open("src/year2015/day05_input.txt").expect("Error on File::open");
    file.read_to_string(&mut data)
        .expect("Error on read_to_string");
    get_nice_strings(data, "a").len()
}

pub fn solution_b() -> usize {
    let mut data = String::new();
    let mut file = File::open("src/year2015/day05_input.txt").expect("Error on File::open");
    file.read_to_string(&mut data)
        .expect("Error on read_to_string");
    get_nice_strings(data, "b").len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_nice_string_a_1() {
        // ugknbfddgicrmopn is nice
        assert!(is_nice_string_a("ugknbfddgicrmopn"));
    }

    #[test]
    fn is_nice_string_a_2() {
        // aaa is nice
        assert!(is_nice_string_a("aaa"));
    }

    #[test]
    fn is_nice_string_a_3() {
        // jchzalrnumimnmhp is naughty - it has no double letter
        assert!(!is_nice_string_a("jchzalrnumimnmhp"));
    }

    #[test]
    fn is_nice_string_a_4() {
        // haegwjzuvuyypxyu is naughty - it contains the string xy
        assert!(!is_nice_string_a("haegwjzuvuyypxyu"));
    }

    #[test]
    fn is_nice_string_a_5() {
        // dvszwmarrgswjxmb is naughty - it contains only one vowel
        assert!(!is_nice_string_a("dvszwmarrgswjxmb"));
    }

    #[test]
    fn is_nice_string_a_6() {
        // iabjgmbbhilrcyyp is naughty - it contains string ab
        assert!(!is_nice_string_a("iabjgmbbhilrcyyp"));
    }
    #[test]
    fn is_nice_string_b_1() {
        // qjhvhtzxzqqjkmpb is nice
        assert!(is_nice_string_b("qjhvhtzxzqqjkmpb"));
    }

    #[test]
    fn is_nice_string_b_2() {
        // xxyxx is nice
        assert!(is_nice_string_b("xxyxx"));
    }

    #[test]
    fn is_nice_string_b_3() {
        // uurcxstgmygtbstg is naughty - it has no repeated letter
        assert!(!is_nice_string_b("uurcxstgmygtbstg"));
    }

    #[test]
    fn is_nice_string_b_4() {
        // ieodomkazucvgmuy is naughty - it has no double pair
        assert!(!is_nice_string_b("ieodomkazucvgmuy"));
    }

    #[test]
    fn is_nice_string_b_5() {
        // qpnxkuldeiituggg is naughty - it contains the string xy
        assert!(!is_nice_string_b("qpnxkuldeiituggg"));
    }
}
