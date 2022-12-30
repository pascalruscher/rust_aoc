use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn file_to_grid(file: File) -> Vec<Vec<u32>> {
    let reader = BufReader::new(file);
    let mut data = Vec::new();

    for line in reader.lines() {
        let items: Vec<u32> = line.unwrap().chars().flat_map(|c| c.to_digit(10)).collect();
        data.push(items);
    }
    data
}

fn count_trees(data: &Vec<Vec<u32>>) -> i32 {
    let mut count: i32 = 0;
    for (y, row) in data.into_iter().enumerate() {
        if y == 0 || y == data.len() - 1 {
            count += data.len() as i32;
            continue;
        }
        for (x, tree) in row.into_iter().enumerate() {
            if x == 0 || x == row.len() - 1 {
                count += 1;
                continue;
            }

            // Check right side
            let mut tmp_x = x;
            let mut tmp_y = y;

            let mut is_higher_right = false;
            while tmp_x < row.len() - 1 && !is_higher_right {
                tmp_x += 1;
                if data[tmp_y][tmp_x] >= *tree {
                    is_higher_right = true;
                }
            }

            // Check left side
            tmp_x = x;

            let mut is_higher_left = false;
            while tmp_x != 0 && !is_higher_left {
                tmp_x -= 1;
                if data[tmp_y][tmp_x] >= *tree {
                    is_higher_left = true;
                }
            }

            // Check top
            tmp_x = x;

            let mut is_higher_top = false;
            while tmp_y != 0 && !is_higher_top {
                tmp_y -= 1;
                if data[tmp_y][tmp_x] >= *tree {
                    is_higher_top = true;
                }
            }

            // Check bottom
            tmp_y = y;

            let mut is_higher_bottom = false;
            while tmp_y < data.len() - 1 && !is_higher_bottom {
                tmp_y += 1;
                if data[tmp_y][tmp_x] >= *tree {
                    is_higher_bottom = true;
                }
            }

            if !(is_higher_left && is_higher_right && is_higher_top && is_higher_bottom) {
                count += 1;
                continue;
            }
        }
    }
    count
}

fn main() {
    let file = File::open("2022/day08/input.txt").expect("Error on File::open");
    let data = file_to_grid(file);
    println!("{}", count_trees(&data));
}
