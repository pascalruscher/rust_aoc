use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

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

fn get_contained_pairs(data: Vec<Option<String>>, day: &str) -> Vec<((i32, i32), (i32, i32))> {
    let mut contained_pairs = Vec::new();

    for item in data {
        if let Some(sections) = item {
            let pair = sections
                .split_once(',')
                .map(|s| -> ((i32, i32), (i32, i32)) {
                    let section_0 = s.0.split_once('-').unwrap();
                    let section_1 = s.1.split_once('-').unwrap();
                    (
                        (
                            section_0.0.parse::<i32>().unwrap(),
                            section_0.1.parse::<i32>().unwrap(),
                        ),
                        (
                            section_1.0.parse::<i32>().unwrap(),
                            section_1.1.parse::<i32>().unwrap(),
                        ),
                    )
                })
                .unwrap();

            match day {
                "a" => {
                    if (pair.0 .0 >= pair.1 .0 && pair.0 .1 <= pair.1 .1)
                        || (pair.1 .0 >= pair.0 .0 && pair.1 .1 <= pair.0 .1)
                    {
                        contained_pairs.push(pair);
                    }
                },
                "b" => {
                    if (pair.0 .0 >= pair.1 .0 && pair.0 .0 <= pair.1 .1)
                        || (pair.0 .1 >= pair.1 .0 && pair.0 .1 <= pair.1 .1)
                        || (pair.1 .0 >= pair.0 .0 && pair.1 .0 <= pair.0 .1)
                        || (pair.1 .1 >= pair.0 .0 && pair.1 .1 <= pair.0 .1)
                    {
                        contained_pairs.push(pair);
                    }
                },
                _ => {}
            }
            
        }
    }

    contained_pairs
}

pub fn solution_a() -> String {
    let file = File::open("src/year2022/day04_input.txt").expect("Error on File::open");
    let data = file_to_vec::<String>(file);
    format!("{}", get_contained_pairs(data, "a").len())
}

pub fn solution_b() -> String {
    let file = File::open("src/year2022/day04_input.txt").expect("Error on File::open");
    let data = file_to_vec::<String>(file);
    format!("{}", get_contained_pairs(data, "b").len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_contained_pairs_a_1() {
        let data = vec![
            Some("2-4,6-8".to_string()),
            Some("2-3,4-5".to_string()),
            Some("5-7,7-9".to_string()),
            Some("2-8,3-7".to_string()),
            Some("6-6,4-6".to_string()),
            Some("2-6,4-8".to_string()),
        ];
        assert_eq!(get_contained_pairs(data, "a").len(), 2);
    }

    #[test]
    fn get_contained_pairs_a_2() {
        let data = vec![Some("9-84,7-9".to_string())];
        assert_eq!(get_contained_pairs(data, "a").len(), 0);
    }

    #[test]
    fn get_contained_pairs_b_1() {
        let data = vec![
            Some("2-4,6-8".to_string()),
            Some("2-3,4-5".to_string()),
            Some("5-7,7-9".to_string()),
            Some("2-8,3-7".to_string()),
            Some("6-6,4-6".to_string()),
            Some("2-6,4-8".to_string()),
        ];
        assert_eq!(get_contained_pairs(data, "b").len(), 4);
    }
}
