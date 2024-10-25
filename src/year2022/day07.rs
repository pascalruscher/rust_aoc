use std::collections::HashMap;

use crate::utils::get_input_vec;

#[derive(Debug, Clone)]
struct Folder {
    children: Vec<String>,
    files: Vec<(String, u64)>,
    size: Option<u64>,
}

impl Folder {
    fn add_child(&mut self, child: String) {
        self.children.push(child);
        self.children.sort_unstable();
        self.children.dedup();
    }

    fn add_file(&mut self, file: (String, u64)) {
        self.files.push(file);
        self.files.sort_unstable();
        self.files.dedup();
    }
}

fn create_tree(data: Vec<Option<String>>) -> HashMap<String, Folder> {
    let mut dir_tree: HashMap<String, Folder> = HashMap::new();
    let mut dir_history: Vec<String> = Vec::new();
    for line in data {
        if let Some(instruction) = line {
            let instruction_parsed: Vec<&str> = instruction.split_whitespace().collect();
            match instruction_parsed[0] {
                "$" => match instruction_parsed[1] {
                    "cd" => match instruction_parsed[2] {
                        ".." => {
                            dir_history.pop();
                        }
                        _ => {
                            dir_history.push(instruction_parsed[2].to_string());
                        }
                    },
                    "ls" => continue,
                    _ => {}
                },
                _ => {
                    let dirname_parent = format!("{}", &dir_history.join("/"));
                    dir_tree.entry(dirname_parent.clone()).or_insert(Folder {
                        children: Vec::new(),
                        files: Vec::new(),
                        size: None,
                    });
                    match instruction_parsed[0] {
                        "dir" => {
                            let dirname_child =
                                format!("{}/{}", &dirname_parent, instruction_parsed[1]);
                            dir_tree
                                .get_mut(&dirname_parent.clone())
                                .unwrap()
                                .add_child(dirname_child);
                        }
                        _ => {
                            dir_tree
                                .get_mut(&dirname_parent.clone())
                                .unwrap()
                                .add_file((
                                    instruction_parsed[1].to_string(),
                                    instruction_parsed[0].to_string().parse::<u64>().unwrap(),
                                ));
                        }
                    }
                }
            }
        }
    }
    dir_tree
}

fn calculate_size(folders: &HashMap<String, Folder>, folder_name: &String) -> u64 {
    if !folders.contains_key(folder_name) {
        return 0;
    }
    let folder: &Folder = folders.get(folder_name).unwrap();
    let mut size: u64 = folder.files.clone().into_iter().map(|(_, c)| c).sum();
    for child in &folder.children {
        size += calculate_size(folders, child);
    }
    size
}

pub fn solution_a() -> String {
    let data = get_input_vec::<String>("src/year2022/day07_input.txt");

    let mut folder_tree = create_tree(data);
    let mut solution = 0;

    for (folder_name, _) in folder_tree.clone() {
        let size = calculate_size(&folder_tree.clone(), &folder_name);
        folder_tree.get_mut(&folder_name).unwrap().size = Some(size);

        if size <= 100000 {
            solution += size;
        }
    }

    format!("{}", solution)
}

pub fn solution_b() -> String {
    let data = get_input_vec::<String>("src/year2022/day07_input.txt");

    let mut folder_tree = create_tree(data);
    let space_needed = 30_000_000 - (70_000_000 - calculate_size(&folder_tree, &"/".to_string()));
    let mut solution = u64::MAX;

    for (folder_name, _) in folder_tree.clone() {
        let size = calculate_size(&folder_tree.clone(), &folder_name);
        folder_tree.get_mut(&folder_name).unwrap().size = Some(size);

        if size >= space_needed && solution > size {
            solution = size;
        }
    }

    format!("{}", solution)
}
