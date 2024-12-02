use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Not a valid file");
    let result = parse_input(&input);
    println!("{result}");
}

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}


fn parse_input(input: &str) -> usize {
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
    let mut start_vec = Vec::new();

    for key in hash_map.clone() {
        if key.0.ends_with('A') {
            start_vec.push(key.0)
        }
    }

    let mut iterations_vec = Vec::new();

    for start in start_vec {
        let mut i = 0;
        let (mut left, mut right) = hash_map.get(start).unwrap().clone();
        loop {
            let current_instruction = instructions_vec[i % len];
    
            match current_instruction {
                'L' => {
                    if left.ends_with('Z') {
                        break;
                    }
                    (left, right) = hash_map.get(left.as_str()).unwrap().clone();
                }
                'R' => {
                    if right.ends_with('Z') {
                        break;
                    }
                    (left, right) = hash_map.get(right.as_str()).unwrap().clone();
                }
                _ => panic!("Unknow instruction"),
            }
            i += 1;
        }
        
        iterations_vec.push(i+1)
    }

    lcm(&iterations_vec)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_1() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        assert_eq!(parse_input(input), 6);
    }
}
