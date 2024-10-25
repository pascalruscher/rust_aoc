use md5::{Digest, Md5};

use crate::utils::get_input_string;

fn find_number(data: String, step: i32) -> usize {
    let mut i = 0;
    loop {
        let mut hasher = Md5::new();
        hasher.update(format!("{}{}", data, i));
        let hash = format!("{:x}", hasher.finalize());

        let mut zero_count = 0;
        for c in hash.chars() {
            if c == '0' {
                zero_count += 1;
                if zero_count == step {
                    return i;
                }
            } else {
                i += 1;
                break;
            }
        }
    }
}

fn solution(step: i32) -> usize {
    let data = get_input_string("src/year2015/day04_input.txt");
    find_number(data, step)
}

pub fn solution_a() -> String {
    format!("{}", solution(5))
}

pub fn solution_b() -> String {
    format!("{}", solution(6))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_number_1() {
        // If your secret key is abcdef, the answer is 609043
        assert_eq!(find_number(String::from("abcdef"), 5), 609043);
    }

    #[test]
    fn find_number_2() {
        // If your secret key is pqrstuv, the lowest number is 1048970
        assert_eq!(find_number(String::from("pqrstuv"), 5), 1048970);
    }
}
