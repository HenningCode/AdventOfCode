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
    let allowed_smudges = 1;
    let mut result = 0;

    for block in data {
        // Horizontal Lines
        for line_index in 1..block.len() {
            let mut smudges = 0;
            // println!("{line_index}");
            for it in 0..line_index {
                let bottom_line = line_index - 1 - it;
                let top_line = line_index + it;

                if top_line < block.len() {
                    for char_index in 0..block[line_index].len() {
                        if block[top_line].chars().nth(char_index)
                            != block[bottom_line].chars().nth(char_index)
                        {
                            smudges += 1;
                        }
                    }
                }
            }
            if smudges == allowed_smudges {
                result += 100 * line_index;
            }
        }

        // Vertical Lines

        for line_index in 1..block[0].len() {
            let mut smudges = 0;
            // println!("{line_index}");
            for it in 0..line_index {
                let left_line = line_index - 1 - it;
                let right_line = line_index + it;

                if right_line < block[0].len() {
                    for char in block.clone() {
                        if char.chars().nth(right_line) != char.chars().nth(left_line) {
                            smudges += 1;
                        }
                    }
                }
            }
            if smudges == allowed_smudges {
                result += line_index;
            }
        }
    }

    result as u64
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

        assert_eq!(solve(input), 400);
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
        assert_eq!(solve(input), 1);
    }

    #[test]
    fn failed_test_2() {
        let input = "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        assert_eq!(solve(input), 1);
    }
}
