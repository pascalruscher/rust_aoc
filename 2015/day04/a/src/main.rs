use md5::{Digest, Md5};
use std::{fs::File, io::Read};

fn find_number(data: String) -> usize {
    let mut i = 0;
    loop {
        let mut hasher = Md5::new();
        hasher.update(format!("{}{}", data, i));
        let hash = format!("{:x}", hasher.finalize());

        let mut zero_count = 0;
        for c in hash.chars() {
            if c == '0' {
                zero_count += 1;
                if zero_count == 5 {
                    return i;
                }
            } else {
                i += 1;
                break;
            }
        }
    }
}

fn main() {
    let mut data = String::new();
    let mut f = File::open("2015/day04/input.txt").expect("Error on File::open");
    f.read_to_string(&mut data)
        .expect("Error on read_to_string");

    println!("Hash: {:?}", find_number(data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_number_1() {
        // If your secret key is abcdef, the answer is 609043
        assert_eq!(find_number(String::from("abcdef")), 609043);
    }

    #[test]
    fn find_number_2() {
        // If your secret key is pqrstuv, the lowest number is 1048970
        assert_eq!(find_number(String::from("pqrstuv")), 1048970);
    }
}
