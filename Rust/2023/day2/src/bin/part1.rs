use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Not a valid file");
    let result = parse_input(&input);
    println!("{result}");
}

fn get_int_from_string(str: &str) -> Option<u32> {
    let mut number_string: String = String::new();
    for char in str.chars() {
        if char.is_ascii_digit() {
            number_string.push(char);
        }
    }
    number_string.parse().ok()
}

fn check_if_valid_game(str: &str, red: u32, green: u32, blue: u32) -> bool {
    for cubes in str.split(',') {
        if let Some(num_cubes) = get_int_from_string(cubes) {
            if cubes.contains("blue") && num_cubes > blue {
                    return false;
            }
            if cubes.contains("red") && num_cubes > red {
                    return false;
            }
            if cubes.contains("green") && num_cubes > green {
                    return false;
            }
        }
    }
    true
}

fn parse_input(str: &str) -> u32 {
    let mut result = 0;
    for line in str.lines() {
        let mut split = line.split(':');
        let mut game_value: u32 = 0;
        if let Some(game) = split.next() {
            if let Some(value) = get_int_from_string(game) {
                game_value = value;
            }
        }
        let mut valid = true;
        if let Some(split_games) = split.next() {
            for games in split_games.split(';') {
                if !check_if_valid_game(games, 12, 13, 14) {
                    valid = false;
                }
            }
        }
        if valid {
            result += game_value;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green\nGame 95: 3 blue, 13 red, 10 green; 4 green, 17 blue, 12 red; 12 red, 10 green, 16 blue; 15 red, 14 green, 2 blue; 12 red, 1 blue, 15 green; 10 green, 13 blue, 19 red";
        assert_eq!(parse_input(input), 8);
    }

    #[test]
    fn test1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        assert_eq!(parse_input(input), 1);
    }

    #[test]
    fn test2() {
        let input = "Game 74: 19 green, 9 blue, 14 red; 11 green, 8 blue, 14 red; 2 green, 17 blue, 14 red; 12 green, 12 blue, 7 red; 6 red, 5 blue, 10 green; 4 blue, 19 green, 15 red";
        assert_eq!(parse_input(input), 0);
    }

    #[test]
    fn test3() {
        let input = "20 red";
        assert_eq!(get_int_from_string(input), Some(20));
    }

    #[test]
    fn test4() {
        let input = "20 red";
        assert_eq!(get_int_from_string(input), Some(20));
    }

    #[test]
    fn test5() {
        let input = "19 green";
        assert_eq!(get_int_from_string(input), Some(19));
    }

    #[test]
    fn test6() {
        let input = "Test";
        assert_eq!(get_int_from_string(input), None);
    }
}
