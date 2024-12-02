use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Not a valid file");
    let result = parse_input(&input);
    println!("{result}");
}

fn parse_input(input: &str) -> u32 {
    let (mut instructions, mut map) = ("", "");

    if let Some((temp1, temp2)) = input.split_once("\n\n") {
        (instructions, map) = (temp1, temp2);
    } else if let Some((temp1, temp2)) = input.split_once("\r\n\r\n") {
        (instructions, map) = (temp1, temp2);
    }

    let instructions_vec: Vec<char> = instructions.chars().collect();

    let mut hash_map = HashMap::new();

    for line in map.lines() {
        let (waypoint, decision) = line.split_once('=').unwrap();
        let (left, right) = decision.split_once(',').unwrap();
        let left = left.trim().replace(['(', ')'], "");
        let right = right.trim().replace(['(', ')'], "");

        hash_map.insert(waypoint.trim(), (left, right));
    }
    let len = instructions_vec.len();
    let mut i = 0;
    let start = "AAA";
    let (mut left, mut right) = hash_map.get(start).unwrap().clone();
    loop {
        let current_instruction = instructions_vec[i % len];

        match current_instruction {
            'L' => {
                if left == "ZZZ" {
                    break;
                }
                (left, right) = hash_map.get(left.as_str()).unwrap().clone();
            }
            'R' => {
                if right == "ZZZ" {
                    break;
                }
                (left, right) = hash_map.get(right.as_str()).unwrap().clone();
            }
            _ => panic!("Unknow instruction"),
        }
        i += 1;
    }

    (i as u32) + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_1() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

        assert_eq!(parse_input(input), 2);
    }

    #[test]
    fn test_input_2() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        assert_eq!(parse_input(input), 6);
    }
}
