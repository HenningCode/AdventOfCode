use regex::{Match, Regex};
use std::fs;
use std::num::ParseIntError;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Not a valid file");
    let result = parse_input(input);
    println!("{result}");
}

fn try_parse_to_int(str: String) -> Result<i32, ParseIntError> {
    let parse = str.parse()?;
    Ok(parse)
}

fn match_string_to_number(str: &str) -> i32 {
    match str {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => panic!("Match number did not work!"),
    }
}

fn regex_find_all_numbers(str: &str) -> (Vec<i32>, Vec<Match<'_>>) {
    let re = Regex::new(r"one|two|three|four|five|six|seven|eight|nine").unwrap();
    let mut offset: Vec<i32> = Vec::new();
    let mut temp: usize = 0;
    let mut slice = &str[0..str.len()];
    let mut result: Vec<Match<'_>> = Vec::new();
    loop {
        if let Some(find) = re.find(slice) {
            result.push(find);
            offset.push(temp as i32);
            temp = find.start() + 1;
            slice = &slice[temp..slice.len()];
        } else {
            return (offset, result);
        }
    }
}

fn parse_input(input: String) -> i32 {
    let lines: Vec<String> = input.lines().map(|x| String::from(x)).collect();
    let mut result = 0;
    for line in lines {
        let (offset, find) = regex_find_all_numbers(line.as_str());

        let char_vec: Vec<char> = line.chars().collect();
        let mut int_vec: Vec<i32> = Vec::new();
        let mut position_int_vec: Vec<i32> = Vec::new();
        for (i, char) in char_vec.iter().enumerate() {
            let string_char = char.to_string();

            if let Ok(try_parse) = try_parse_to_int(string_char) {
                int_vec.push(try_parse);
                position_int_vec.push(i as i32);
            }
        }

        let mut value1: i32 = 0;
        let mut value2: i32 = 0;

        if let Some(result) = find.first() {
            let index = result.start() as i32;
            if let Some(position_int) = position_int_vec.first() {
                if *position_int > index {
                    value1 = match_string_to_number(result.as_str());
                } else {
                    if let Some(result) = int_vec.first() {
                        value1 = *result;
                    }
                }
            } else {
                value1 = match_string_to_number(find[0].as_str());
                if let Some(result) = find.last() {
                    value2 = match_string_to_number(result.as_str());
                }
            }
            if let Some(result) = find.last() {
                let index = offset.iter().sum::<i32>() + result.start() as i32;
                if let Some(position_int) = position_int_vec.last() {
                    if *position_int < index {
                        value2 = match_string_to_number(result.as_str())
                    } else {
                        if let Some(result) = int_vec.last() {
                            value2 = *result;
                        }
                    }
                }
            }
        } else {
            if let Some(result) = int_vec.first() {
                value1 = *result;
                if let Some(result) = int_vec.last() {
                    value2 = *result;
                }
            }
        }

        let value1 = value1.to_string().to_owned();
        let value2 = value2.to_string().to_owned();
        let together: i32 = format!("{value1}{value2}").parse().unwrap();

        result += together;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let input= "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen".to_string();
        assert_eq!(parse_input(input), 281);
    }

    #[test]
    fn test1() {
        let input = "two1nine".to_string();
        assert_eq!(parse_input(input), 29);
    }

    #[test]
    fn test2() {
        let input = "eightwothree".to_string();
        assert_eq!(parse_input(input), 83);
    }
    #[test]
    fn test3() {
        let input = "abcone2threexyz".to_string();
        assert_eq!(parse_input(input), 13);
    }
    #[test]
    fn test4() {
        let input = "xtwone3four".to_string();
        assert_eq!(parse_input(input), 24);
    }
    #[test]
    fn test5() {
        let input = "4nineeightseven2".to_string();
        assert_eq!(parse_input(input), 42);
    }
    #[test]
    fn test6() {
        let input = "zoneight234".to_string();
        assert_eq!(parse_input(input), 14);
    }
    #[test]
    fn test7() {
        let input = "7pqrstsixteen".to_string();
        assert_eq!(parse_input(input), 76);
    }

    #[test]
    fn test8() {
        let input = "5onetwo".to_string();
        assert_eq!(parse_input(input), 52);
    }

    #[test]
    fn test9() {
        let input = "".to_string();
        assert_eq!(parse_input(input), 0);
    }

    #[test]
    fn test10() {
        let input = "asdfoneight".to_string();
        assert_eq!(parse_input(input), 18);
    }
}
