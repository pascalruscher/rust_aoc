use crate::utils::get_input_vec;

fn init_crates(data: &mut Vec<Option<String>>) -> Vec<Vec<char>> {
    let mut initial_state = Vec::new();
    data.reverse();

    // Prepare our initial state
    loop {
        let state_item = data.pop().unwrap();
        if let Some(state) = state_item {
            if state.len() > 0 {
                initial_state.push(state);
                continue;
            }
        }
        break;
    }

    // Last element is number of stacks
    let stack_size: usize = *initial_state
        .pop()
        .unwrap()
        .split_whitespace()
        .filter_map(|s| s.parse::<usize>().ok())
        .collect::<Vec<usize>>()
        .last()
        .unwrap();

    let mut crates: Vec<Vec<char>> = vec![Vec::new(); stack_size];

    while let Some(row) = initial_state.pop() {
        for i in 0..stack_size {
            let start = i * 4;
            let mut end = start + 4;
            if end > row.len() {
                end = row.len();
            }

            let item = &row[start..end].trim();
            if item.len() > 0 {
                crates[i].push(item.chars().nth(1).unwrap());
            }
        }
    }

    crates
}

fn parse_instruction(instruction: String) -> Vec<usize> {
    let mut parsed = instruction
        .split_whitespace()
        .filter_map(|s| s.parse::<usize>().ok())
        .collect::<Vec<usize>>();

    // Reduce targets by 1 because we start counting at 0
    parsed[1] = parsed[1] - 1;
    parsed[2] = parsed[2] - 1;

    parsed
}

fn move_crates_a(crates: &mut Vec<Vec<char>>, instructions: &mut Vec<Option<String>>) {
    while let Some(instruction) = instructions.pop() {
        let moves = parse_instruction(instruction.unwrap());

        for _ in 0..moves[0] {
            let crate_item = crates[moves[1]].pop().unwrap();
            crates[moves[2]].push(crate_item);
        }
    }
}

fn move_crates_b(crates: &mut Vec<Vec<char>>, instructions: &mut Vec<Option<String>>) {
    while let Some(instruction) = instructions.pop() {
        let moves = parse_instruction(instruction.unwrap());
        let mut crate_stack = Vec::new();

        for _ in 0..moves[0] {
            let crate_item = crates[moves[1]].pop().unwrap();
            crate_stack.push(crate_item);
        }

        for _ in 0..crate_stack.len() {
            crates[moves[2]].push(crate_stack.pop().unwrap());
        }
    }
}

fn get_top_crates(crates: Vec<Vec<char>>) -> String {
    let mut top_crates = String::new();
    for crate_items in crates {
        top_crates.push(*crate_items.last().unwrap());
    }
    top_crates
}

pub fn solution_a() -> String {
    let mut data = get_input_vec::<String>("src/year2022/day04_input.txt");
    let mut crates = init_crates(&mut data);
    move_crates_a(&mut crates, &mut data);
    get_top_crates(crates)
}

pub fn solution_b() -> String {
    let mut data = get_input_vec::<String>("src/year2022/day04_input.txt");
    let mut crates = init_crates(&mut data);
    move_crates_b(&mut crates, &mut data);
    get_top_crates(crates)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_top_crates_a_1() {
        let mut data = vec![
            Some("    [D]    ".to_string()),
            Some("[N] [C]    ".to_string()),
            Some("[Z] [M] [P]".to_string()),
            Some(" 1   2   3 ".to_string()),
            Some("".to_string()),
            Some("move 1 from 2 to 1".to_string()),
            Some("move 3 from 1 to 3".to_string()),
            Some("move 2 from 2 to 1".to_string()),
            Some("move 1 from 1 to 2".to_string()),
        ];
        let mut crates = init_crates(&mut data);

        move_crates_a(&mut crates, &mut data);

        assert_eq!(get_top_crates(crates), "CMZ");
    }

    #[test]
    fn get_top_crates_b_1() {
        let mut data = vec![
            Some("    [D]    ".to_string()),
            Some("[N] [C]    ".to_string()),
            Some("[Z] [M] [P]".to_string()),
            Some(" 1   2   3 ".to_string()),
            Some("".to_string()),
            Some("move 1 from 2 to 1".to_string()),
            Some("move 3 from 1 to 3".to_string()),
            Some("move 2 from 2 to 1".to_string()),
            Some("move 1 from 1 to 2".to_string()),
        ];
        let mut crates = init_crates(&mut data);

        move_crates_b(&mut crates, &mut data);

        assert_eq!(get_top_crates(crates), "MCD");
    }
}
