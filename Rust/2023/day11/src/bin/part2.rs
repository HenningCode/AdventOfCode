use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Not a valid filename");
    let result = solve_part2(&input, 1_000_000);
    println!("{result}");
}

fn manhattan_distance(x1: i64, x2: i64, y1: i64, y2: i64) -> i64 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

#[derive(Debug)]
struct Universe {
    map: Vec<Vec<char>>,
    galaxies: Vec<Galaxy>,
    universe_width: usize,
    universe_length: usize,
}

impl Universe {
    fn new(input: &str) -> Self {
        let map: Vec<Vec<char>> = input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect();

        let galaxies = map
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .filter(|(_, c)| **c == '#')
                    .map(move |(x, _)| Galaxy::new(x, y))
            })
            .collect::<Vec<_>>();

        let universe_length = map.len();
        let universe_width = map[0].len();

        Self {
            map,
            galaxies,
            universe_width,
            universe_length,
        }
    }

    fn expand_universe(&mut self, mut expansion: u32) {
        let empty_rows = self
            .map
            .iter()
            .enumerate()
            .filter_map(|(row, line)| {
                if line.iter().any(|char| char == &'#') {
                    None
                } else {
                    Some(row)
                }
            })
            .collect::<Vec<usize>>();

        let mut empty_cols: Vec<usize> = Vec::new();

        for x in 0..self.universe_width {
            let mut empty = true;
            for y in 0..self.universe_length {
                if self.map[y][x] == '#' {
                    empty = false;
                    break;
                }
            }
            if empty {
                empty_cols.push(x);
            }
        }

        expansion -= 1;

        for galaxy in self.galaxies.iter_mut() {
            let mut num_smaller = 0;
            for i in &empty_rows {
                if *i > galaxy.y {
                    break;
                }
                num_smaller += 1;
            }
            galaxy.y += &((expansion * num_smaller) as usize);
            num_smaller = 0;
            for i in &empty_cols {
                if *i > galaxy.x {
                    break;
                }
                num_smaller += 1;
            }
            galaxy.x += &((expansion * num_smaller) as usize);
        }
    }
}

#[derive(Debug, Clone)]
struct Galaxy {
    x: usize,
    y: usize,
}

impl Galaxy {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

fn solve_part2(input: &str, expansion: u32) -> i64 {
    let mut universe = Universe::new(input);
    universe.expand_universe(expansion);

    let mut result = 0;

    for i in 0..universe.galaxies.len() {
        for j in i + 1..universe.galaxies.len() {
            let temp = manhattan_distance(
                universe.galaxies[i].x as i64,
                universe.galaxies[j].x as i64,
                universe.galaxies[i].y as i64,
                universe.galaxies[j].y as i64,
            );

            result += temp;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        assert_eq!(solve_part2(input, 10), 1030);

        assert_eq!(solve_part2(input, 100), 8410);
    }
}
