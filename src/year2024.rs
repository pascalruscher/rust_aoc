pub mod day01;

pub fn solution(day: &str) -> String{
    match day {
        "01a" => day01::solution_a(),
        _ => "".to_string()
    }
}