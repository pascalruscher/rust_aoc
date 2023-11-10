use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn parse_packet_a(packet: String, packet_limit: Option<u8>) -> u32 {
    if packet.len() == 0 || !packet.contains('1') {
        return 0;
    }

    let mut new_packet_limit = None;
    if packet_limit.is_some() && packet_limit.unwrap() == 0 {
        return parse_packet_a(packet, None);
    } else {
        if packet_limit.is_some() {
            new_packet_limit = Some(packet_limit.unwrap() - 1);
        };
    }

    let mut version = u32::from_str_radix(&packet[0..3], 2).unwrap();
    let type_id = u8::from_str_radix(&packet[3..6], 2).unwrap();

    if type_id == 4 {
        // Literal
        let mut start = 6; // Initial start at position 6 of the packet
        loop {
            if &packet[start..start + 1] == "0" {
                start += 5;
                break;
            }
            start += 5;
        }
        version += parse_packet_a(packet[start..].to_string(), new_packet_limit);
    } else {
        // Operator
        if &packet[6..7] == "0" {
            // 15 bits give length of bits for packet content
            let bit_length = usize::from_str_radix(&packet[7..22], 2).unwrap();
            // Calculate current content and pass on rest of the bit string
            version += parse_packet_a(packet[22..22 + bit_length].to_string(), None)
                + parse_packet_a(packet[22 + bit_length..].to_string(), new_packet_limit);
        } else {
            // 11 bits give length of bits for packet content
            let packet_count = u8::from_str_radix(&packet[7..18], 2).unwrap();
            version += parse_packet_a(packet[18..].to_string(), Some(packet_count));
        }
    }
    version
}

pub fn parse_packet_b(
    packet: String,
    mut start: usize,
    end: Option<usize>,
) -> (Option<u64>, Option<usize>) {
    if end.is_some() && start == end.unwrap() || packet.len() == 0 || !packet.contains("1") {
        return (None, None);
    }
    let type_id = u8::from_str_radix(&packet[start + 3..start + 6], 2).unwrap();

    if type_id == 4 {
        // Literal
        start += 6; // Initial start at position 6 of the packet
        let mut binary_string = String::new();
        loop {
            binary_string.push_str(&packet[start + 1..start + 5].to_string());
            if &packet[start..start + 1] == "0" {
                start += 5;
                break;
            }
            start += 5;
        }
        return (
            Some(u64::from_str_radix(&binary_string, 2).unwrap()),
            Some(start),
        );
    }

    let mut results: Vec<u64> = Vec::new();
    let mut start_new: Option<usize>;

    // Operator
    if &packet[start + 6..start + 7] == "0" {
        // 15 bits give length of bits for packet content
        start_new = Some(start + 22);
        let bit_length = usize::from_str_radix(&packet[start + 7..start_new.unwrap()], 2).unwrap();
        let packet_end = start_new.unwrap() + bit_length;
        loop {
            let (result, next_start_result) =
                parse_packet_b(packet.clone(), start_new.unwrap(), Some(packet_end));
            if result.is_some() {
                results.push(result.unwrap());
            }
            if next_start_result.is_none() {
                break;
            }
            start_new = next_start_result;
        }
    } else {
        // 11 bits give length of bits for packet content
        start_new = Some(start + 18);
        let mut packet_count =
            u32::from_str_radix(&packet[start + 7..start_new.unwrap()], 2).unwrap();

        while packet_count > 0 {
            let (result, next_start_result) =
                parse_packet_b(packet.clone(), start_new.unwrap(), None);
            packet_count -= 1;
            if result.is_some() {
                results.push(result.unwrap());
            }
            start_new = next_start_result;
        }
    }

    match type_id {
        0 => (Some(results.iter().sum()), start_new),
        1 => (Some(results.iter().product()), start_new),
        2 => (Some(*results.iter().min().unwrap()), start_new),
        3 => (Some(*results.iter().max().unwrap()), start_new),
        5 => (Some((results[0] > results[1]) as u64), start_new),
        6 => (Some((results[0] < results[1]) as u64), start_new),
        7 => (Some((results[0] == results[1]) as u64), start_new),
        _ => (None, None),
    }
}

pub fn hex_to_binary(hex: &str) -> String {
    hex.chars()
        .map(|c| {
            let mut binary = format!("{:b}", c.to_digit(16).unwrap());
            let fill_bits = 4 - (binary.len() % 4);
            if fill_bits < 4 {
                binary = format!("{}{}", "0".repeat(fill_bits), binary);
            }
            binary
        })
        .collect()
}

pub fn solution_a() -> String {
    let file = File::open("src/year2021/day16_input.txt").unwrap();
    let mut reader = BufReader::new(file);

    let mut input = String::new();
    reader.read_line(&mut input).unwrap();

    let binary: String = hex_to_binary(input.trim());
    format!("{}", parse_packet_a(binary, None))
}

pub fn solution_b() -> String {
    let file = File::open("src/year2021/day16_input.txt").unwrap();
    let mut reader = BufReader::new(file);

    let mut input = String::new();
    reader.read_line(&mut input).unwrap();

    let binary: String = hex_to_binary(input.trim());
    let result = parse_packet_b(binary, 0, None);
    format!("{}", result.0.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_binary() {
        assert_eq!(hex_to_binary("D2FE28"), "110100101111111000101000");
        assert_eq!(
            hex_to_binary("38006F45291200"),
            "00111000000000000110111101000101001010010001001000000000"
        );
        assert_eq!(
            hex_to_binary("EE00D40C823060"),
            "11101110000000001101010000001100100000100011000001100000"
        );
    }

    #[test]
    fn test_parse_packet_a() {
        assert_eq!(parse_packet_a(hex_to_binary("8A004A801A8002F478"), None), 16);
        assert_eq!(
            parse_packet_a(hex_to_binary("620080001611562C8802118E34"), None),
            12
        );
        assert_eq!(
            parse_packet_a(hex_to_binary("C0015000016115A2E0802F182340"), None),
            23
        );
        assert_eq!(
            parse_packet_a(hex_to_binary("A0016C880162017C3686B18A3D4780"), None),
            31
        );
    }

    #[test]
    fn test_parse_packet_b() {
        assert_eq!(
            parse_packet_b(hex_to_binary("C200B40A82"), 0, None)
                .0
                .unwrap(),
            3
        );
        assert_eq!(
            parse_packet_b(hex_to_binary("04005AC33890"), 0, None)
                .0
                .unwrap(),
            54
        );
        assert_eq!(
            parse_packet_b(hex_to_binary("880086C3E88112"), 0, None)
                .0
                .unwrap(),
            7
        );
        assert_eq!(
            parse_packet_b(hex_to_binary("CE00C43D881120"), 0, None)
                .0
                .unwrap(),
            9
        );
        assert_eq!(
            parse_packet_b(hex_to_binary("D8005AC2A8F0"), 0, None)
                .0
                .unwrap(),
            1
        );
        assert_eq!(
            parse_packet_b(hex_to_binary("F600BC2D8F"), 0, None)
                .0
                .unwrap(),
            0
        );
        assert_eq!(
            parse_packet_b(hex_to_binary("9C005AC2F8F0"), 0, None)
                .0
                .unwrap(),
            0
        );
        assert_eq!(
            parse_packet_b(hex_to_binary("9C0141080250320F1802104A08"), 0, None)
                .0
                .unwrap(),
            1
        );
    }
}
