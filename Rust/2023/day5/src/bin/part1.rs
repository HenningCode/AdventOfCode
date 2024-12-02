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

fn parse_input(input: &str) -> i64 {
    let lines: Vec<String> = input.lines().map(String::from).collect();
    let mut seeds_split = lines[0].split(":");
    seeds_split.next();
    let mut seeds = get_number_from_line(seeds_split.next().unwrap());
    let mut allready_changed = vec![false; seeds.len()];

    for line in lines[1..].into_iter() {
        if !line.is_empty() {
            let current_map = get_number_from_line(&line);
            if current_map.is_empty() {
                allready_changed = vec![false; seeds.len()];
                continue;
            }

            for i in 0..seeds.len() {
                let range = current_map[1]..(current_map[1] + current_map[2]);
                if range.contains(&seeds[i]) && !allready_changed[i] {
                    let offset = current_map[0] - current_map[1];
                    seeds[i] = seeds[i] + offset;
                    allready_changed[i] = true;
                }
            }
        }
    }

    *seeds.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let result = parse_input(input);
        assert_eq!(result, 35);
    }
}
