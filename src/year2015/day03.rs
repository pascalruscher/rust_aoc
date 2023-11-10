use std::{collections::HashMap, fs::File, io::Read};

fn get_houses_a(data: String) -> HashMap<String, i32> {
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

fn get_houses_b(data: String) -> HashMap<String, i32> {
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

pub fn solution_a() -> String {
    let mut data = String::new();
    let mut file = File::open("src/year2015/day03_input.txt").expect("Error on File::open");
    file.read_to_string(&mut data)
        .expect("Error on read_to_string");
    format!("{}", get_houses_a(data).len())
}

pub fn solution_b() -> String {
    let mut data = String::new();
    let mut file = File::open("src/year2015/day03_input.txt").expect("Error on File::open");
    file.read_to_string(&mut data)
        .expect("Error on read_to_string");
    format!("{}", get_houses_b(data).len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_houses_a_1() {
        // > delivers presents to 2 houses
        assert_eq!(get_houses_a(String::from(">")).len(), 2);
    }

    #[test]
    fn get_houses_a_2() {
        // ^>v< delivers presents to 4 houses
        assert_eq!(get_houses_a(String::from("^>v<")).len(), 4);
    }

    #[test]
    fn get_houses_a_3() {
        // ^v^v^v^v^v delivers a bunch of presents to only 2 houses
        assert_eq!(get_houses_a(String::from("^v^v^v^v^v")).len(), 2);
    }

    #[test]
    fn get_houses_b_1() {
        // ^v delivers presents to 3 houses
        assert_eq!(get_houses_b(String::from("^v")).len(), 3);
    }

    #[test]
    fn get_houses_b_2() {
        // ^>v< now delivers presents to 3 houses
        assert_eq!(get_houses_b(String::from("^>v<")).len(), 3);
    }

    #[test]
    fn get_houses_b_3() {
        // ^v^v^v^v^v now delivers presents to 11 houses
        assert_eq!(get_houses_b(String::from("^v^v^v^v^v")).len(), 11);
    }
}
