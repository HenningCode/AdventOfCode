fn main() {}

#[derive(Debug, Clone)]
struct Field {
    data: String,
    positions: Vec<i32>,
}

impl Field {
    fn new(data: String, positions: Vec<i32>) -> Self {
        Self { data, positions }
    }

    fn calc_combinations(&self) -> i32 {
        let predefined_blocks: Vec<String> = self
            .data
            .split('.')
            .flat_map(|x| {
                let temp = x.to_string();
                if temp != "" {
                    Some(temp)
                } else {
                    None
                }
            })
            .collect();

        let mut result = 0;

        if predefined_blocks.len() == self.positions.len() {
            for (i, block) in predefined_blocks.iter().enumerate() {
                if block.contains("?") {
                    if block.len() as i32 == self.positions[i] {
                        break;
                    } else {
                        let temp = (block.len() + 1) as i32
                            - Field::count_hashtags(block)
                            - self.positions[i];
                        result += temp;
                    }
                }
            }
        } else if predefined_blocks.len() < self.positions.len() {
            let predefinded_block_length:Vec<i32> = predefined_blocks.iter().map(|x| {
                x.len() as i32
            }).collect();



            println!("{:?}", predefinded_block_length);

        } else if predefined_blocks.len() > self.positions.len() {
            panic!("Not solvable!");
        }
        println!("{:?}", self.positions);
        println!("{:?}", predefined_blocks);
        println!("Calc: {result}");

        444
    }

    fn count_hashtags(input: &String) -> i32 {
        let mut result = 0;
        for char in input.chars() {
            if char == '#' {
                result += 1;
            }
        }
        result
    }

    fn get_countinues_hastags(input: &String) -> Vec<i32> {
        let continues = false;
        input.chars().map(|x| {
            
        })
    }
}

fn solve_part_1(input: &str) -> i32 {
    let field_vector: Vec<Field> = input
        .lines()
        .map(|line| {
            let (data, positions) = line.split_once(' ').unwrap();
            let positions = positions
                .split(',')
                .flat_map(|x| x.parse().ok())
                .collect::<Vec<i32>>();
            Field::new(data.to_string(), positions)
        })
        .collect();

    let mut result = 0;
    for field in field_vector.clone() {
        result += field.calc_combinations();
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_1() {
        let input = "???.### 1,1,3";
        assert_eq!(solve_part_1(input), 1);
    }

    #[test]
    fn test_input_2() {
        let input = ".??..??...?##. 1,1,3";
        assert_eq!(solve_part_1(input), 4);
    }

    #[test]
    fn test_input_3() {
        let input = "?#?#?#?#?#?#?#? 1,3,1,6";
        assert_eq!(solve_part_1(input), 1);
    }

    #[test]
    fn test_input_4() {
        let input = "????.#...#... 4,1,1";
        assert_eq!(solve_part_1(input), 1);
    }

    #[test]
    fn test_input_5() {
        let input = "????.######..#####. 1,6,5";
        assert_eq!(solve_part_1(input), 4);
    }

    #[test]
    fn test_input_6() {
        let input = "?###???????? 3,2,1";
        assert_eq!(solve_part_1(input), 10);
    }

    #[test]
    fn test_input_7() {
        let input = "??#?.### 2,3";
        assert_eq!(solve_part_1(input), 2);
    }
}
