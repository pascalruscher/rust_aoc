use crate::utils::get_input_vec;

fn get_total_calories(data: Vec<Option<i32>>, day: &str) -> i32 {
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

    match day {
        "a" => total_calories[0],
        "b" => total_calories[0] + total_calories[1] + total_calories[2],
        _ => 0,
    }
}

pub fn solution_a() -> String {
    let data = get_input_vec::<i32>("src/year2022/day01_input.txt");
    format!("{}", get_total_calories(data, "a"))
}

pub fn solution_b() -> String {
    let data = get_input_vec::<i32>("src/year2022/day01_input.txt");
    format!("{}", get_total_calories(data, "b"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn highest_calories_a() {
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
        assert_eq!(get_total_calories(data, "a"), 24000);
    }

    #[test]
    fn highest_calories_b() {
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
        assert_eq!(get_total_calories(data, "b"), 45000);
    }
}
