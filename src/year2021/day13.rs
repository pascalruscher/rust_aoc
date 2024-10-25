use std::{
    collections::{HashMap, VecDeque},
    io::BufRead,
};

use crate::utils::get_input_reader;

pub fn fold_grid(grid: HashMap<(i32, i32), char>, fold: (char, i32)) -> HashMap<(i32, i32), char> {
    let mut new_grid: HashMap<(i32, i32), char> = HashMap::new();
    for pos in grid.keys() {
        let (mut x, mut y) = pos;
        if fold.0 == 'y' && y > fold.1 {
            y = fold.1 - (y - fold.1);
        } else if fold.0 == 'x' && x > fold.1 {
            x = fold.1 - (x - fold.1);
        }
        new_grid.entry((x, y)).or_insert('#');
    }
    new_grid
}

pub fn solution_a() -> String {
    let reader = get_input_reader("src/year2021/day13_input.txt");
    let mut lines = reader.lines().map(|s| s.unwrap()).collect::<Vec<_>>();

    let mut fold_que: VecDeque<(char, i32)> = VecDeque::new();
    while lines[lines.len() - 1].len() != 0 {
        let last_line = lines.pop().unwrap();
        let line_value = last_line.split_once('=').unwrap();
        if line_value.0.contains("fold along y") {
            fold_que.push_front(('y', line_value.1.parse::<i32>().unwrap()));
        } else {
            fold_que.push_front(('x', line_value.1.parse::<i32>().unwrap()));
        }
    }
    lines.pop(); // remove empty line

    let mut grid: HashMap<(i32, i32), char> = HashMap::new();
    let fold = fold_que.pop_front().unwrap(); // first fold

    for line in lines {
        let (mut x, mut y) = line
            .split_once(',')
            .map(|s| (s.0.parse::<i32>().unwrap(), s.1.parse::<i32>().unwrap()))
            .unwrap();

        if fold.0 == 'y' && y > fold.1 {
            y = fold.1 - (y - fold.1);
        } else if fold.0 == 'x' && x > fold.1 {
            x = fold.1 - (x - fold.1);
        }
        grid.entry((x, y)).or_insert('#');
    }
    format!("{}", grid.len())
}

pub fn solution_b() -> String {
    let reader = get_input_reader("src/year2021/day13_input.txt");
    let mut lines = reader.lines().map(|s| s.unwrap()).collect::<Vec<_>>();

    let mut fold_que: VecDeque<(char, i32)> = VecDeque::new();
    while lines[lines.len() - 1].len() != 0 {
        let last_line = lines.pop().unwrap();
        let line_value = last_line.split_once('=').unwrap();
        if line_value.0.contains("fold along y") {
            fold_que.push_front(('y', line_value.1.parse::<i32>().unwrap()));
        } else {
            fold_que.push_front(('x', line_value.1.parse::<i32>().unwrap()));
        }
    }
    lines.pop(); // remove empty line

    let mut grid: HashMap<(i32, i32), char> = HashMap::new();
    let fold = fold_que.pop_front().unwrap(); // first fold

    for line in lines {
        let (mut x, mut y) = line
            .split_once(',')
            .map(|s| (s.0.parse::<i32>().unwrap(), s.1.parse::<i32>().unwrap()))
            .unwrap();

        if fold.0 == 'y' && y > fold.1 {
            y = fold.1 - (y - fold.1);
        } else if fold.0 == 'x' && x > fold.1 {
            x = fold.1 - (x - fold.1);
        }
        grid.entry((x, y)).or_insert('#');
    }

    while fold_que.len() > 0 {
        let fold = fold_que.pop_front().unwrap();
        grid = fold_grid(grid, fold);
    }
    format!("{:?}", grid)
}
