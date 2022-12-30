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

fn get_tree_score(data: &Vec<Vec<u32>>) -> i32 {
    let mut score: i32 = 0;
    for (y, row) in data.into_iter().enumerate() {
        for (x, tree) in row.into_iter().enumerate() {
            let mut tmp_score: i32;

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
            tmp_score = (tmp_x - x) as i32;

            // Check left side
            tmp_x = x;

            let mut is_higher_left = false;
            while tmp_x != 0 && !is_higher_left {
                tmp_x -= 1;
                if data[tmp_y][tmp_x] >= *tree {
                    is_higher_left = true;
                }
            }
            if tmp_score == 0 {
                tmp_score = (x - tmp_x) as i32;
            } else {
                tmp_score *= (x - tmp_x) as i32;
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
            if tmp_score == 0 {
                tmp_score = (y - tmp_y) as i32;
            } else {
                tmp_score *= (y - tmp_y) as i32;
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
            if tmp_score == 0 {
                tmp_score = (tmp_y - y) as i32;
            } else {
                tmp_score *= (tmp_y - y) as i32;
            }

            if tmp_score > score {
                score = tmp_score;
            }
        }
    }
    score
}

fn main() {
    let file = File::open("2022/day08/input.txt").expect("Error on File::open");
    let data = file_to_grid(file);
    println!("{}", get_tree_score(&data));
}
