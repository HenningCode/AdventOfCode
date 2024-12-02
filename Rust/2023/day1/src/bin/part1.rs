use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Not a valid file");
    let result = parse_input(&input);
    println!("{result}");
}

fn parse_input(input: &str) -> i32 {
    let mut result = 0;

    for x in input.lines() {
        let mut int_vec: Vec<i32> = Vec::new();
        for char in x.chars() {
            if char.is_digit(10){
                int_vec.push(char.to_string().parse().unwrap());
            }
        }
        let mut together_int: i32 = 0;
        if let Some(result) = int_vec.first() {
            let value1 = result;
            let value2:i32;
            if let Some(result2) = int_vec.last() {
                value2 = *result2;
            } else {
                value2 =*value1;
            }
            together_int = value1 * 10 + value2;
        }
        result += together_int;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        assert_eq!(parse_input(input), 142);
    }
}
