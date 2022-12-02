use std::{collections::HashMap, fs::File, io::Read};

#[derive(Debug, Clone)]
struct Instruction {
    operation: Option<String>,
    left: Option<String>,
    right: String,
}

impl Instruction {
    fn from_string(s: String) -> Instruction {
        let instruction_parts: Vec<String> = s.split(' ').map(|s| s.to_owned()).collect();
        if instruction_parts.len() == 1 {
            return Instruction {
                operation: None,
                left: None,
                right: instruction_parts[0].clone(),
            };
        } else if instruction_parts.len() == 2 {
            return Instruction {
                operation: Some(instruction_parts[0].clone()),
                left: None,
                right: instruction_parts[1].clone(),
            };
        }
        Instruction {
            operation: Some(instruction_parts[1].clone()),
            left: Some(instruction_parts[0].clone()),
            right: instruction_parts[2].clone(),
        }
    }
}

fn parse(data: String) -> HashMap<String, Instruction> {
    let mut wires: HashMap<String, Instruction> = HashMap::new();
    let lines = data.lines();
    for line in lines {
        let split: Vec<String> = line.split(" -> ").map(|s| s.to_owned()).collect();
        wires.insert(split[1].clone(), Instruction::from_string(split[0].clone()));
    }
    wires
}

fn get_wire_value(values: &mut HashMap<String, u16>, instruction: &Instruction) -> Option<u16> {
    let mut left_value: Option<u16> = None;
    if let Some(left) = &instruction.left {
        let parsed_left = left.parse::<u16>();
        if let Ok(parsed) = parsed_left {
            left_value = Some(parsed);
        } else if values.contains_key(left.as_str()) {
            left_value = Some(*values.get(left.as_str()).unwrap());
        } else {
            return None;
        }
    }

    let right_value: u16;
    let parsed_right = instruction.right.parse::<u16>();
    if let Ok(parsed) = parsed_right {
        right_value = parsed;
    } else if values.contains_key(instruction.right.as_str()) {
        right_value = *values.get(instruction.right.as_str()).unwrap();
    } else {
        return None;
    }

    if let Some(operation) = &instruction.operation {
        match operation.as_str() {
            "NOT" => return Some(!right_value),
            "AND" => return Some(left_value.unwrap() & right_value),
            "OR" => return Some(left_value.unwrap() | right_value),
            "XOR" => return Some(left_value.unwrap() ^ right_value),
            "LSHIFT" => return Some(left_value.unwrap() << right_value),
            "RSHIFT" => return Some(left_value.unwrap() >> right_value),
            _ => unreachable!(),
        }
    }
    Some(right_value)
}

fn get_wire_values(wires: HashMap<String, Instruction>) -> HashMap<String, u16> {
    let mut values: HashMap<String, u16> = HashMap::new();
    while values.len() < wires.len() {
        for (wire, instruction) in &wires.clone() {
            if values.get(wire).is_some() {
                continue;
            }
            let value = get_wire_value(&mut values, instruction);
            if let Some(v) = value {
                values.insert(wire.to_string(), v);
            }
        }
    }
    values
}

fn main() {
    let mut data = String::new();
    let mut f = File::open("2015/day07/input.txt").expect("Error on File::open");
    f.read_to_string(&mut data)
        .expect("Error on read_to_string");

    let mut wires = parse(data);
    let mut wire_values = get_wire_values(wires.clone());
    let solution_a = wire_values.get("a");
    wires.insert(
        "b".to_string(),
        Instruction {
            operation: None,
            left: None,
            right: format!("{}", solution_a.unwrap()),
        },
    );
    wire_values = get_wire_values(wires);
    println!("Solution: {:?}", wire_values.get("a"));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_data() -> HashMap<String, Instruction> {
        let data = String::from(
            "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i",
        );
        parse(data)
    }

    #[test]
    fn wire_value_d() {
        let wires = get_wire_values(get_data());
        assert_eq!(wires.get("d"), Some(&72));
    }

    #[test]
    fn wire_value_e() {
        let wires = get_wire_values(get_data());
        assert_eq!(wires.get("e"), Some(&507));
    }

    #[test]
    fn wire_value_f() {
        let wires = get_wire_values(get_data());
        assert_eq!(wires.get("f"), Some(&492));
    }

    #[test]
    fn wire_value_g() {
        let wires = get_wire_values(get_data());
        assert_eq!(wires.get("g"), Some(&114));
    }

    #[test]
    fn wire_value_h() {
        let wires = get_wire_values(get_data());
        assert_eq!(wires.get("h"), Some(&65412));
    }

    #[test]
    fn wire_value_i() {
        let wires = get_wire_values(get_data());
        assert_eq!(wires.get("i"), Some(&65079));
    }

    #[test]
    fn wire_value_x() {
        let wires = get_wire_values(get_data());
        assert_eq!(wires.get("x"), Some(&123));
    }

    #[test]
    fn wire_value_y() {
        let wires = get_wire_values(get_data());
        assert_eq!(wires.get("y"), Some(&456));
    }
}
