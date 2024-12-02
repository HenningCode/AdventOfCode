
use std::fs;
fn main() {
    let input = fs::read_to_string("input.txt").expect("Not a valid file");
    let result = parse_input(&input);
    println!("{result}");
}

fn get_number_from_line(input: &str) -> Vec<i64> {
    let mut number_vec = Vec::new();

    for number in input.trim().split(' ') {
        if let Ok(number) = number.parse() {
            number_vec.push(number)
        };
    }
    number_vec
}

fn parse_input(input: &str) -> u32{
    let (time, distance) = input.split_once("\n").unwrap();
    let time = get_number_from_line(time);
    let distance = get_number_from_line(distance);

    let time:i64 = time.iter().map(|x| x.to_string()).collect::<String>().parse().unwrap();
    let distance:i64 = distance.iter().map(|x| x.to_string()).collect::<String>().parse().unwrap();
    let mut right = 0;

    for j in 0..time {
        if distance < -j.pow(2) + time*j {
            right += 1;
        }
    }

    right
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_input(){
        let input = "Time:      7  15   30
Distance:  9  40  200";

        assert_eq!(parse_input(input), 71503)
    }
}