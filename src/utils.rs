use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
    str::FromStr,
};

pub fn get_input_string(path: &str) -> String {
    let mut data = String::new();
    let mut file = File::open(path).expect("Error on File::open");
    file.read_to_string(&mut data)
        .expect("Error on read_to_string");
    data
}

pub fn get_input_reader(path: &str) -> BufReader<File> {
    let file = File::open(path).expect("Error on File::open");
    BufReader::new(file)
}

pub fn get_input_vec<T: FromStr>(path: &str) -> Vec<Option<T>> {
    let reader = get_input_reader(path);
    let mut data = Vec::new();
    for line in reader.lines() {
        data.push({
            let this = line.unwrap().parse::<T>();
            match this {
                Ok(t) => Some(t),
                Err(_) => None,
            }
        });
    }
    data
}
