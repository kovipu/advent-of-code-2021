pub fn part_1(input: &str) -> i64 {
    let binary_data: String = hex_to_bin(input);
    let packet: Packet = read_packet(&binary_data);
    version_sum(packet)
}

pub fn part_2(input: &str) -> i64 {
    0
}

fn hex_to_bin(input: &str) -> String {
    let mut output: String = String::new();

    for c in input.chars() {
        match c {
            '0' => output.push_str("0000"),
            '1' => output.push_str("0001"),
            '2' => output.push_str("0010"),
            '3' => output.push_str("0011"),
            '4' => output.push_str("0100"),
            '5' => output.push_str("0101"),
            '6' => output.push_str("0110"),
            '7' => output.push_str("0111"),
            '8' => output.push_str("1000"),
            '9' => output.push_str("1001"),
            'A' => output.push_str("1010"),
            'B' => output.push_str("1011"),
            'C' => output.push_str("1100"),
            'D' => output.push_str("1101"),
            'E' => output.push_str("1110"),
            'F' => output.push_str("1111"),
            _ => panic!("Invalid hex character: {}", c),
        }
    }

    output
}

fn read_packet(binary: &str) -> Packet {
    let version: u8 = u8::from_str_radix(&binary[0..3], 2).unwrap();
    let packet_type: u8 = u8::from_str_radix(&binary[3..6], 2).unwrap();

    let header_length: usize = 6; // header is always 6 bits

    let payload: &str = &binary[6..];

    match packet_type {
        4 => {
            // Literal data packet

            // handle data as 5 long chunks
            let mut binary_data: String = String::new();

            let mut chunk_start = 0;
            let mut length = header_length;

            loop {
                let chunk = &payload[chunk_start..chunk_start + 5];
                let (is_last, chunk_data) = chunk.split_at(1);

                binary_data.push_str(chunk_data);
                length += 5;

                if is_last == "0" {
                    break;
                }
                chunk_start += 5;
            }

            let literal_data = i32::from_str_radix(&binary_data, 2).unwrap();

            Packet {
                version,
                length,
                data: PacketData::Literal(literal_data),
            }
        }
        _ => {
            // Operator data packet

            let (length_type_id, rest) = payload.split_at(1);

            let (data_length, packets) = if length_type_id == "1" {
                rest.split_at(11)
            } else {
                rest.split_at(15)
            };

            let data_length: usize = usize::from_str_radix(data_length, 2).unwrap();

            let mut sub_packet_start: usize = 0;
            let mut sub_packet_count: usize = 0;
            let mut sub_packets: Vec<Packet> = vec![];

            while (length_type_id == "1" && sub_packet_count < data_length)
                || (length_type_id == "0" && sub_packet_start < data_length)
            {
                let sub_packet = read_packet(&packets[sub_packet_start..]);
                sub_packets.push(sub_packet.clone());

                sub_packet_start += sub_packet.length;
                sub_packet_count += 1;
            }

            let length = if length_type_id == "1" {
                header_length + 12 + sub_packet_start
            } else {
                header_length + 16 + data_length
            };

            let operator_type = match packet_type {
                0 => OperatorType::Sum,
                1 => OperatorType::Product,
                2 => OperatorType::Minimum,
                3 => OperatorType::Maximum,
                5 => OperatorType::GreaterThan,
                6 => OperatorType::LessThan,
                7 => OperatorType::EqualTo,
                _ => panic!("Invalid operator type: {}", packet_type),
            };

            Packet {
                version,
                length,
                data: PacketData::Operator(OperatorData {
                    operator_type,
                    sub_packets,
                }),
            }
        }
    }
}

fn version_sum(packet: Packet) -> i64 {
    match packet.data {
        PacketData::Literal(_) => packet.version as i64,
        PacketData::Operator(OperatorData {
            sub_packets,
            operator_type: _,
        }) => {
            let mut sum: i64 = packet.version as i64;

            for sub_packet in sub_packets {
                sum += version_sum(sub_packet);
            }

            sum
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Packet {
    version: u8,
    length: usize,
    data: PacketData,
}

#[derive(Debug, PartialEq, Clone)]
enum PacketData {
    Literal(i32),
    Operator(OperatorData),
}

#[derive(Debug, PartialEq, Clone)]
struct OperatorData {
    operator_type: OperatorType,
    sub_packets: Vec<Packet>,
}

#[derive(Debug, PartialEq, Clone)]
enum OperatorType {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_bin() {
        assert_eq!(hex_to_bin(INPUT_HEX_LITERAL), INPUT_BIN_LITERAL);
    }

    #[test]
    fn test_read_literal_packet() {
        assert_eq!(
            read_packet(INPUT_BIN_LITERAL),
            Packet {
                version: 6,
                length: 21,
                data: PacketData::Literal(2021),
            }
        );
    }

    #[test]
    fn test_read_operator_packet() {
        assert_eq!(
            read_packet(INPUT_BIN_OPERATOR),
            Packet {
                version: 1,
                length: 49,
                data: PacketData::Operator(OperatorData {
                    operator_type: OperatorType::LessThan,
                    sub_packets: vec![
                        Packet {
                            version: 6,
                            length: 11,
                            data: PacketData::Literal(10),
                        },
                        Packet {
                            version: 2,
                            length: 16,
                            data: PacketData::Literal(20),
                        },
                    ]
                }),
            }
        );
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 31);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 0);
    }
}

#[cfg(test)]
const INPUT_HEX_LITERAL: &str = "D2FE28";

#[cfg(test)]
const INPUT_BIN_LITERAL: &str = "110100101111111000101000";

#[cfg(test)]
const INPUT_BIN_OPERATOR: &str = "00111000000000000110111101000101001010010001001000000000";

#[cfg(test)]
const INPUT: &str = "A0016C880162017C3686B18A3D4780";
