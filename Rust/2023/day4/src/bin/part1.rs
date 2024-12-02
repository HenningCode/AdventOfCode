use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Not a valid file");
    let result = parse_input(&input);
    println!("{result}");
}

fn get_numbers_into_vec(input: &str) -> Vec<u32> {
    let mut number_vec: Vec<u32> = Vec::new();

    for number in input.trim().split(' ') {
        if let Ok(number) = number.parse() {
            number_vec.push(number)
        };
    }
    number_vec
}

fn parse_input(input: &str) -> u32 {
    let mut result = 0;
    for line in input.lines() {
        let mut right: u32 = 0;
        let mut split = line.split(':');
        split.next();
        if let Some(card) = split.next() {
            let mut game_split = card.split('|');
            if let Some(winning_num) = game_split.next() {
                if let Some(game_num) = game_split.next() {
                    let winning_num = get_numbers_into_vec(winning_num);
                    let game_num = get_numbers_into_vec(game_num);
                    for number in game_num {
                        if winning_num.contains(&number) {
                            right += 1;
                        }
                    }
                }
            }
        }

        let base: u32 = 2;
        if right > 0 {
            result += base.pow(right.saturating_sub(1));
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(parse_input(input), 13);
    }
}
