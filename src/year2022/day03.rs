use std::{fs::File, io::{BufReader, BufRead}, str::FromStr};

static LETTERS: [char; 52] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

fn file_to_vec<T: FromStr>(file: File) -> Vec<Option<T>> {
    let reader = BufReader::new(file);
    let mut data = Vec::new();

    for line in reader.lines() {
        data.push({
            let this = line.unwrap().parse::<T>();
            match this {
                Ok(t) => Some(t),
                Err(_) => None,
            }
        });
    }

    data
}

fn get_sum_a(data: Vec<Option<String>>) -> usize {
    let mut sum = 0;

    'outer: for line in data {
        let rucksack = line.unwrap();
        let items_1 = &rucksack[..rucksack.len()/2];
        let items_2 = &rucksack[rucksack.len()/2..];
        for item_1 in items_1.chars() {
            for item_2 in items_2.chars() {
                if item_1 == item_2 {
                    sum += LETTERS.iter().position(|&letter| letter == item_1).unwrap() + 1;
                    continue 'outer;
                }
            }
        }
    }

    sum
}


fn get_sum_b(data: Vec<Option<String>>) -> usize {
    let mut sum = 0;
    let mut rucksack_set = Vec::new();

    'outer: for line in data {
        let rucksack = line.unwrap();
        rucksack_set.push(rucksack);

        if rucksack_set.len() == 3 {
            for item_1 in rucksack_set[0].chars() {
                for item_2 in rucksack_set[1].chars() {
                    for item_3 in rucksack_set[2].chars() {
                        if item_1 == item_2 && item_1 == item_3 {
                            sum += LETTERS.iter().position(|&letter| letter == item_1).unwrap() + 1;
                            rucksack_set = Vec::new();
                            continue 'outer;
                        }
                    }
                }
            }
        }
    }

    sum
}

pub fn solution_a() -> String {
    let file = File::open("src/year2022/day03_input.txt").expect("Error on File::open");
    let data = file_to_vec::<String>(file);
    format!("{}", get_sum_a(data))
}

pub fn solution_b() -> String {
    let file = File::open("src/year2022/day03_input.txt").expect("Error on File::open");
    let data = file_to_vec::<String>(file);
    format!("{}", get_sum_b(data))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_sum_a_1() {
        let data = vec![
            Some("vJrwpWtwJgWrhcsFMMfFFhFp".to_string()),
            Some("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string()),
            Some("PmmdzqPrVvPwwTWBwg".to_string()),
            Some("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_string()),
            Some("ttgJtRGJQctTZtZT".to_string()),
            Some("CrZsJsPPZsGzwwsLwLmpwMDw".to_string()),
        ];
        assert_eq!(get_sum_a(data), 157);
    }

    #[test]
    fn get_sum_b_1() {
        let data = vec![
            Some("vJrwpWtwJgWrhcsFMMfFFhFp".to_string()),
            Some("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string()),
            Some("PmmdzqPrVvPwwTWBwg".to_string()),
            Some("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_string()),
            Some("ttgJtRGJQctTZtZT".to_string()),
            Some("CrZsJsPPZsGzwwsLwLmpwMDw".to_string()),
        ];
        assert_eq!(get_sum_b(data), 70);
    }
}
