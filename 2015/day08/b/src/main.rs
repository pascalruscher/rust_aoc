use std::{fs::File, io::Read};

fn get_char_count(data: String) -> usize {
    let mut char_count: usize = 0;
    let lines = data.lines();
    for line in lines {
        char_count += line.escape_default().count() + 2 - line.len();
    }
    char_count
}

fn main() {
    let mut data = String::new();
    let mut f = File::open("2015/day08/input.txt").expect("Error on File::open");
    f.read_to_string(&mut data)
        .expect("Error on read_to_string");

    println!("Solution: {:?}", get_char_count(data));
}
