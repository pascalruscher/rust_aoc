use std::{collections::HashMap, io::BufRead};

use crate::utils::get_input_reader;

pub fn explore_basin(
    grid: &HashMap<i32, HashMap<i32, u8>>,
    y: i32,
    x: i32,
    visited: &mut Vec<String>,
) -> u32 {
    if y < 0
        || y >= grid.len() as i32
        || x < 0
        || x >= grid[&y].len() as i32
        || grid[&y][&x] == 9
        || visited.contains(&format!("{},{}", y, x))
    {
        return 0;
    }
    visited.push(format!("{},{}", y, x));

    let mut size: u32 = 1;
    size += explore_basin(grid, y - 1, x, visited);
    size += explore_basin(grid, y + 1, x, visited);
    size += explore_basin(grid, y, x - 1, visited);
    size += explore_basin(grid, y, x + 1, visited);
    size
}

pub fn solution_a() -> String {
    let reader = get_input_reader("src/year2021/day09_input.txt");
    let rows = reader.lines().map(|s| s.unwrap()).collect::<Vec<_>>();

    let mut grid: HashMap<usize, HashMap<usize, u8>> = HashMap::new();
    for (y, row) in rows.iter().enumerate() {
        let mut x_values: HashMap<usize, u8> = HashMap::new();
        for (x, val) in row.chars().enumerate() {
            x_values.entry(x).or_insert(val.to_digit(10).unwrap() as u8);
        }
        grid.entry(y).or_insert(x_values);
    }

    let mut risk_level: u32 = 0;
    for y in 0..grid.len() {
        for x in 0..grid[&(y)].len() {
            let mut is_smallest: bool = true;
            if (y > 0 && grid[&y][&x] >= grid[&(y - 1)][&x])
                || (x < grid[&y].len() - 1 && grid[&y][&x] >= grid[&y][&(x + 1)])
                || (y < grid.len() - 1 && grid[&y][&x] >= grid[&(y + 1)][&x])
                || (x > 0 && grid[&y][&x] >= grid[&y][&(x - 1)])
            {
                is_smallest = false;
            }
            if is_smallest {
                risk_level += (1 + grid[&y][&x]) as u32;
            }
        }
    }
    format!("{}", risk_level)
}

pub fn solution_b() -> String {
    let reader = get_input_reader("src/year2021/day09_input.txt");
    let rows = reader.lines().map(|s| s.unwrap()).collect::<Vec<_>>();

    let mut grid: HashMap<i32, HashMap<i32, u8>> = HashMap::new();
    for (y, row) in rows.iter().enumerate() {
        let mut x_values: HashMap<i32, u8> = HashMap::new();
        for (x, val) in row.chars().enumerate() {
            x_values
                .entry(x as i32)
                .or_insert(val.to_digit(10).unwrap() as u8);
        }
        grid.entry(y as i32).or_insert(x_values);
    }

    let mut visited: Vec<String> = Vec::new();
    let mut basin_sizes: Vec<u32> = Vec::new();
    for y in 0..grid.len() {
        for x in 0..grid[&(y as i32)].len() {
            let basin_size = explore_basin(&grid, y as i32, x as i32, &mut visited);
            if basin_size > 0 {
                basin_sizes.push(basin_size);
            }
        }
    }
    basin_sizes.sort_by(|a, b| b.cmp(a));
    format!("{}", basin_sizes[0] * basin_sizes[1] * basin_sizes[2])
}
