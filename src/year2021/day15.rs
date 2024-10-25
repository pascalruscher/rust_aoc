use std::{collections::VecDeque, io::BufRead};

use crate::utils::get_input_reader;

pub fn find_shortest_count(grid: Vec<Vec<u32>>) -> u32 {
    let directions_x: Vec<i8> = vec![-1, 0, 1, 0];
    let directions_y: Vec<i8> = vec![0, 1, 0, -1];

    let mut distance = vec![vec![u32::MAX; grid[0].len()]; grid.len()];
    distance[0][0] = 0;

    // (x, y, distance)
    let mut distance_sort: VecDeque<(u32, u32, u32)> = VecDeque::new();
    distance_sort.push_back((0, 0, 0));

    while distance_sort.len() != 0 {
        let min_distance = distance_sort.pop_front().unwrap();

        for i in 0..4 {
            let x = min_distance.0 as i32 + directions_x[i] as i32;
            let y = min_distance.1 as i32 + directions_y[i] as i32;

            if x >= grid[0].len() as i32 || x < 0 || y >= grid.len() as i32 || y < 0 {
                continue;
            }

            if distance[y as usize][x as usize]
                > distance[min_distance.1 as usize][min_distance.0 as usize]
                    + grid[y as usize][x as usize]
            {
                distance[y as usize][x as usize] = distance[min_distance.1 as usize]
                    [min_distance.0 as usize]
                    + grid[y as usize][x as usize];
                distance_sort.push_back((x as u32, y as u32, distance[y as usize][x as usize]));
            }
        }
        distance_sort.make_contiguous().sort_by(|a, b| {
            if a.2 == b.2 {
                if a.0 != b.0 {
                    return a.0.cmp(&b.0);
                } else {
                    return a.1.cmp(&b.1);
                }
            }
            return a.2.cmp(&b.2);
        });
    }

    return distance[grid.len() - 1][grid[0].len() - 1];
}

pub fn solution_a() -> String {
    let reader = get_input_reader("src/year2021/day15_input.txt");

    let mut grid: Vec<Vec<u32>> = Vec::new();
    for line in reader.lines() {
        let x_values: Vec<u32> = line
            .unwrap()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        grid.push(x_values);
    }
    format!("{}", find_shortest_count(grid))
}

pub fn solution_b() -> String {
    let reader = get_input_reader("src/year2021/day15_input.txt");

    let mut initial_grid: Vec<Vec<u32>> = Vec::new();
    for line in reader.lines() {
        let x_values: Vec<u32> = line
            .unwrap()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        initial_grid.push(x_values);
    }

    let mut grid: Vec<Vec<u32>> = Vec::new();
    for i in 0..5 {
        for y in 0..initial_grid.len() {
            let mut x_values = Vec::new();
            for j in 0..5 {
                for x in 0..initial_grid[y].len() {
                    x_values.push((initial_grid[y][x] + i + j - 1) % 9 + 1);
                }
            }
            grid.push(x_values);
        }
    }
    format!("{}", find_shortest_count(grid))
}
