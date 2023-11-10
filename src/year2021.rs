pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;

pub fn solution(day: &str) -> String{
    match day {
        "01a" => day01::solution_a(),
        "01b" => day01::solution_b(),
        "02a" => day02::solution_a(),
        "02b" => day02::solution_b(),
        "03a" => day03::solution_a(),
        "03b" => day03::solution_b(),
        "04a" => day04::solution_a(),
        "04b" => day04::solution_b(),
        "05a" => day05::solution_a(),
        "05b" => day05::solution_b(),
        "06a" => day06::solution_a(),
        "06b" => day06::solution_b(),
        "07a" => day07::solution_a(),
        "07b" => day07::solution_b(),
        "08a" => day08::solution_a(),
        "08b" => day08::solution_b(),
        "09a" => day09::solution_a(),
        "09b" => day09::solution_b(),
        "10a" => day10::solution_a(),
        "10b" => day10::solution_b(),
        "11a" => day11::solution_a(),
        "11b" => day11::solution_b(),
        "12a" => day12::solution_a(),
        "12b" => day12::solution_b(),
        "13a" => day13::solution_a(),
        "13b" => day13::solution_b(),
        "14a" => day14::solution_a(),
        "14b" => day14::solution_b(),
        "15a" => day15::solution_a(),
        "15b" => day15::solution_b(),
        "16a" => day16::solution_a(),
        "16b" => day16::solution_b(),
        "17a" => day17::solution_a(),
        "17b" => day17::solution_b(),
        _ => "".to_string()
    }
}