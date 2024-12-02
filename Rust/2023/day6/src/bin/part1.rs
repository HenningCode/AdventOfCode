
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
    let mut right = vec![0; time.len()];

    for i in 0..time.len() {
        for j in 0..time[i] {
            if distance[i] < -j.pow(2) + time[i]*j {
                right[i] += 1;
            }
        }
    }
    right.iter().product()
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_input(){
        let input = "Time:      7  15   30
Distance:  9  40  200";

        assert_eq!(parse_input(input), 288)
    }
}