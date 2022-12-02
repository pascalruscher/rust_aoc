use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

fn file_to_vec<T: FromStr>(file: File) -> Vec<Option<T>> {
    let reader = BufReader::new(file);
    let mut data = Vec::new();

    for (_, line) in reader.lines().enumerate() {
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

fn get_total_calories(data: Vec<Option<i32>>) -> i32 {
    let mut total_calories: Vec<i32> = Vec::new();
    let mut elve_id = 0;
    // first elve calories
    total_calories.push(0);

    for calories in data {
        if let Some(calories_count) = calories {
            total_calories[elve_id] += calories_count;
        } else {
            // None in our vec means there is a new elve
            elve_id += 1;
            total_calories.push(0);
        }
    }

    total_calories.sort_unstable();
    total_calories.reverse();
    total_calories[0] + total_calories[1] + total_calories[2]
}

fn main() {
    let file = File::open("2022/day01/input.txt").expect("Error on File::open");
    let data = file_to_vec::<i32>(file);
    println!("{}", get_total_calories(data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn highest_calories() {
        let data = vec![
            Some(1000),
            Some(2000),
            Some(3000),
            None,
            Some(4000),
            None,
            Some(5000),
            Some(6000),
            None,
            Some(7000),
            Some(8000),
            Some(9000),
            None,
            Some(10000),
        ];
        assert_eq!(get_total_calories(data), 45000);
    }
}
