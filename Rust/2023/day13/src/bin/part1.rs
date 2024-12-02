use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let result = solve(&input);
    println!("{result}");
}

fn solve(input: &str) -> u64 {
    let mut data: Vec<Vec<String>> = Vec::new();
    let mut block: Vec<String> = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            data.push(block.clone());
            block.clear();
        } else {
            block.push(line.to_string());
        }
    }
    data.push(block);

    let mut result = 0;
    for block in data {
        let mirror_horizontal = find_horizontal_mirror_lines(&block);
        let mirror_vertical = find_vertical_mirror_lines(&block);

        if !mirror_horizontal.is_empty() {
            for mirror in mirror_horizontal {
                if check_horizontal_mirror(&block, mirror) {
                    result += mirror * 100;
                }
            }
        }
        if !mirror_vertical.is_empty() {
            for mirror in mirror_vertical {
                if check_vertical_mirror(&block, mirror) {
                    result += mirror;
                }
            }
        }
    }

    result as u64
}

fn find_horizontal_mirror_lines(input: &Vec<String>) -> Vec<usize> {
    let mut mirror_lines = Vec::new();
    for i in 1..input.len() {
        if input[i] == input[i - 1] {
            mirror_lines.push(i);
        }
    }

    mirror_lines
}

fn find_vertical_mirror_lines(input: &Vec<String>) -> Vec<usize> {
    let mut mirror_lines = Vec::new();
    for i in 1..input[0].len() {
        let mut found = true;
        for line in input {
            if line.chars().nth(i) != line.chars().nth(i - 1) {
                found = false;
                break;
            }
        }

        if found {
            mirror_lines.push(i);
        }
    }
    mirror_lines
}

fn check_horizontal_mirror(input: &Vec<String>, mirror_line: usize) -> bool {
    if input.len() - mirror_line > mirror_line {
        for (i, j) in (0..mirror_line - 1).rev().enumerate() {
            if input[j] != input[mirror_line + i + 1] {
                return false;
            }
        }
    } else {
        for (i, j) in (mirror_line + 1..input.len()).enumerate() {
            if input[j] != input[mirror_line - 2 - i] {
                return false;
            }
        }
    }

    true
}

fn check_vertical_mirror(input: &Vec<String>, mirror_line: usize) -> bool {
    if input[0].len() - mirror_line > mirror_line {
        for (i, j) in (0..mirror_line - 1).rev().enumerate() {
            for line in input {
                if line.chars().nth(j) != line.chars().nth((mirror_line) + i + 1) {
                    return false;
                }
            }
        }
        true
    } else {
        for (i, j) in (mirror_line + 1..input[0].len()).enumerate() {
            for line in input {
                if line.chars().nth(j) != line.chars().nth((mirror_line) - 2 - i) {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

        assert_eq!(solve(input), 405);
    }

    #[test]
    fn failed_test() {
        let input = "##.#.....
##.#.....
#......##
..#......
..##...##
.#.###.##
.##.##..#
###.#####
###.#####
.##.##..#
.#.#.#.##
..##...##";
        assert_eq!(solve(input), 7);
    }
}
