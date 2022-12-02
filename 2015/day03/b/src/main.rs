use std::{collections::HashMap, fs::File, io::Read};

fn get_houses(data: String) -> HashMap<String, i32> {
    let mut houses = HashMap::new();
    let mut x = 0;
    let mut y = 0;
    let mut x_robot = 0;
    let mut y_robot = 0;
    // start location is always "0,0" as in x=0 y=0 and both santas start here
    houses.insert(format!("{},{}", 0, 0), 2);
    for (i, c) in data.chars().enumerate() {
        let mut x_mod = 0;
        let mut y_mod = 0;
        match c {
            '>' => x_mod += 1,
            '^' => y_mod -= 1,
            '<' => x_mod -= 1,
            'v' => y_mod += 1,
            _ => {}
        }
        // the santas take turns in moving, so every 2nd move is santa
        if i % 2 == 0 {
            x += x_mod;
            y += y_mod;
            *houses.entry(format!("{},{}", x, y)).or_insert(0) += 1;
        } else {
            x_robot += x_mod;
            y_robot += y_mod;
            *houses
                .entry(format!("{},{}", x_robot, y_robot))
                .or_insert(0) += 1;
        }
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
        // ^v delivers presents to 3 houses
        assert_eq!(get_houses(String::from("^v")).len(), 3);
    }

    #[test]
    fn get_houses_2() {
        // ^>v< now delivers presents to 3 houses
        assert_eq!(get_houses(String::from("^>v<")).len(), 3);
    }

    #[test]
    fn get_houses_3() {
        // ^v^v^v^v^v now delivers presents to 11 houses
        assert_eq!(get_houses(String::from("^v^v^v^v^v")).len(), 11);
    }
}
