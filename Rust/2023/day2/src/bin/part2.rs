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

fn get_values_from_game(str: &str) -> (u32, u32, u32) {
    let (mut red, mut green, mut blue) = (0, 0, 0);
    for cubes in str.split(',') {
        if let Some(num_cubes) = get_int_from_string(cubes) {
            if cubes.contains("blue") {
                blue = num_cubes;
            }
            if cubes.contains("red") {
                red = num_cubes;
            }
            if cubes.contains("green") {
                green = num_cubes;
            }
        }
    }
    (red, green, blue)
}

fn parse_input(str: &str) -> u32 {
    let mut result = 0;
    for line in str.lines() {
        let mut split = line.split(':');
        split.next();
        if let Some(split_games) = split.next() {
            let (mut red, mut green, mut blue) = (0, 0, 0);
            for game in split_games.split(';') {
                let (new_red, new_green, new_blue) = get_values_from_game(game);
                red = red.max(new_red);
                green = green.max(new_green);
                blue = blue.max(new_blue);
            }
            result += red * green * blue;
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
        assert_eq!(parse_input(input), 2286);
    }

    #[test]
    fn test_input_1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        assert_eq!(parse_input(input), 48);
    }

    #[test]
    fn test_input_2() {
        let input = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
        assert_eq!(parse_input(input), 12);
    }

    #[test]
    fn test_input_3() {
        let input = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        assert_eq!(parse_input(input), 1560);
    }

    #[test]
    fn test_input_4() {
        let input = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red";
        assert_eq!(parse_input(input), 630);
    }

    #[test]
    fn test_input_5() {
        let input = "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(parse_input(input), 36);
    }

    #[test]
    fn test_input_95() {
        let input = "Game 95: 3 blue, 13 red, 10 green; 4 green, 17 blue, 12 red; 12 red, 10 green, 16 blue; 15 red, 14 green, 2 blue; 12 red, 1 blue, 15 green; 10 green, 13 blue, 19 red";
        assert_eq!(parse_input(input), 4845);
        // red green blue
        // 19 15 17
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
}
