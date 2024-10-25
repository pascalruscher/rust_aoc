use crate::utils::get_input_string;

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

pub fn solution_a() -> String {
    let data = get_input_string("src/year2022/day06_input.txt");
    format!("{}", get_marker_position(data, 4))
}

pub fn solution_b() -> String {
    let data = get_input_string("src/year2022/day06_input.txt");
    format!("{}", get_marker_position(data, 14))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_marker_position_1() {
        let data = "bvwbjplbgvbhsrlpgdmjqwftvncz".to_string();
        assert_eq!(get_marker_position(data, 4), 5);
    }

    #[test]
    fn get_marker_position_2() {
        let data = "nppdvjthqldpwncqszvftbrmjlhg".to_string();
        assert_eq!(get_marker_position(data, 4), 6);
    }

    #[test]
    fn get_marker_position_3() {
        let data = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string();
        assert_eq!(get_marker_position(data, 4), 10);
    }

    #[test]
    fn get_marker_position_4() {
        let data = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string();
        assert_eq!(get_marker_position(data, 4), 11);
    }

    #[test]
    fn get_marker_position_5() {
        let data = "mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string();
        assert_eq!(get_marker_position(data, 14), 19);
    }
}
