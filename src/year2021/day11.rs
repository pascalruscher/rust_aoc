use std::io::BufRead;

use crate::utils::get_input_reader;

const GRID_SIZE: usize = 10;

pub fn flash(grid: &mut Vec<Vec<i8>>, y: i8, x: i8, flash_count: &mut u32) {
    let mut flash_positions: Vec<String> = Vec::new();
    let ops: Vec<(i8, i8)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for op in ops {
        let pos_y = y + op.0;
        let pos_x = x + op.1;
        if pos_y >= 0 && pos_y < GRID_SIZE as i8 && pos_x >= 0 && pos_x < GRID_SIZE as i8 {
            if grid[pos_y as usize][pos_x as usize] < 10 {
                grid[pos_y as usize][pos_x as usize] += 1;
                if grid[pos_y as usize][pos_x as usize] == 10 {
                    *flash_count += 1;
                    flash_positions.push(format!("{},{}", y + op.0, x + op.1));
                }
            }
        }
    }

    if flash_positions.len() > 0 {
        for position in flash_positions {
            let coord: Vec<&str> = position.split(',').collect();
            flash(
                grid,
                coord[0].parse::<i8>().unwrap(),
                coord[1].parse::<i8>().unwrap(),
                flash_count,
            );
        }
    }
}

pub fn solution_a() -> String {
    let reader = get_input_reader("src/year2021/day11_input.txt");
    let rows = reader.lines().map(|s| s.unwrap()).collect::<Vec<_>>();

    let mut grid = vec![vec![0; GRID_SIZE]; GRID_SIZE];
    for (y, row) in rows.iter().enumerate() {
        let row_values: Vec<char> = row.chars().collect();
        for (x, val) in row_values.iter().enumerate() {
            grid[y][x] = val.to_digit(10).unwrap() as i8;
        }
    }

    let mut flash_count: u32 = 0;
    for _ in 0..100 {
        let mut flash_positions: Vec<String> = Vec::new();
        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                if grid[y][x] < 10 {
                    grid[y][x] += 1;
                    if grid[y][x] == 10 {
                        flash_count += 1;
                        flash_positions.push(format!("{},{}", y, x));
                    }
                }
            }
        }

        if flash_positions.len() > 0 {
            for position in flash_positions {
                let coord: Vec<&str> = position.split(',').collect();
                flash(
                    &mut grid,
                    coord[0].parse::<i8>().unwrap(),
                    coord[1].parse::<i8>().unwrap(),
                    &mut flash_count,
                );
            }
        }

        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                if grid[y][x] == 10 {
                    grid[y][x] = 0;
                }
            }
        }
    }
    format!("{}", flash_count)
}

pub fn solution_b() -> String {
    let reader = get_input_reader("src/year2021/day11_input.txt");
    let rows = reader.lines().map(|s| s.unwrap()).collect::<Vec<_>>();

    let mut grid = vec![vec![0; GRID_SIZE]; GRID_SIZE];
    for (y, row) in rows.iter().enumerate() {
        let row_values: Vec<char> = row.chars().collect();
        for (x, val) in row_values.iter().enumerate() {
            grid[y][x] = val.to_digit(10).unwrap() as i8;
        }
    }

    let mut flash_count: u32 = 0;
    let mut all_flashed_turn = 0;
    let octo_count = GRID_SIZE * GRID_SIZE;

    loop {
        all_flashed_turn += 1;
        let mut flash_positions: Vec<String> = Vec::new();
        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                if grid[y][x] < 10 {
                    grid[y][x] += 1;
                    if grid[y][x] == 10 {
                        flash_count += 1;
                        flash_positions.push(format!("{},{}", y, x));
                    }
                }
            }
        }

        if flash_positions.len() > 0 {
            for position in flash_positions {
                let coord: Vec<&str> = position.split(',').collect();
                flash(
                    &mut grid,
                    coord[0].parse::<i8>().unwrap(),
                    coord[1].parse::<i8>().unwrap(),
                    &mut flash_count,
                );
            }
        }

        let mut flashed = 0;
        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                if grid[y][x] == 10 {
                    flashed += 1;
                    grid[y][x] = 0;
                }
            }
        }

        if flashed == octo_count {
            break;
        }
    }
    format!("{}", all_flashed_turn)
}
