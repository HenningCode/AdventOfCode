use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Not a valid filename");
    let result = solve_part1(&input);
    println!("{result}");
}

fn manhattan_distance(x1: i32, x2: i32, y1: i32, y2: i32) -> i32 {
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

        let galaxies = Vec::new();

        let universe_length = map.len();
        let universe_width = map[0].len();

        Self {
            map,
            galaxies,
            universe_width,
            universe_length,
        }
    }

    fn expand_universe(&mut self) {
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


        for (i, x) in empty_rows.clone().iter().enumerate() {
            self.map.insert(x + i, vec!['.'; self.universe_width]);
        }

        self.universe_length += empty_rows.len();

        for (i, y) in empty_cols.clone().iter().enumerate() {
            for j in 0..self.universe_length {
                self.map[j].insert(y + i, '.');
                
            }
        }
        self.universe_width += empty_cols.len();
    }

    fn get_galaxies(&mut self) {
        self.galaxies = self
            .map
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .filter(|(_, c)| **c == '#')
                    .map(move |(x, _)| Galaxy::new(x, y))
            })
            .collect::<Vec<_>>()
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

fn solve_part1(input: &str) -> i32 {
    let mut universe = Universe::new(input);
    universe.expand_universe();

    let mut result = 0;

    for i in &universe.map {
        println!("{:?}", i);
    }

    universe.get_galaxies();

    for i in 0..universe.galaxies.len() {
        for j in i + 1..universe.galaxies.len() {
            let temp = manhattan_distance(
                universe.galaxies[i].x as i32,
                universe.galaxies[j].x as i32,
                universe.galaxies[i].y as i32,
                universe.galaxies[j].y as i32,
            );
            // println!(
            //     "Pair:{i}{j}, G:{:?},{:?} R:{temp}",
            //     universe.galaxies[i], universe.galaxies[j]
            // );
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

        assert_eq!(solve_part1(input), 374);
    }
}
