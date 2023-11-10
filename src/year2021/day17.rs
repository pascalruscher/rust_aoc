use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn calc_x_velo(x: i32) -> i32 {
    (x as f32 * 2.).sqrt() as i32
}

pub fn calc_y_velo(y1: i32, y2: i32) -> i32 {
    let mut smallest_y = y1;
    if smallest_y > y2 {
        smallest_y = y2;
    }
    smallest_y * -1 - 1
}

pub fn calc_coord(velo: i32) -> i32 {
    velo * (velo + 1) / 2
}

pub fn solution_a() -> String {
    let file = File::open("src/year2021/day17_input.txt").unwrap();
    let mut reader = BufReader::new(file);

    let mut input = String::new();
    reader.read_line(&mut input).unwrap();

    let target = &input.trim()[13..]
        .split_once(", ")
        .map(|(a, b)| {
            let x = a[2..]
                .split_once("..")
                .map(|(c, d)| {
                    let x1 = i32::from_str_radix(&c, 10).unwrap();
                    let x2 = i32::from_str_radix(&d, 10).unwrap();
                    (x1, x2)
                })
                .unwrap();
            let y = b[2..]
                .split_once("..")
                .map(|(c, d)| {
                    let y1 = i32::from_str_radix(&c, 10).unwrap();
                    let y2 = i32::from_str_radix(&d, 10).unwrap();
                    (y1, y2)
                })
                .unwrap();
            vec![x, y]
        })
        .unwrap();

    let velo_y_max = calc_y_velo(target[1].0, target[1].1);
    format!("{}", calc_coord(velo_y_max))
}

pub fn solution_b() -> String {
    let file = File::open("src/year2021/day17_input.txt").unwrap();
    let mut reader = BufReader::new(file);

    let mut input = String::new();
    reader.read_line(&mut input).unwrap();

    let target = &input.trim()[13..]
        .split_once(", ")
        .map(|(a, b)| {
            let x = a[2..]
                .split_once("..")
                .map(|(c, d)| {
                    let x1 = i32::from_str_radix(&c, 10).unwrap();
                    let x2 = i32::from_str_radix(&d, 10).unwrap();
                    (x1, x2)
                })
                .unwrap();
            let y = b[2..]
                .split_once("..")
                .map(|(c, d)| {
                    let y1 = i32::from_str_radix(&c, 10).unwrap();
                    let y2 = i32::from_str_radix(&d, 10).unwrap();
                    (y1, y2)
                })
                .unwrap();
            vec![x, y]
        })
        .unwrap();

    let velo_y_max = calc_y_velo(target[1].0, target[1].1);
    let velo_x_min = calc_x_velo(target[0].0);

    let mut hit_count = 0;
    for start_x_velo in velo_x_min..=target[0].1 {
        for start_y_velo in target[1].0..=velo_y_max {
            let mut cur_x = 0;
            let mut cur_y = 0;
            let mut cur_x_velo = start_x_velo;
            let mut cur_y_velo = start_y_velo;

            loop {
                cur_x += cur_x_velo;
                cur_y += cur_y_velo;

                if cur_x_velo > 0 {
                    cur_x_velo -= 1;
                }
                cur_y_velo -= 1;

                if cur_x > target[0].1 || cur_y < target[1].0 {
                    break;
                }

                if cur_x >= target[0].0
                    && cur_x <= target[0].1
                    && cur_y >= target[1].0
                    && cur_y <= target[1].1
                {
                    hit_count += 1;
                    break;
                }
            }
        }
    }
    format!("{}", hit_count)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_x_velo() {
        let mut target_x_min = 20;
        let mut velo = calc_x_velo(target_x_min);
        let mut x_coord = calc_coord(velo);
        assert_eq!(x_coord >= target_x_min, true);

        target_x_min = 25;
        velo = calc_x_velo(target_x_min);
        x_coord = calc_coord(velo);
        assert_eq!(x_coord >= target_x_min, true);

        target_x_min = 1010101;
        velo = calc_x_velo(target_x_min);
        x_coord = calc_coord(velo);
        assert_eq!(x_coord >= target_x_min, true);

        target_x_min = -11;
        velo = calc_x_velo(target_x_min);
        x_coord = calc_coord(velo);
        assert_eq!(x_coord >= target_x_min, true);

        target_x_min = -33;
        velo = calc_x_velo(target_x_min);
        x_coord = calc_coord(velo);
        assert_eq!(x_coord >= target_x_min, true);

        target_x_min = -12121217;
        velo = calc_x_velo(target_x_min);
        x_coord = calc_coord(velo);
        assert_eq!(x_coord >= target_x_min, true);
    }
}
