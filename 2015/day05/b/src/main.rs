use std::{collections::HashMap, fs::File, io::Read};

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

fn is_nice_string(line: &str) -> bool {
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
        // qjhvhtzxzqqjkmpb is nice
        assert!(is_nice_string("qjhvhtzxzqqjkmpb"));
    }

    #[test]
    fn is_nice_string_2() {
        // xxyxx is nice
        assert!(is_nice_string("xxyxx"));
    }

    #[test]
    fn is_nice_string_3() {
        // uurcxstgmygtbstg is naughty - it has no repeated letter
        assert!(!is_nice_string("uurcxstgmygtbstg"));
    }

    #[test]
    fn is_nice_string_4() {
        // ieodomkazucvgmuy is naughty - it has no double pair
        assert!(!is_nice_string("ieodomkazucvgmuy"));
    }

    #[test]
    fn is_nice_string_5() {
        // qpnxkuldeiituggg is naughty - it contains the string xy
        assert!(!is_nice_string("qpnxkuldeiituggg"));
    }
}
