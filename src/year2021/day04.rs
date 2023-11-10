use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn get_winner_board_a(
    mut boards: Vec<Vec<Vec<i8>>>,
    draw_numbers: Vec<i8>,
) -> Result<(i8, Vec<Vec<i8>>), &'static str> {
    for number in draw_numbers {
        for board_id in 0..boards.len() {
            let mut col_nums_found = vec![0; boards[board_id].len()];
            for row_id in 0..boards[board_id].len() {
                let mut row_nums_found: usize = 0;
                for number_id in 0..boards[board_id][row_id].len() {
                    if boards[board_id][row_id][number_id] == -1 {
                        col_nums_found[number_id] += 1;
                        row_nums_found += 1;
                    }
                    if number == boards[board_id][row_id][number_id] {
                        boards[board_id][row_id][number_id] = -1;
                        row_nums_found += 1;
                        col_nums_found[number_id] += 1;
                    }
                }

                if row_nums_found == boards[board_id][row_id].len()
                    || col_nums_found.contains(&boards[board_id].len())
                {
                    let found_board = &boards[board_id];
                    return Ok((number, found_board.to_vec()));
                }
            }
        }
    }
    return Err("No board won");
}

pub fn get_winner_board_b(
    mut boards: Vec<Vec<Vec<i8>>>,
    draw_numbers: Vec<i8>,
) -> Result<(i8, Vec<Vec<i8>>), &'static str> {
    let mut found_boards: Vec<usize> = Vec::new();
    for number in draw_numbers {
        for board_id in 0..boards.len() {
            let mut col_nums_found = vec![0; boards[board_id].len()];
            for row_id in 0..boards[board_id].len() {
                let mut row_nums_found: usize = 0;
                for number_id in 0..boards[board_id][row_id].len() {
                    if boards[board_id][row_id][number_id] == -1 {
                        col_nums_found[number_id] += 1;
                        row_nums_found += 1;
                    }
                    if number == boards[board_id][row_id][number_id] {
                        boards[board_id][row_id][number_id] = -1;
                        row_nums_found += 1;
                        col_nums_found[number_id] += 1;
                    }
                }
                if row_nums_found == boards[board_id][row_id].len()
                    || col_nums_found.contains(&boards[board_id][row_id].len())
                {
                    if !found_boards.contains(&board_id) {
                        found_boards.push(board_id);
                    }
                    if found_boards.len() == boards.len() {
                        let found_board = &boards[board_id];
                        return Ok((number, found_board.to_vec()));
                    }
                }
            }
        }
    }
    return Err("No board won");
}


pub fn solution_a() -> String {
    let file = File::open("src/year2021/day04_input.txt").unwrap();
    let mut reader = BufReader::new(file);

    let mut first_line = String::new();
    reader.read_line(&mut first_line).unwrap();
    let draw_numbers: Vec<i8> = first_line
        .trim_end()
        .split(',')
        .map(|n| n.parse::<i8>().unwrap())
        .collect();

    let mut boards: Vec<Vec<Vec<i8>>> = Vec::new();
    let mut board_count: usize = 0;

    for line in reader.lines() {
        let line_len = line.as_ref().unwrap().len();
        if line_len == 0 {
            boards.push(Vec::new());
            if boards.len() > 0 {
                board_count += 1;
            }
            continue;
        }
        let row_numbers: Vec<i8> = line
            .as_ref()
            .unwrap()
            .split(' ')
            .filter(|n| n.len() > 0)
            .map(|n| n.parse::<i8>().unwrap())
            .collect();
        boards[board_count - 1].push(row_numbers);
    }

    let win_result = get_winner_board_b(boards, draw_numbers);
    if win_result.is_ok() {
        let mut unmarked_score: i32 = 0;
        let last_found = win_result.as_ref().unwrap().0;
        for rows in win_result.unwrap().1 {
            for number in rows {
                if number != -1 {
                    unmarked_score += number as i32;
                }
            }
        }
        return format!("{}", last_found as i32 * unmarked_score);
    }
    "Invalid".to_string()
}

pub fn solution_b() -> String {
    let file = File::open("src/year2021/day04_input.txt").unwrap();
    let mut reader = BufReader::new(file);

    let mut first_line = String::new();
    reader.read_line(&mut first_line).unwrap();
    let draw_numbers: Vec<i8> = first_line
        .trim_end()
        .split(',')
        .map(|n| n.parse::<i8>().unwrap())
        .collect();

    let mut boards: Vec<Vec<Vec<i8>>> = Vec::new();
    let mut board_count: usize = 0;

    for line in reader.lines() {
        let line_len = line.as_ref().unwrap().len();
        if line_len == 0 {
            boards.push(Vec::new());
            if boards.len() > 0 {
                board_count += 1;
            }
            continue;
        }
        let row_numbers: Vec<i8> = line
            .as_ref()
            .unwrap()
            .split(' ')
            .filter(|n| n.len() > 0)
            .map(|n| n.parse::<i8>().unwrap())
            .collect();
        boards[board_count - 1].push(row_numbers);
    }

    let win_result = get_winner_board_a(boards, draw_numbers);
    if win_result.is_ok() {
        let mut unmarked_score: i32 = 0;
        let last_found = win_result.as_ref().unwrap().0;
        for rows in win_result.unwrap().1 {
            for number in rows {
                if number != -1 {
                    unmarked_score += number as i32;
                }
            }
        }
        return format!("{}", last_found as i32 * unmarked_score);
    }
    "Invalid".to_string()
}