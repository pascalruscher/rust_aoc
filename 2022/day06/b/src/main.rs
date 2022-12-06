use std::{fs::File, io::Read};

fn is_unique_sequence(sequence: Vec<char>) -> bool {
    let mut sequence_check = sequence.clone();

    sequence_check.sort();
    sequence_check.dedup();

    sequence.len() == sequence_check.len()
}

fn get_marker_position(data: String, sequence_length: usize) -> usize {
    let mut sequence: Vec<char> = Vec::new();

    for (i, c) in data.chars().enumerate() {
        // Sequence not long enough for further processing
        if sequence.len() < sequence_length {
            sequence.push(c);
            continue;
        }

        if is_unique_sequence(sequence.clone()) {
            return i;
        }

        sequence = sequence[1..].to_vec();
        sequence.push(c);
    }

    0
}

fn main() {
    let mut data = String::new();
    let mut f = File::open("2022/day06/input.txt").expect("Error on File::open");
    f.read_to_string(&mut data)
        .expect("Error on read_to_string");

    println!("{}", get_marker_position(data, 14));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_marker_position_1() {
        let data = "bvwbjplbgvbhsrlpgdmjqwftvncz".to_string();
        assert_eq!(get_marker_position(data, 14), 23);
    }

    #[test]
    fn get_marker_position_2() {
        let data = "nppdvjthqldpwncqszvftbrmjlhg".to_string();
        assert_eq!(get_marker_position(data, 14), 23);
    }

    #[test]
    fn get_marker_position_3() {
        let data = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string();
        assert_eq!(get_marker_position(data, 14), 29);
    }

    #[test]
    fn get_marker_position_4() {
        let data = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string();
        assert_eq!(get_marker_position(data, 14), 26);
    }

    #[test]
    fn get_marker_position_5() {
        let data = "mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string();
        assert_eq!(get_marker_position(data, 14), 19);
    }
}
