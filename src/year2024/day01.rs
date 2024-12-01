use std::io::{BufRead, BufReader};

use crate::utils::get_input_reader;

/** 
 * Function that takes two Vec<i32> and iterates through i...len() and adds the difference of the pair to an i32 and returns the i32
 */
fn calculate_distances(x: Vec<i32>, y: Vec<i32>) -> i32 {
    let mut distance = 0;
    for i in 0..x.len() {
        distance += (x[i] - y[i]).abs();
    }
    distance
}

/**
 * Function that takes a BufReader<File> splits a line by space into two i32 and adds them to two seperate Vec<i32> and returns the two Vec<i32>
 */
fn parse_input(data: BufReader<std::fs::File>) -> (Vec<i32>, Vec<i32>) {
    let mut x: Vec<i32> = Vec::new();
    let mut y: Vec<i32> = Vec::new();

    for line in data.lines() {
        let l = line.unwrap();
        let mut line_values = l.split_whitespace();
        x.push(line_values.next().unwrap().parse().unwrap());
        y.push(line_values.next().unwrap().parse().unwrap());
    }
    x.sort();
    y.sort();
    (x, y)
}

pub fn solution_a() -> String {
    let data = get_input_reader("src/year2024/day01_input.txt");
    let (x, y) = parse_input(data);
    format!("{}", calculate_distances(x, y))
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1, 1);
    }
}
*/
