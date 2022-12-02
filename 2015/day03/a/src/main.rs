use std::{collections::HashMap, fs::File, io::Read};

fn get_houses(data: String) -> HashMap<String, i32> {
    let mut houses = HashMap::new();
    let mut x = 0;
    let mut y = 0;
    // start location is always "0,0" as in x=0 y=0
    houses.insert(format!("{},{}", x, y), 1);
    for c in data.chars() {
        match c {
            '>' => x += 1,
            '^' => y -= 1,
            '<' => x -= 1,
            'v' => y += 1,
            _ => {}
        }
        *houses.entry(format!("{},{}", x, y)).or_insert(0) += 1;
    }
    houses
}

fn main() {
    let mut data = String::new();
    let mut f = File::open("2015/day03/input.txt").expect("Error on File::open");
    f.read_to_string(&mut data)
        .expect("Error on read_to_string");

    println!("Houses: {:?}", get_houses(data).len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_houses_1() {
        // > delivers presents to 2 houses
        assert_eq!(get_houses(String::from(">")).len(), 2);
    }

    #[test]
    fn get_houses_2() {
        // ^>v< delivers presents to 4 houses
        assert_eq!(get_houses(String::from("^>v<")).len(), 4);
    }

    #[test]
    fn get_houses_3() {
        // ^v^v^v^v^v delivers a bunch of presents to only 2 houses
        assert_eq!(get_houses(String::from("^v^v^v^v^v")).len(), 2);
    }
}
