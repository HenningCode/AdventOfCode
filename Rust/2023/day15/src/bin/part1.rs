use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let result = solve(&input);
    println!("{result}");
}

fn calc_hash(str: &str) -> u64 {
    let mut current_value = 0;
    for char in str.chars() {
        current_value += char as u64;
        current_value *= 17;
        current_value %= 256;
    }
    current_value
}

fn solve(input: &str) -> u64 {
    let mut result = 0;
    for split in input.split(',') {
        result += calc_hash(split);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

        assert_eq!(solve(input), 1320);
    }
}
